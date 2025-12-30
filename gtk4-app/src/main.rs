mod db;
mod models;
mod utils;
mod views;
mod window;

use adw::prelude::*;
use anyhow::Result;
use db::{Database, Repository};
use std::sync::Arc;
use window::MainWindow;

const APP_ID: &str = "com.github.claudecontexttracker";

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Claude Context Tracker GTK4 application");

    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK");

    // Initialize libadwaita
    adw::init().expect("Failed to initialize libadwaita");

    // Create the application
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    // Setup signal handlers
    app.connect_startup(|_| {
        log::info!("Application startup");
        load_css();
    });

    app.connect_activate(build_ui);

    // Run the application
    let exit_code = app.run();

    log::info!("Application exiting with code: {}", exit_code);
    Ok(())
}

/// Build the main UI
fn build_ui(app: &adw::Application) {
    log::info!("Building UI");

    // Create embedded database
    let database = match Database::new(None) {
        Ok(db) => {
            log::info!("Database initialized at: {}", db.db_path().display());
            db
        }
        Err(e) => {
            log::error!("Failed to initialize database: {}", e);
            show_error_dialog(app, "Database Initialization Failed", &e.to_string());
            return;
        }
    };

    // Create repository for database operations
    let repository = Repository::new(database.into_shared());

    // Create main window
    let window = MainWindow::new(app, repository);
    window.present();
}

/// Load custom CSS for styling
fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("../resources/style.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    log::info!("CSS loaded");
}

/// Show an error dialog
fn show_error_dialog(app: &adw::Application, title: &str, message: &str) {
    let dialog = adw::AlertDialog::builder()
        .heading(title)
        .body(message)
        .build();

    dialog.add_response("ok", "OK");
    dialog.set_default_response(Some("ok"));

    // Show dialog on the active window or create a temporary one
    if let Some(window) = app.active_window() {
        dialog.present(Some(&window));
    } else {
        let temp_window = adw::ApplicationWindow::builder()
            .application(app)
            .build();
        dialog.present(Some(&temp_window));
    }
}
