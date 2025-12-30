mod api;
mod models;
mod utils;
mod views;
mod window;

use adw::prelude::*;
use anyhow::Result;
use api::PocketBaseClient;
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

    // Create PocketBase client (read URL from environment or use default)
    let pb_url = std::env::var("POCKETBASE_URL").ok();
    let pb_client = match PocketBaseClient::new(pb_url) {
        Ok(client) => Arc::new(client),
        Err(e) => {
            log::error!("Failed to create PocketBase client: {}", e);
            show_error_dialog(app, "Failed to initialize", &e.to_string());
            return;
        }
    };

    // Check PocketBase connection
    let client_clone = pb_client.clone();
    glib::spawn_future_local(async move {
        match client_clone.health_check().await {
            true => log::info!("PocketBase connection successful"),
            false => {
                log::warn!("PocketBase server not reachable at startup");
                // Don't block the UI, just warn
            }
        }
    });

    // Create main window
    let window = MainWindow::new(app, pb_client);
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
