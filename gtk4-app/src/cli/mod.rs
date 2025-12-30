pub mod commands;

use clap::{Parser, Subcommand};

/// Claude Context Tracker - Unified CLI and GUI application
#[derive(Parser)]
#[command(name = "claude-context-tracker")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Pull project context to CLAUDE.md file
    Pull {
        /// Project name or ID
        project: String,

        /// Output file path (default: ./CLAUDE.md)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Push session summary to project history
    Push {
        /// Project name or ID
        project: String,

        /// Session summary
        summary: String,

        /// Token count for this session
        #[arg(short, long)]
        tokens: Option<i64>,
    },

    /// Show status of active project and token usage
    Status {
        /// Project name or ID (optional, shows all if not specified)
        project: Option<String>,
    },

    /// Switch active project
    Switch {
        /// Project name or ID
        project: String,
    },

    /// Show diff between sessions
    Diff {
        /// Project name or ID
        project: String,

        /// Session ID to compare from (optional, uses previous if not specified)
        #[arg(short, long)]
        from: Option<String>,

        /// Session ID to compare to (optional, uses latest if not specified)
        #[arg(short, long)]
        to: Option<String>,
    },

    /// List all projects
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
    },

    /// Create a new project
    New {
        /// Project name
        name: String,

        /// Repository path
        #[arg(short, long)]
        repo: Option<String>,

        /// Tech stack (comma-separated)
        #[arg(short, long)]
        tech: Option<String>,

        /// Description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Start background monitoring daemon
    Monitor {
        /// Project name or ID to monitor
        project: String,

        /// Claude Code logs directory (auto-detected if not specified)
        #[arg(short, long)]
        logs_dir: Option<String>,
    },

    /// Launch GUI (default if no command specified)
    Gui,
}
