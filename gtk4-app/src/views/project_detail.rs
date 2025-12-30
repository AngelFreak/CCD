use crate::api::SharedPocketBaseClient;
use crate::models::{ContextSection, ExtractedFact, Project, SessionHistory};
use crate::views::{ContextEditorView, FactsListView, SessionMonitorView};
use adw::prelude::*;
use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;

/// Project detail view with tabbed interface
pub struct ProjectDetailView {
    container: gtk::Box,
    pb_client: SharedPocketBaseClient,
    project_id: String,
    project: Rc<RefCell<Option<Project>>>,
}

impl ProjectDetailView {
    /// Create a new project detail view
    pub fn new(
        pb_client: SharedPocketBaseClient,
        project_id: String,
        _navigation_view: adw::NavigationView,
    ) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let mut view = Self {
            container,
            pb_client,
            project_id,
            project: Rc::new(RefCell::new(None)),
        };

        view.setup_ui();
        view.load_project();

        view
    }

    /// Setup the UI
    fn setup_ui(&mut self) {
        // Main content area with tabs
        let main_content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        main_content.set_hexpand(true);

        // Tab view for different sections
        let tab_view = adw::TabView::new();

        // Context Editor Tab
        let context_editor = ContextEditorView::new(
            self.pb_client.clone(),
            self.project_id.clone(),
        );
        let context_page = adw::TabPage::builder()
            .child(&context_editor.widget())
            .title("Context")
            .build();
        tab_view.append(&context_page);

        // Session History Tab (placeholder)
        let session_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        session_box.set_margin_top(16);
        session_box.set_margin_bottom(16);
        session_box.set_margin_start(16);
        session_box.set_margin_end(16);

        let session_label = gtk::Label::new(Some("Session history will be displayed here"));
        session_label.add_css_class("dim-label");
        session_box.append(&session_label);

        let session_page = adw::TabPage::builder()
            .child(&session_box)
            .title("Sessions")
            .build();
        tab_view.append(&session_page);

        // Compressed Context Tab (placeholder)
        let compressed_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        compressed_box.set_margin_top(16);
        compressed_box.set_margin_bottom(16);
        compressed_box.set_margin_start(16);
        compressed_box.set_margin_end(16);

        let compressed_label = gtk::Label::new(Some("Compressed context view (top facts) will be displayed here"));
        compressed_label.add_css_class("dim-label");
        compressed_box.append(&compressed_label);

        let compressed_page = adw::TabPage::builder()
            .child(&compressed_box)
            .title("Compressed")
            .build();
        tab_view.append(&compressed_page);

        // Tab bar
        let tab_bar = adw::TabBar::builder()
            .view(&tab_view)
            .build();

        main_content.append(&tab_bar);
        main_content.append(&tab_view);

        self.container.append(&main_content);

        // Sidebar for facts and session monitor
        let sidebar = self.create_sidebar();
        self.container.append(&sidebar);
    }

    /// Create the right sidebar
    fn create_sidebar(&self) -> gtk::Box {
        let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 0);
        sidebar.set_width_request(320);
        sidebar.add_css_class("sidebar");

        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .build();

        let sidebar_content = gtk::Box::new(gtk::Orientation::Vertical, 16);
        sidebar_content.set_margin_top(16);
        sidebar_content.set_margin_bottom(16);
        sidebar_content.set_margin_start(16);
        sidebar_content.set_margin_end(16);

        // Session Monitor
        let monitor_section = gtk::Box::new(gtk::Orientation::Vertical, 8);
        let monitor_title = gtk::Label::new(Some("Session Monitor"));
        monitor_title.add_css_class("sidebar-title");
        monitor_title.set_xalign(0.0);
        monitor_section.append(&monitor_title);

        let session_monitor = SessionMonitorView::new(self.pb_client.clone(), self.project_id.clone());
        monitor_section.append(&session_monitor.widget());

        sidebar_content.append(&monitor_section);

        // Facts List
        let facts_section = gtk::Box::new(gtk::Orientation::Vertical, 8);
        let facts_title = gtk::Label::new(Some("Extracted Facts"));
        facts_title.add_css_class("sidebar-title");
        facts_title.set_xalign(0.0);
        facts_section.append(&facts_title);

        let facts_list = FactsListView::new(self.pb_client.clone(), self.project_id.clone());
        facts_section.append(&facts_list.widget());

        sidebar_content.append(&facts_section);

        scrolled.set_child(Some(&sidebar_content));
        sidebar.append(&scrolled);

        sidebar
    }

    /// Load project details
    fn load_project(&self) {
        let pb_client = self.pb_client.clone();
        let project_id = self.project_id.clone();
        let project = self.project.clone();

        glib::spawn_future_local(async move {
            match pb_client.get_project(&project_id).await {
                Ok(loaded_project) => {
                    log::info!("Loaded project: {}", loaded_project.name);
                    *project.borrow_mut() = Some(loaded_project);
                }
                Err(e) => {
                    log::error!("Failed to load project: {}", e);
                }
            }
        });
    }

    /// Get the widget
    pub fn widget(&self) -> gtk::Box {
        self.container.clone()
    }
}
