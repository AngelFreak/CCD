use crate::api::SharedPocketBaseClient;
use crate::models::SessionHistory;
use adw::prelude::*;
use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;

/// Session monitor view showing current session token usage
pub struct SessionMonitorView {
    container: gtk::Box,
    pb_client: SharedPocketBaseClient,
    project_id: String,
    current_session: Rc<RefCell<Option<SessionHistory>>>,
}

impl SessionMonitorView {
    /// Create a new session monitor view
    pub fn new(pb_client: SharedPocketBaseClient, project_id: String) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 12);

        let mut view = Self {
            container,
            pb_client,
            project_id,
            current_session: Rc::new(RefCell::new(None)),
        };

        view.setup_ui();
        view.load_current_session();

        view
    }

    /// Setup the UI
    fn setup_ui(&mut self) {
        // Session info card
        let card = gtk::Box::new(gtk::Orientation::Vertical, 8);
        card.set_margin_top(8);
        card.set_margin_bottom(8);
        card.set_margin_start(8);
        card.set_margin_end(8);
        card.add_css_class("session-card");

        // Token usage label
        let token_label = gtk::Label::new(Some("Token Usage"));
        token_label.set_xalign(0.0);
        token_label.add_css_class("caption");
        card.append(&token_label);

        // Progress bar for token usage
        let progress_bar = gtk::ProgressBar::new();
        progress_bar.add_css_class("token-progress");
        progress_bar.set_show_text(true);
        progress_bar.set_fraction(0.0);
        progress_bar.set_text(Some("0 / 200,000 tokens (0%)"));
        card.append(&progress_bar);

        // Session duration
        let duration_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        duration_box.set_margin_top(8);

        let duration_icon = gtk::Image::from_icon_name("appointment-symbolic");
        duration_box.append(&duration_icon);

        let duration_label = gtk::Label::new(Some("No active session"));
        duration_label.add_css_class("caption");
        duration_label.set_hexpand(true);
        duration_label.set_xalign(0.0);
        duration_box.append(&duration_label);

        card.append(&duration_box);

        // Facts extracted
        let facts_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        let facts_icon = gtk::Image::from_icon_name("emblem-documents-symbolic");
        facts_box.append(&facts_icon);

        let facts_label = gtk::Label::new(Some("0 facts extracted"));
        facts_label.add_css_class("caption");
        facts_label.set_hexpand(true);
        facts_label.set_xalign(0.0);
        facts_box.append(&facts_label);

        card.append(&facts_box);

        self.container.append(&card);

        // Warning message if near limit
        let warning_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        warning_box.set_margin_top(8);
        warning_box.set_visible(false);
        warning_box.add_css_class("warning");

        let warning_icon = gtk::Image::from_icon_name("dialog-warning-symbolic");
        warning_box.append(&warning_icon);

        let warning_label = gtk::Label::new(Some("Approaching context limit"));
        warning_label.set_wrap(true);
        warning_label.add_css_class("caption");
        warning_box.append(&warning_label);

        self.container.append(&warning_box);
    }

    /// Load current session
    fn load_current_session(&self) {
        let pb_client = self.pb_client.clone();
        let project_id = self.project_id.clone();
        let current_session = self.current_session.clone();

        glib::spawn_future_local(async move {
            match pb_client.list_sessions(&project_id).await {
                Ok(sessions) => {
                    // Get the most recent active session
                    let active = sessions.into_iter().find(|s| s.is_active());
                    *current_session.borrow_mut() = active;
                    // Update UI with session data
                    // This would be implemented with proper state management
                }
                Err(e) => {
                    log::error!("Failed to load sessions: {}", e);
                }
            }
        });
    }

    /// Update the UI with session data
    fn update_ui(&self, session: Option<&SessionHistory>) {
        // This would update the progress bar, labels, etc.
        // For now, this is a placeholder
        if let Some(sess) = session {
            log::info!(
                "Session: {} tokens ({:.1}%)",
                sess.token_count,
                sess.token_percentage()
            );
        }
    }

    /// Get the widget
    pub fn widget(&self) -> gtk::Box {
        self.container.clone()
    }
}
