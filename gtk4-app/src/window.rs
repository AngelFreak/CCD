use crate::api::SharedPocketBaseClient;
use crate::models::Project;
use crate::views::{DashboardView, ProjectDetailView};
use adw::prelude::*;
use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

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
    pb_client: SharedPocketBaseClient,
    state: Rc<RefCell<NavigationState>>,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(app: &adw::Application, pb_client: SharedPocketBaseClient) -> Self {
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
            pb_client,
            state,
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
    }

    /// Create the dashboard view
    fn create_dashboard_view(&self) -> gtk::Box {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Header bar
        let header = adw::HeaderBar::new();

        // Add new project button
        let new_project_btn = gtk::Button::builder()
            .icon_name("list-add-symbolic")
            .tooltip_text("Create New Project")
            .build();
        new_project_btn.add_css_class("flat");

        let pb_client = self.pb_client.clone();
        let nav_view = self.navigation_view.clone();
        new_project_btn.connect_clicked(clone!(@weak nav_view => move |_| {
            Self::show_new_project_dialog(pb_client.clone(), nav_view.clone());
        }));

        header.pack_end(&new_project_btn);

        // Refresh button
        let refresh_btn = gtk::Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Refresh Projects")
            .build();
        refresh_btn.add_css_class("flat");
        header.pack_end(&refresh_btn);

        container.append(&header);

        // Dashboard content
        let dashboard_view = DashboardView::new(self.pb_client.clone(), self.navigation_view.clone());
        container.append(&dashboard_view.widget());

        // Connect refresh button
        refresh_btn.connect_clicked(clone!(@weak dashboard_view => move |_| {
            dashboard_view.refresh();
        }));

        container
    }

    /// Show dialog to create a new project
    fn show_new_project_dialog(pb_client: SharedPocketBaseClient, nav_view: adw::NavigationView) {
        // This will be implemented when we create the dashboard view
        log::info!("New project dialog requested");
    }

    /// Setup keyboard shortcuts
    fn setup_shortcuts(&self) {
        let shortcuts = gtk::EventControllerKey::new();

        // Ctrl+Q to quit
        let window = self.window.clone();
        shortcuts.connect_key_pressed(move |_, key, _, modifier| {
            if modifier.contains(gtk::gdk::ModifierType::CONTROL_MASK) {
                match key {
                    gtk::gdk::Key::q => {
                        window.close();
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
            self.pb_client.clone(),
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
