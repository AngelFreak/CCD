mod cli;
mod db;
mod models;
mod monitor;
mod notifications;
mod settings;
mod utils;
mod views;
mod window;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use db::{Database, Repository};

const APP_ID: &str = "com.github.claudecontexttracker";

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize database (always needed)
    let database = Database::new(None)?;
    let repository = Repository::new(database.into_shared());

    // Execute based on command (or launch GUI if no command)
    match cli.command {
        Some(Commands::Pull { project, output }) => {
            cli::commands::pull_command(&repository, &project, output)?;
        }
        Some(Commands::Push { project, summary, tokens }) => {
            cli::commands::push_command(&repository, &project, summary, tokens)?;
        }
        Some(Commands::Status { project }) => {
            cli::commands::status_command(&repository, project)?;
        }
        Some(Commands::List { status }) => {
            cli::commands::list_command(&repository, status)?;
        }
        Some(Commands::New { name, repo, tech, description }) => {
            cli::commands::new_command(&repository, name, repo, tech, description)?;
        }
        Some(Commands::Diff { project, from, to }) => {
            cli::commands::diff_command(&repository, &project, from, to)?;
        }
        Some(Commands::Monitor { project, logs_dir }) => {
            run_daemon_mode(repository, project, logs_dir)?;
        }
        Some(Commands::Switch { .. }) => {
            println!("Switch command not yet implemented");
        }
        Some(Commands::Gui) | None => {
            // Default: launch GUI
            run_gui_mode(repository)?;
        }
    }

    Ok(())
}

/// Run in daemon mode (file monitoring only)
fn run_daemon_mode(repository: Repository, project: String, logs_dir: Option<String>) -> Result<()> {
    log::info!("Starting daemon mode for project: {}", project);

    // Find project
    let proj = cli::commands::find_project(&repository, &project)?;

    // Convert logs_dir to PathBuf
    let logs_path = logs_dir.map(std::path::PathBuf::from);

    // Start monitoring (blocking)
    let monitor = monitor::LogMonitor::new(proj.id, repository, logs_path)?;
    monitor.start_monitoring()?;

    Ok(())
}

/// Run in GUI mode
fn run_gui_mode(repository: Repository) -> Result<()> {
    use adw::prelude::*;
    use window::MainWindow;

    log::info!("Starting GUI mode");

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

    // Build UI on activate
    let repo_clone = repository.clone();
    app.connect_activate(move |app| {
        build_ui(app, repo_clone.clone());
    });

    // Run the application
    let exit_code = app.run();

    log::info!("Application exiting with code: {}", exit_code);
    Ok(())
}

/// Build the main UI
fn build_ui(app: &adw::Application, repository: Repository) {
    log::info!("Building UI");

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
