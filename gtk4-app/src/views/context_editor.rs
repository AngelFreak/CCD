use crate::api::SharedPocketBaseClient;
use crate::models::{ContextSection, SectionType};
use crate::utils::generate_claude_md;
use adw::prelude::*;
use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;

/// Context editor view for managing project context sections
pub struct ContextEditorView {
    container: gtk::Box,
    sections_list: gtk::ListBox,
    pb_client: SharedPocketBaseClient,
    project_id: String,
    sections: Rc<RefCell<Vec<ContextSection>>>,
}

impl ContextEditorView {
    /// Create a new context editor view
    pub fn new(pb_client: SharedPocketBaseClient, project_id: String) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Create toolbar
        let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        toolbar.set_margin_top(12);
        toolbar.set_margin_bottom(12);
        toolbar.set_margin_start(12);
        toolbar.set_margin_end(12);

        let title = gtk::Label::new(Some("Context Sections"));
        title.add_css_class("heading");
        title.set_halign(gtk::Align::Start);
        title.set_hexpand(true);
        toolbar.append(&title);

        // Export button
        let export_btn = gtk::Button::builder()
            .icon_name("document-save-symbolic")
            .tooltip_text("Export to CLAUDE.md")
            .build();
        export_btn.add_css_class("flat");
        toolbar.append(&export_btn);

        // Copy button
        let copy_btn = gtk::Button::builder()
            .icon_name("edit-copy-symbolic")
            .tooltip_text("Copy to Clipboard")
            .build();
        copy_btn.add_css_class("flat");
        toolbar.append(&copy_btn);

        // Add section button
        let add_btn = gtk::Button::builder()
            .icon_name("list-add-symbolic")
            .tooltip_text("Add Section")
            .build();
        add_btn.add_css_class("flat");
        toolbar.append(&add_btn);

        container.append(&toolbar);

        // Create scrolled window for sections
        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .build();

        let sections_list = gtk::ListBox::new();
        sections_list.set_selection_mode(gtk::SelectionMode::None);
        sections_list.set_margin_top(12);
        sections_list.set_margin_bottom(12);
        sections_list.set_margin_start(12);
        sections_list.set_margin_end(12);

        scrolled.set_child(Some(&sections_list));
        container.append(&scrolled);

        let mut view = Self {
            container,
            sections_list,
            pb_client,
            project_id,
            sections: Rc::new(RefCell::new(Vec::new())),
        };

        view.load_sections();

        view
    }

    /// Load context sections
    fn load_sections(&self) {
        let pb_client = self.pb_client.clone();
        let project_id = self.project_id.clone();
        let sections = self.sections.clone();
        let sections_list = self.sections_list.clone();

        glib::spawn_future_local(async move {
            match pb_client.list_context_sections(&project_id).await {
                Ok(loaded_sections) => {
                    *sections.borrow_mut() = loaded_sections.clone();
                    Self::update_sections_list(&sections_list, &loaded_sections);
                }
                Err(e) => {
                    log::error!("Failed to load context sections: {}", e);
                }
            }
        });
    }

    /// Update the sections list
    fn update_sections_list(sections_list: &gtk::ListBox, sections: &[ContextSection]) {
        // Clear existing rows
        while let Some(row) = sections_list.first_child() {
            sections_list.remove(&row);
        }

        if sections.is_empty() {
            let empty_label = gtk::Label::new(Some("No context sections yet.\nClick + to add one."));
            empty_label.add_css_class("dim-label");
            empty_label.set_margin_top(32);
            empty_label.set_margin_bottom(32);
            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&empty_label));
            row.set_activatable(false);
            sections_list.append(&row);
            return;
        }

        for section in sections {
            let row = Self::create_section_row(section);
            sections_list.append(&row);
        }
    }

    /// Create a section row
    fn create_section_row(section: &ContextSection) -> gtk::ListBoxRow {
        let row_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
        row_box.set_margin_top(8);
        row_box.set_margin_bottom(8);
        row_box.set_margin_start(12);
        row_box.set_margin_end(12);

        // Header with title and type
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        let icon = gtk::Image::from_icon_name(section.section_type.icon_name());
        header.append(&icon);

        let title = gtk::Label::new(Some(&section.title));
        title.add_css_class("section-header");
        title.set_halign(gtk::Align::Start);
        title.set_hexpand(true);
        header.append(&title);

        let type_label = gtk::Label::new(Some(section.section_type.display_name()));
        type_label.add_css_class("dim-label");
        header.append(&type_label);

        row_box.append(&header);

        // Content preview
        let content_label = gtk::Label::new(Some(&section.content_preview()));
        content_label.set_wrap(true);
        content_label.set_xalign(0.0);
        content_label.add_css_class("dim-label");
        row_box.append(&content_label);

        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&row_box));
        row.set_activatable(true);

        row
    }

    /// Get the widget
    pub fn widget(&self) -> gtk::Box {
        self.container.clone()
    }
}
