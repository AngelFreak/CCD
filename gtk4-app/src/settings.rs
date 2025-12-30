use adw::prelude::*;
use gtk::glib;
use std::path::PathBuf;

/// Settings dialog for application preferences
pub struct SettingsDialog {
    dialog: adw::PreferencesWindow,
}

impl SettingsDialog {
    /// Create a new settings dialog
    pub fn new(parent: &impl IsA<gtk::Window>) -> Self {
        let dialog = adw::PreferencesWindow::builder()
            .title("Preferences")
            .modal(true)
            .transient_for(parent)
            .search_enabled(false)
            .build();

        // General settings page
        let general_page = Self::create_general_page();
        dialog.add(&general_page);

        // Monitoring settings page
        let monitoring_page = Self::create_monitoring_page();
        dialog.add(&monitoring_page);

        // Appearance settings page
        let appearance_page = Self::create_appearance_page();
        dialog.add(&appearance_page);

        Self { dialog }
    }

    /// Create general settings page
    fn create_general_page() -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder()
            .title("General")
            .icon_name("preferences-system-symbolic")
            .build();

        // Database group
        let db_group = adw::PreferencesGroup::builder()
            .title("Database")
            .description("Configure database location and storage")
            .build();

        let db_location = Self::get_database_location();
        let db_row = adw::ActionRow::builder()
            .title("Database Location")
            .subtitle(&db_location)
            .build();

        let db_button = gtk::Button::builder()
            .icon_name("document-open-symbolic")
            .valign(gtk::Align::Center)
            .tooltip_text("Open database folder")
            .build();
        db_button.add_css_class("flat");

        db_button.connect_clicked(move |_| {
            if let Some(parent_dir) = PathBuf::from(&db_location).parent() {
                let uri = format!("file://{}", parent_dir.display());
                let _ = gtk::UriLauncher::new(&uri).launch(
                    None::<&gtk::Window>,
                    None::<&gtk::gio::Cancellable>,
                    |_| {},
                );
            }
        });

        db_row.add_suffix(&db_button);
        db_group.add(&db_row);

        page.add(&db_group);
        page
    }

    /// Create monitoring settings page
    fn create_monitoring_page() -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder()
            .title("Monitoring")
            .icon_name("emblem-synchronizing-symbolic")
            .build();

        // Auto-start group
        let autostart_group = adw::PreferencesGroup::builder()
            .title("Auto-Start")
            .description("Automatically start monitoring when application launches")
            .build();

        let autostart_row = adw::SwitchRow::builder()
            .title("Enable Auto-Start Monitoring")
            .subtitle("Start monitoring active project on launch")
            .build();

        autostart_group.add(&autostart_row);

        // Logs directory group
        let logs_group = adw::PreferencesGroup::builder()
            .title("Claude Code Logs")
            .description("Configure where to find Claude Code conversation logs")
            .build();

        let logs_location = Self::get_default_logs_dir();
        let logs_row = adw::ActionRow::builder()
            .title("Logs Directory")
            .subtitle(&logs_location)
            .build();

        let logs_button = gtk::Button::builder()
            .icon_name("folder-open-symbolic")
            .valign(gtk::Align::Center)
            .tooltip_text("Choose logs directory")
            .build();
        logs_button.add_css_class("flat");

        logs_button.connect_clicked(move |btn| {
            let dialog = gtk::FileDialog::builder()
                .title("Select Claude Code Logs Directory")
                .modal(true)
                .build();

            let window = btn.root().and_downcast::<gtk::Window>();
            dialog.select_folder(
                window.as_ref(),
                None::<&gtk::gio::Cancellable>,
                move |result| {
                    if let Ok(file) = result {
                        if let Some(path) = file.path() {
                            log::info!("Selected logs directory: {}", path.display());
                            // TODO: Save to settings
                        }
                    }
                },
            );
        });

        logs_row.add_suffix(&logs_button);
        logs_group.add(&logs_row);

        page.add(&autostart_group);
        page.add(&logs_group);
        page
    }

    /// Create appearance settings page
    fn create_appearance_page() -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder()
            .title("Appearance")
            .icon_name("preferences-desktop-theme-symbolic")
            .build();

        // Theme group
        let theme_group = adw::PreferencesGroup::builder()
            .title("Theme")
            .description("Choose application color scheme")
            .build();

        let theme_row = adw::ComboRow::builder()
            .title("Color Scheme")
            .subtitle("Select light, dark, or follow system")
            .build();

        let model = gtk::StringList::new(&["System Default", "Light", "Dark"]);
        theme_row.set_model(Some(&model));
        theme_row.set_selected(0);

        theme_row.connect_selected_notify(|row| {
            let selected = row.selected();
            log::info!("Theme changed to: {}", selected);
            // TODO: Apply theme
        });

        theme_group.add(&theme_row);

        // Token warning group
        let token_group = adw::PreferencesGroup::builder()
            .title("Token Warning")
            .description("Set threshold for context size warnings")
            .build();

        let token_row = adw::SpinRow::builder()
            .title("Warning Threshold")
            .subtitle("Show warning at this token count")
            .build();

        let adjustment = gtk::Adjustment::new(
            170000.0, // value
            100000.0, // min
            195000.0, // max
            1000.0,   // step
            10000.0,  // page increment
            0.0,      // page size
        );
        token_row.set_adjustment(Some(&adjustment));

        token_group.add(&token_row);

        page.add(&theme_group);
        page.add(&token_group);
        page
    }

    /// Get database location
    fn get_database_location() -> String {
        if let Some(data_dir) = dirs::data_dir() {
            let db_path = data_dir.join("claude-context-tracker").join("tracker.db");
            db_path.to_string_lossy().to_string()
        } else {
            "~/.local/share/claude-context-tracker/tracker.db".to_string()
        }
    }

    /// Get default logs directory
    fn get_default_logs_dir() -> String {
        if let Some(home) = home::home_dir() {
            home.join(".claude").join("logs").to_string_lossy().to_string()
        } else {
            "~/.claude/logs".to_string()
        }
    }

    /// Show the dialog
    pub fn present(&self) {
        self.dialog.present();
    }
}
