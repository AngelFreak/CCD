# Claude Context Tracker

> **Native Linux application for managing Claude Code context across projects**

A modern GTK4 desktop application that helps you track, manage, and maintain context for your Claude Code projects. Never lose important details when conversations get compacted.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)
![GTK](https://img.shields.io/badge/GTK-4-orange.svg)

## Features

### 🖥️ Native Desktop Application
- **Modern GTK4 Interface** - Clean, native GNOME application with libadwaita
- **Background Monitoring** - Toggle to automatically track Claude Code conversations
- **Project Management** - Organize multiple projects with easy switching
- **Context Editing** - Structured sections for project overview, tech stack, decisions, and gotchas

### 🤖 Intelligent Fact Extraction
- **Automatic Detection** - Extracts decisions, blockers, TODOs, file changes, dependencies, and insights
- **Importance Scoring** - Facts auto-scored 1-5 based on type and content
- **Staleness Detection** - Automatically marks outdated facts (resolved TODOs, old blockers)
- **Session Tracking** - Monitor token usage and conversation history

### 💻 Command Line Interface
- **Pull Context** - Generate `CLAUDE.md` files from stored project data
- **Push Sessions** - Save conversation summaries and token counts
- **Project Management** - Create, list, and switch between projects
- **Status Monitoring** - Check current project and token usage
- **Diff View** - Compare context changes between sessions

### 🔄 Background Daemon
- **File Monitoring** - Watches `~/.claude/logs/` for conversation files
- **Real-time Extraction** - Processes conversations as they happen
- **Embedded Database** - SQLite storage with no external dependencies

## Tech Stack

- **Language**: Rust
- **UI Framework**: GTK4 + libadwaita
- **Database**: Embedded SQLite (rusqlite with connection pooling)
- **Architecture**: Single binary with three modes (GUI, CLI, daemon)
- **Build System**: Cargo with automated .deb packaging

## Installation

### Download Pre-built Package (Recommended)

```bash
# Download the latest .deb from GitHub Releases
wget https://github.com/AngelFreak/CCD/releases/latest/download/claude-context-tracker_<version>_amd64.deb

# Install
sudo apt install ./claude-context-tracker_<version>_amd64.deb
```

**What you get:**
- ✅ Single native application binary
- ✅ Desktop launcher (Applications → Development → Claude Context Tracker)
- ✅ CLI commands available: `claude-context-tracker`
- ✅ Embedded SQLite database (no external services)
- ✅ All dependencies included

### Build from Source

**Prerequisites:**
- Rust 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- GTK4 development libraries
- libadwaita development libraries

**Ubuntu/Debian:**
```bash
# Install dependencies
sudo apt-get install libgtk-4-dev libadwaita-1-dev build-essential pkg-config

# Clone and build
git clone https://github.com/AngelFreak/CCD.git
cd CCD/gtk4-app
cargo build --release

# Install binary
sudo cp target/release/claude-context-tracker /usr/local/bin/

# Or build .deb package
cargo install cargo-deb
cargo deb
sudo apt install ./target/debian/*.deb
```

## Usage

### Launch GUI

**From Applications Menu:**
```
Applications → Development → Claude Context Tracker
```

**From Terminal:**
```bash
claude-context-tracker gui
```

### Enable Background Monitoring

1. Launch the application
2. Click the **Monitor** toggle in the header bar
3. Label changes to "Monitoring" (orange) when active
4. Facts are automatically extracted from `~/.claude/logs/` conversations

### CLI Commands

```bash
# Pull context to CLAUDE.md
claude-context-tracker pull <project-name>

# Pull to specific file
claude-context-tracker pull <project-name> --output /path/to/file.md

# Save session summary
claude-context-tracker push <project-name> "Implemented new feature"

# Save with token count
claude-context-tracker push <project-name> "Fixed bugs" --tokens 45000

# Check status
claude-context-tracker status

# List all projects
claude-context-tracker list

# Filter by status
claude-context-tracker list --status active

# Create new project
claude-context-tracker new "My Project" --repo /path/to/repo

# Add metadata
claude-context-tracker new "My Project" \
  --repo /path/to/repo \
  --tech "Rust,GTK4,SQLite" \
  --description "My awesome project"

# View changes between sessions
claude-context-tracker diff <project-name>

# Run daemon mode (background monitoring)
claude-context-tracker monitor <project-name>

# Custom logs directory
claude-context-tracker monitor <project-name> --logs-dir /custom/path
```

### CLAUDE.md Format

When you run `pull`, the generated `CLAUDE.md` includes:

```markdown
# Project Name

## Project Overview
[Your overview section]

## Current State
Status: Active
Priority: High
Tech Stack: Rust, GTK4, SQLite

## Next Steps
1. [Action items from context]

## Gotchas
- [Important notes and warnings]

## Decisions Log
- [Key architectural decisions]

## Important Facts
### Blockers (Priority 5)
- [Critical blocking issues]

### Decisions (Priority 4)
- [Recent important decisions]

### TODOs (Priority 3)
- [Active tasks]
```

## How It Works

### Architecture

```
┌─────────────────────────────────────────┐
│         Single Rust Binary              │
├─────────────────────────────────────────┤
│  GUI Mode  │  CLI Mode  │  Daemon Mode  │
├─────────────────────────────────────────┤
│          Repository Pattern             │
├─────────────────────────────────────────┤
│       Embedded SQLite Database          │
│    (~/.local/share/claude-context-      │
│         tracker/tracker.db)             │
└─────────────────────────────────────────┘
```

### Fact Extraction Engine

**Detected Fact Types:**
- **Decisions** - `"decided to..."`, `"we're using..."`, `"chose..."`
- **Blockers** - `"blocked by..."`, `"can't proceed..."`, `"waiting for..."`
- **TODOs** - `"TODO:"`, `"need to..."`, `"should implement..."`
- **File Changes** - `"created file..."`, `"modified..."`, `"renamed..."`
- **Dependencies** - `"added dependency..."`, `"using library..."`
- **Insights** - `"learned that..."`, `"discovered..."`, `"realized..."`

**Importance Scoring:**
```rust
Base Scores:
- Blocker: 5 (always high priority)
- Decision/Dependency: 4
- TODO/FileChange/Insight: 3

Bonuses:
+ Content analysis (critical, urgent, security, breaking): +1-2
+ Recency bonus (< 1 hour old): +1
```

**Staleness Detection:**
- **Content-based**: Marks facts with "resolved", "fixed", "done", "completed", "merged", "closed"
- **Time-based**: Different thresholds per type
  - Blockers: 3 days
  - TODOs: 14 days
  - File Changes: 30 days
  - Dependencies: 90 days
  - Decisions: 180 days
  - Insights: 90 days

### Database Schema

**Projects**: Track multiple development projects
```sql
id, name, slug, repo_path, status, priority, tech_stack, description, created, updated
```

**Context Sections**: Structured markdown sections
```sql
id, project, section_type, title, content, order, created, updated
```

**Extracted Facts**: Automatically detected information
```sql
id, project, session, fact_type, content, importance, stale, created, updated
```

**Session History**: Conversation tracking
```sql
id, project, summary, facts_extracted, token_count, session_start, session_end, created
```

## Development

### Project Structure

```
CCD/
├── gtk4-app/           # Main Rust application
│   ├── src/
│   │   ├── main.rs     # Entry point (GUI/CLI/daemon modes)
│   │   ├── window.rs   # GTK4 main window
│   │   ├── db/         # Database layer
│   │   ├── models/     # Data models
│   │   ├── views/      # GTK4 view components
│   │   ├── monitor/    # Log monitoring & fact extraction
│   │   ├── cli/        # CLI commands
│   │   └── utils/      # Utilities
│   ├── resources/      # CSS, desktop files, icons
│   └── Cargo.toml      # Dependencies & deb metadata
├── .github/
│   └── workflows/
│       └── release.yml # Automated .deb builds
└── README.md
```

### Building

```bash
cd gtk4-app

# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run GUI
cargo run -- gui

# Run CLI
cargo run -- pull myproject

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Build .deb package
cargo deb
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and formatting (`cargo test && cargo fmt`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Configuration

### Database Location

Default: `~/.local/share/claude-context-tracker/tracker.db`

The database is created automatically on first run.

### Claude Code Logs

Default: `~/.claude/logs/`

Override with `--logs-dir` flag:
```bash
claude-context-tracker monitor myproject --logs-dir /custom/logs
```

### Desktop Integration

Desktop file location: `/usr/share/applications/com.github.claudecontexttracker.desktop`

Icon location: Inherited from system theme

## Troubleshooting

### Application won't start

```bash
# Check if GTK4 is installed
pkg-config --modversion gtk4

# Should be >= 4.12
# If not: sudo apt install libgtk-4-1 libadwaita-1-0
```

### Database errors

```bash
# Reset database (WARNING: deletes all data)
rm ~/.local/share/claude-context-tracker/tracker.db

# Restart application to recreate
```

### Monitoring not working

```bash
# Check Claude Code logs directory exists
ls ~/.claude/logs/

# Run in debug mode
RUST_LOG=debug claude-context-tracker monitor myproject
```

## Roadmap

- [ ] Settings dialog (database location, auto-start monitoring)
- [ ] Keyboard shortcuts (Ctrl+N for new project, Ctrl+F for search)
- [ ] Context menus (right-click actions)
- [ ] Desktop notifications (new facts, token thresholds)
- [ ] Export to PDF/HTML
- [ ] Flatpak packaging
- [ ] Search and filtering
- [ ] Dark mode toggle
- [ ] Multi-project monitoring

## License

MIT License - see [LICENSE](LICENSE) file for details

## Credits

Built with:
- [GTK4](https://gtk.org/) - Modern cross-platform UI toolkit
- [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/) - GNOME design patterns
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite bindings for Rust
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [notify](https://github.com/notify-rs/notify) - File system monitoring

Inspired by the need to maintain context across long Claude Code conversations.
