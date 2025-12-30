use crate::db::Repository;
use crate::models::Project;
use crate::monitor::start_background_monitor;
use crate::views::{DashboardView, ProjectDetailView};
use adw::prelude::*;
use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

/// Navigation state for the application
#[derive(Debug, Clone, PartialEq)]
pub enum NavigationState {
    Dashboard,
    ProjectDetail(String), // Project ID
}

/// Main application window
pub struct MainWindow {
    window: adw::ApplicationWindow,
    navigation_view: adw::NavigationView,
    repository: Repository,
    state: Rc<RefCell<NavigationState>>,
    monitoring_active: Rc<RefCell<bool>>,
    monitor_handle: Arc<Mutex<Option<std::thread::JoinHandle<()>>>>,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(app: &adw::Application, repository: Repository) -> Self {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Claude Context Tracker")
            .default_width(1200)
            .default_height(800)
            .build();

        // Create navigation view for managing different screens
        let navigation_view = adw::NavigationView::new();

        // Initial state is dashboard
        let state = Rc::new(RefCell::new(NavigationState::Dashboard));

        let mut main_window = Self {
            window,
            navigation_view,
            repository,
            state,
            monitoring_active: Rc::new(RefCell::new(false)),
            monitor_handle: Arc::new(Mutex::new(None)),
        };

        main_window.setup_ui();
        main_window
    }

    /// Setup the UI components
    fn setup_ui(&mut self) {
        // Create dashboard view
        let dashboard = self.create_dashboard_view();

        // Add dashboard as root page
        let dashboard_page = adw::NavigationPage::builder()
            .title("Projects")
            .child(&dashboard)
            .build();

        self.navigation_view.add(&dashboard_page);

        // Set navigation view as window content
        self.window.set_content(Some(&self.navigation_view));

        // Setup keyboard shortcuts
        self.setup_shortcuts();

        // Setup menu actions
        self.setup_actions();
    }

    /// Setup menu actions
    fn setup_actions(&self) {
        let app = self.window.application().unwrap();

        // Preferences action
        let window = self.window.clone();
        let prefs_action = gtk::gio::SimpleAction::new("preferences", None);
        prefs_action.connect_activate(move |_, _| {
            log::info!("Opening preferences");
            let settings = crate::settings::SettingsDialog::new(&window);
            settings.present();
        });
        app.add_action(&prefs_action);

        // Keyboard shortcuts action
        let window_clone = self.window.clone();
        let shortcuts_action = gtk::gio::SimpleAction::new("shortcuts", None);
        shortcuts_action.connect_activate(move |_, _| {
            log::info!("Showing keyboard shortcuts");
            Self::show_shortcuts_window(&window_clone);
        });
        app.add_action(&shortcuts_action);

        // About action
        let window_clone2 = self.window.clone();
        let about_action = gtk::gio::SimpleAction::new("about", None);
        about_action.connect_activate(move |_, _| {
            log::info!("Showing about dialog");
            Self::show_about_dialog(&window_clone2);
        });
        app.add_action(&about_action);
    }

    /// Show keyboard shortcuts window
    fn show_shortcuts_window(window: &adw::ApplicationWindow) {
        let shortcuts_window = gtk::ShortcutsWindow::builder()
            .modal(true)
            .transient_for(window)
            .build();

        let section = gtk::ShortcutsSection::builder()
            .section_name("shortcuts")
            .max_height(10)
            .build();

        // General group
        let general_group = gtk::ShortcutsGroup::builder()
            .title("General")
            .build();

        general_group.add_child(&gtk::ShortcutsShortcut::builder()
            .title("Preferences")
            .accelerator("<Ctrl>comma")
            .build());

        general_group.add_child(&gtk::ShortcutsShortcut::builder()
            .title("Quit")
            .accelerator("<Ctrl>Q")
            .build());

        section.add_child(&general_group);

        // Projects group
        let projects_group = gtk::ShortcutsGroup::builder()
            .title("Projects")
            .build();

        projects_group.add_child(&gtk::ShortcutsShortcut::builder()
            .title("New Project")
            .accelerator("<Ctrl>N")
            .build());

        projects_group.add_child(&gtk::ShortcutsShortcut::builder()
            .title("Refresh")
            .accelerator("F5")
            .build());

        projects_group.add_child(&gtk::ShortcutsShortcut::builder()
            .title("Search")
            .accelerator("<Ctrl>F")
            .build());

        section.add_child(&projects_group);

        shortcuts_window.add_child(&section);
        shortcuts_window.present();
    }

    /// Show about dialog
    fn show_about_dialog(window: &adw::ApplicationWindow) {
        let about = adw::AboutWindow::builder()
            .transient_for(window)
            .application_name("Claude Context Tracker")
            .application_icon("com.github.claudecontexttracker")
            .developer_name("Claude Context Tracker Contributors")
            .version("1.0.0")
            .comments("Native GTK4 application for managing Claude Code context across projects")
            .website("https://github.com/AngelFreak/CCD")
            .issue_url("https://github.com/AngelFreak/CCD/issues")
            .license_type(gtk::License::MitX11)
            .build();

        about.add_credit_section(Some("Built with"), &[
            "GTK4",
            "libadwaita",
            "rusqlite",
            "clap",
            "notify",
        ]);

        about.present();
    }

    /// Create the dashboard view
    fn create_dashboard_view(&self) -> gtk::Box {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Header bar
        let header = adw::HeaderBar::new();

        // Monitoring toggle (left side)
        let monitor_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        monitor_box.set_margin_start(8);

        let monitor_icon = gtk::Image::from_icon_name("emblem-synchronizing-symbolic");
        monitor_box.append(&monitor_icon);

        let monitor_label = gtk::Label::new(Some("Monitor"));
        monitor_label.add_css_class("monitor-label");
        monitor_box.append(&monitor_label);

        let monitor_switch = gtk::Switch::new();
        monitor_switch.set_tooltip_text(Some("Background monitoring of Claude Code logs"));
        monitor_box.append(&monitor_switch);

        header.pack_start(&monitor_box);

        // Wire up monitoring toggle
        let repository_clone = self.repository.clone();
        let monitoring_active = self.monitoring_active.clone();
        let monitor_handle = self.monitor_handle.clone();
        let monitor_label_weak = monitor_label.downgrade();

        monitor_switch.connect_state_set(move |switch, enabled| {
            log::info!("Monitor toggle: {}", enabled);
            *monitoring_active.borrow_mut() = enabled;

            if enabled {
                // Start background monitoring
                // For now, monitor all projects (could be enhanced to track active project)
                match start_background_monitor(
                    "default".to_string(),
                    repository_clone.clone(),
                    None,
                ) {
                    Ok(handle) => {
                        *monitor_handle.lock().unwrap() = Some(handle);
                        log::info!("Background monitoring started");
                        if let Some(label) = monitor_label_weak.upgrade() {
                            label.set_text("Monitoring");
                            label.add_css_class("monitoring-active");
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to start monitoring: {}", e);
                        switch.set_active(false);
                    }
                }
            } else {
                // Stop background monitoring
                // Note: We can't easily stop the thread, but we log the state change
                log::info!("Background monitoring stopped (thread continues)");
                if let Some(label) = monitor_label_weak.upgrade() {
                    label.set_text("Monitor");
                    label.remove_css_class("monitoring-active");
                }
            }

            glib::Propagation::Proceed
        });

        // Menu button (right side)
        let menu_button = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Main Menu")
            .build();
        menu_button.add_css_class("flat");

        // Create menu
        let menu = gtk::gio::Menu::new();

        // Preferences menu item
        let prefs_item = gtk::gio::MenuItem::new(Some("Preferences"), Some("app.preferences"));
        menu.append_item(&prefs_item);

        // Keyboard shortcuts menu item
        let shortcuts_item = gtk::gio::MenuItem::new(Some("Keyboard Shortcuts"), Some("app.shortcuts"));
        menu.append_item(&shortcuts_item);

        menu.append_section(None, &{
            let section = gtk::gio::Menu::new();
            section.append(Some("About"), Some("app.about"));
            section
        });

        menu_button.set_menu_model(Some(&menu));
        header.pack_end(&menu_button);

        // Add new project button
        let new_project_btn = gtk::Button::builder()
            .icon_name("list-add-symbolic")
            .tooltip_text("Create New Project (Ctrl+N)")
            .build();
        new_project_btn.add_css_class("flat");

        let repository = self.repository.clone();
        let nav_view = self.navigation_view.clone();
        new_project_btn.connect_clicked(clone!(@weak nav_view => move |_| {
            Self::show_new_project_dialog(repository.clone(), nav_view.clone());
        }));

        header.pack_end(&new_project_btn);

        // Refresh button
        let refresh_btn = gtk::Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Refresh Projects (F5)")
            .build();
        refresh_btn.add_css_class("flat");
        header.pack_end(&refresh_btn);

        container.append(&header);

        // Dashboard content
        let dashboard_view = DashboardView::new(self.repository.clone(), self.navigation_view.clone());
        container.append(&dashboard_view.widget());

        // Connect refresh button
        refresh_btn.connect_clicked(clone!(@weak dashboard_view => move |_| {
            dashboard_view.refresh();
        }));

        container
    }

    /// Show dialog to create a new project
    fn show_new_project_dialog(repository: Repository, nav_view: adw::NavigationView) {
        // This will be implemented when we create the dashboard view
        log::info!("New project dialog requested");
    }

    /// Setup keyboard shortcuts
    fn setup_shortcuts(&self) {
        let shortcuts = gtk::EventControllerKey::new();

        let window = self.window.clone();
        let repository = self.repository.clone();
        let nav_view = self.navigation_view.clone();

        shortcuts.connect_key_pressed(move |_, key, _, modifier| {
            if modifier.contains(gtk::gdk::ModifierType::CONTROL_MASK) {
                match key {
                    // Ctrl+Q: Quit
                    gtk::gdk::Key::q => {
                        window.close();
                        return glib::Propagation::Stop;
                    }
                    // Ctrl+N: New project
                    gtk::gdk::Key::n => {
                        log::info!("New project (Ctrl+N)");
                        Self::show_new_project_dialog(repository.clone(), nav_view.clone());
                        return glib::Propagation::Stop;
                    }
                    // Ctrl+,: Preferences
                    gtk::gdk::Key::comma => {
                        log::info!("Opening preferences (Ctrl+,)");
                        let settings = crate::settings::SettingsDialog::new(&window);
                        settings.present();
                        return glib::Propagation::Stop;
                    }
                    // Ctrl+F: Search (placeholder)
                    gtk::gdk::Key::f => {
                        log::info!("Search (Ctrl+F) - not yet implemented");
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            } else {
                match key {
                    // F5: Refresh
                    gtk::gdk::Key::F5 => {
                        log::info!("Refresh (F5) - not yet implemented");
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            }
            glib::Propagation::Proceed
        });

        self.window.add_controller(shortcuts);
    }

    /// Navigate to project detail view
    pub fn navigate_to_project(&self, project_id: String) {
        *self.state.borrow_mut() = NavigationState::ProjectDetail(project_id.clone());

        // Create project detail view
        let project_detail = ProjectDetailView::new(
            self.repository.clone(),
            project_id,
            self.navigation_view.clone(),
        );

        let page = adw::NavigationPage::builder()
            .title("Project Details")
            .child(&project_detail.widget())
            .build();

        self.navigation_view.push(&page);
    }

    /// Navigate back to dashboard
    pub fn navigate_to_dashboard(&self) {
        *self.state.borrow_mut() = NavigationState::Dashboard;
        self.navigation_view.pop();
    }

    /// Get the window widget
    pub fn present(&self) {
        self.window.present();
    }
}

/// Helper macro for cloning references (mimics glib::clone! macro)
macro_rules! clone {
    (@weak $var:ident => $body:expr) => {{
        let $var = $var.downgrade();
        move |_| {
            if let Some($var) = $var.upgrade() {
                $body
            }
        }
    }};
    (@strong $var:ident => $body:expr) => {{
        let $var = $var.clone();
        move |_| $body
    }};
}

use clone;
