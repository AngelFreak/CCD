use crate::db::Repository;
use crate::models::ExtractedFact;
use adw::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Facts list view showing extracted facts
pub struct FactsListView {
    container: gtk::Box,
    facts_list: gtk::ListBox,
    repository: Repository,
    project_id: String,
    facts: Rc<RefCell<Vec<ExtractedFact>>>,
}

impl FactsListView {
    /// Create a new facts list view
    pub fn new(repository: Repository, project_id: String) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Create scrolled window
        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .min_content_height(200)
            .build();

        let facts_list = gtk::ListBox::new();
        facts_list.set_selection_mode(gtk::SelectionMode::None);
        facts_list.add_css_class("compact");

        scrolled.set_child(Some(&facts_list));
        container.append(&scrolled);

        let mut view = Self {
            container,
            facts_list,
            repository,
            project_id,
            facts: Rc::new(RefCell::new(Vec::new())),
        };

        view.load_facts();

        view
    }

    /// Load facts from database
    fn load_facts(&self) {
        match self.repository.list_facts(&self.project_id, false) {
            Ok(loaded_facts) => {
                // Take top 10 most important facts
                let top_facts: Vec<_> = loaded_facts.into_iter().take(10).collect();
                *self.facts.borrow_mut() = top_facts.clone();
                Self::update_facts_list(&self.facts_list, &top_facts);
            }
            Err(e) => {
                log::error!("Failed to load facts: {}", e);
            }
        }
    }

    /// Update the facts list
    fn update_facts_list(facts_list: &gtk::ListBox, facts: &[ExtractedFact]) {
        // Clear existing rows
        while let Some(row) = facts_list.first_child() {
            facts_list.remove(&row);
        }

        if facts.is_empty() {
            let empty_label = gtk::Label::new(Some("No facts extracted yet"));
            empty_label.add_css_class("dim-label");
            empty_label.set_margin_top(16);
            empty_label.set_margin_bottom(16);
            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&empty_label));
            row.set_activatable(false);
            facts_list.append(&row);
            return;
        }

        for fact in facts {
            let row = Self::create_fact_row(fact);
            facts_list.append(&row);
        }
    }

    /// Create a fact row
    fn create_fact_row(fact: &ExtractedFact) -> gtk::ListBoxRow {
        let row_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        row_box.set_margin_top(6);
        row_box.set_margin_bottom(6);
        row_box.set_margin_start(6);
        row_box.set_margin_end(6);

        // Header with type and importance
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 6);

        let type_label = gtk::Label::new(Some(fact.fact_type.display_name()));
        type_label.add_css_class("fact-badge");
        type_label.add_css_class(&format!("fact-{}", fact.fact_type.as_str()));
        header.append(&type_label);

        let importance_label = gtk::Label::new(Some(&fact.importance_stars()));
        importance_label.add_css_class("importance-stars");
        if fact.is_high_importance() {
            importance_label.add_css_class("importance-high");
        } else if fact.is_low_importance() {
            importance_label.add_css_class("importance-low");
        }
        header.append(&importance_label);

        let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        header.append(&spacer);

        let age_label = gtk::Label::new(Some(&fact.age_display()));
        age_label.add_css_class("dim-label");
        age_label.set_css_classes(&["dim-label", "caption"]);
        header.append(&age_label);

        row_box.append(&header);

        // Content
        let content_label = gtk::Label::new(Some(&fact.content_preview()));
        content_label.set_wrap(true);
        content_label.set_xalign(0.0);
        content_label.set_css_classes(&["caption"]);
        if fact.stale {
            content_label.add_css_class("fact-stale");
        }
        row_box.append(&content_label);

        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&row_box));
        row.set_activatable(false);

        row
    }

    /// Get the widget
    pub fn widget(&self) -> gtk::Box {
        self.container.clone()
    }
}
