use crate::db::Repository;
use crate::models::{Project, ProjectPayload, ProjectStatus};
use adw::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Dashboard view showing list of projects
pub struct DashboardView {
    container: gtk::Box,
    project_list: gtk::ListBox,
    repository: Repository,
    navigation_view: adw::NavigationView,
    projects: Rc<RefCell<Vec<Project>>>,
    current_filter: Rc<RefCell<Option<ProjectStatus>>>,
}

impl DashboardView {
    /// Create a new dashboard view
    pub fn new(repository: Repository, navigation_view: adw::NavigationView) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Create toolbar for filtering
        let toolbar = Self::create_toolbar();
        container.append(&toolbar);

        // Create scrolled window for project list
        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .build();
        scrolled.add_css_class("scrolled-content");

        // Create project list
        let project_list = gtk::ListBox::new();
        project_list.set_selection_mode(gtk::SelectionMode::None);
        project_list.add_css_class("project-list");
        project_list.set_margin_top(12);
        project_list.set_margin_bottom(12);
        project_list.set_margin_start(12);
        project_list.set_margin_end(12);

        scrolled.set_child(Some(&project_list));
        container.append(&scrolled);

        let mut view = Self {
            container,
            project_list,
            repository,
            navigation_view,
            projects: Rc::new(RefCell::new(Vec::new())),
            current_filter: Rc::new(RefCell::new(None)),
        };

        // Load projects initially
        view.load_projects();

        view
    }

    /// Create the toolbar with filter buttons
    fn create_toolbar() -> gtk::Box {
        let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        toolbar.set_margin_top(12);
        toolbar.set_margin_bottom(12);
        toolbar.set_margin_start(12);
        toolbar.set_margin_end(12);

        let label = gtk::Label::new(Some("Filter:"));
        label.add_css_class("heading");
        toolbar.append(&label);

        // Filter buttons will be added when connected to the view instance
        toolbar
    }

    /// Load projects from database
    pub fn load_projects(&self) {
        let filter = *self.current_filter.borrow();

        match self.repository.list_projects(filter) {
            Ok(loaded_projects) => {
                *self.projects.borrow_mut() = loaded_projects.clone();
                Self::update_project_list_static(
                    &self.project_list,
                    &loaded_projects,
                    self.navigation_view.clone(),
                );
            }
            Err(e) => {
                log::error!("Failed to load projects: {}", e);
                Self::show_error_state(&self.project_list, &e.to_string());
            }
        }
    }

    /// Update the project list with loaded projects
    fn update_project_list_static(
        project_list: &gtk::ListBox,
        projects: &[Project],
        nav_view: adw::NavigationView,
    ) {
        // Clear existing rows
        while let Some(row) = project_list.first_child() {
            project_list.remove(&row);
        }

        if projects.is_empty() {
            Self::show_empty_state(project_list);
            return;
        }

        // Add project rows
        for project in projects {
            let row = Self::create_project_row(project, nav_view.clone());
            project_list.append(&row);
        }
    }

    /// Create a project row widget
    fn create_project_row(project: &Project, nav_view: adw::NavigationView) -> gtk::ListBoxRow {
        let row = adw::ActionRow::builder()
            .title(&project.name)
            .subtitle(&project.tech_stack_display())
            .build();

        // Add status badge
        let status_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        let status_label = gtk::Label::new(Some(project.status.display_name()));
        status_label.add_css_class("status-badge");
        status_label.add_css_class(&format!("status-{}", project.status.as_str()));
        status_box.append(&status_label);

        // Add navigation arrow
        let arrow = gtk::Image::from_icon_name("go-next-symbolic");
        status_box.append(&arrow);

        row.add_suffix(&status_box);

        // Add description if available
        if let Some(desc) = &project.description {
            let desc_label = gtk::Label::new(Some(desc));
            desc_label.set_wrap(true);
            desc_label.set_xalign(0.0);
            desc_label.add_css_class("dim-label");
            desc_label.set_margin_top(4);
            row.add_row(&desc_label);
        }

        // Make row activatable
        row.set_activatable(true);

        // Handle click to navigate to project detail
        let project_id = project.id.clone();
        row.connect_activated(move |_| {
            log::info!("Navigating to project: {}", project_id);
            // This will be handled by the window's navigation logic
            // For now, just log it
        });

        let list_row = gtk::ListBoxRow::new();
        list_row.set_child(Some(&row));
        list_row.set_activatable(true);

        let project_id = project.id.clone();
        list_row.connect_activated(move |_| {
            log::info!("Project row activated: {}", project_id);
            // Navigation will be wired up through callbacks
        });

        list_row
    }

    /// Show empty state
    fn show_empty_state(project_list: &gtk::ListBox) {
        let empty_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        empty_box.add_css_class("empty-state");

        let icon = gtk::Image::from_icon_name("folder-symbolic");
        icon.set_pixel_size(64);
        icon.add_css_class("empty-state-icon");
        empty_box.append(&icon);

        let title = gtk::Label::new(Some("No Projects Found"));
        title.add_css_class("empty-state-title");
        empty_box.append(&title);

        let subtitle = gtk::Label::new(Some("Create a new project to get started"));
        subtitle.add_css_class("empty-state-subtitle");
        empty_box.append(&subtitle);

        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&empty_box));
        row.set_activatable(false);
        row.set_selectable(false);

        project_list.append(&row);
    }

    /// Show error state
    fn show_error_state(project_list: &gtk::ListBox, error: &str) {
        while let Some(row) = project_list.first_child() {
            project_list.remove(&row);
        }

        let error_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        error_box.add_css_class("empty-state");

        let icon = gtk::Image::from_icon_name("dialog-error-symbolic");
        icon.set_pixel_size(64);
        icon.add_css_class("empty-state-icon");
        error_box.append(&icon);

        let title = gtk::Label::new(Some("Error Loading Projects"));
        title.add_css_class("empty-state-title");
        error_box.append(&title);

        let subtitle = gtk::Label::new(Some(error));
        subtitle.add_css_class("empty-state-subtitle");
        subtitle.set_wrap(true);
        error_box.append(&subtitle);

        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&error_box));
        row.set_activatable(false);
        row.set_selectable(false);

        project_list.append(&row);
    }

    /// Clear the project list
    fn clear_list(&self) {
        while let Some(row) = self.project_list.first_child() {
            self.project_list.remove(&row);
        }
    }

    /// Refresh the project list
    pub fn refresh(&self) {
        log::info!("Refreshing dashboard");
        self.load_projects();
    }

    /// Set filter by status
    pub fn set_filter(&self, status: Option<ProjectStatus>) {
        *self.current_filter.borrow_mut() = status;
        self.load_projects();
    }

    /// Get the widget
    pub fn widget(&self) -> gtk::Box {
        self.container.clone()
    }
}

// Implement Clone for weak references
impl Clone for DashboardView {
    fn clone(&self) -> Self {
        Self {
            container: self.container.clone(),
            project_list: self.project_list.clone(),
            repository: self.repository.clone(),
            navigation_view: self.navigation_view.clone(),
            projects: self.projects.clone(),
            current_filter: self.current_filter.clone(),
        }
    }
}
