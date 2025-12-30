# Claude Context Tracker - GTK4 Native Application

A native GTK4 desktop application for managing Claude Code context across development projects. Built with Rust, GTK4, and libadwaita for a modern GNOME experience.

## Features

- **Project Management**: Organize multiple development projects with status tracking
- **Context Sections**: Structured sections for Architecture, Current State, Next Steps, Gotchas, and Decisions
- **Smart Facts**: Auto-extracted facts from Claude Code sessions with importance scoring
- **Session Monitoring**: Real-time token usage tracking and session history
- **Context Compression**: Keep top facts by type to prevent context window overflow
- **Markdown Export**: Export project context to CLAUDE.md files
- **Native GNOME Integration**: Follows GNOME Human Interface Guidelines with libadwaita

## Architecture

### Technology Stack

- **Language**: Rust (Edition 2021)
- **UI Framework**: GTK4 + libadwaita
- **Async Runtime**: Tokio
- **HTTP Client**: reqwest (for PocketBase API)
- **Serialization**: serde + serde_json
- **Date/Time**: chrono

### Project Structure

```
gtk4-app/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── window.rs               # Main window & navigation
│   ├── models/                 # Data models
│   │   ├── project.rs          # Project model
│   │   ├── context_section.rs  # Context sections
│   │   ├── session.rs          # Session history
│   │   └── fact.rs             # Extracted facts
│   ├── api/
│   │   └── pocketbase.rs       # PocketBase REST API client
│   ├── views/                  # UI views
│   │   ├── dashboard.rs        # Project list dashboard
│   │   ├── project_detail.rs   # Project detail with tabs
│   │   ├── context_editor.rs   # Context sections editor
│   │   ├── facts_list.rs       # Facts sidebar
│   │   └── session_monitor.rs  # Token usage monitor
│   └── utils/
│       └── markdown.rs         # Markdown export utilities
├── resources/
│   ├── style.css               # GTK CSS styling
│   └── *.desktop               # Desktop entry file
├── Cargo.toml                  # Rust dependencies
└── build.rs                    # Build configuration
```

## Prerequisites

### System Dependencies

#### Ubuntu/Debian
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev build-essential
```

#### Fedora
```bash
sudo dnf install gtk4-devel libadwaita-devel
```

#### Arch Linux
```bash
sudo pacman -S gtk4 libadwaita
```

### Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

### PocketBase Backend

The GTK4 app requires a running PocketBase instance. See the main project README for PocketBase setup.

## Building

### Debug Build

```bash
cd gtk4-app
cargo build
```

### Release Build (Optimized)

```bash
cargo build --release
```

The release build is optimized with LTO and stripped symbols for minimal binary size.

## Running

### Development

```bash
# Set PocketBase URL (optional, defaults to localhost:8090)
export POCKETBASE_URL=http://localhost:8090

# Run the application
cargo run
```

### Production

```bash
# Install to system
cargo install --path .

# Run installed binary
claude-context-tracker
```

## Configuration

### Environment Variables

- `POCKETBASE_URL`: PocketBase server URL (default: `http://localhost:8090`)
- `RUST_LOG`: Logging level (default: `info`, options: `debug`, `warn`, `error`)

### Logging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Specific module logging
RUST_LOG=claude_context_tracker=debug,gtk=warn cargo run
```

## Web to GTK4 Mapping

This application is a native conversion of the original React web app. Here's how web components map to GTK4:

| Web Component | GTK4 Widget | Notes |
|---------------|-------------|-------|
| React Router | AdwNavigationView | Stack-based navigation |
| Dashboard (page) | DashboardView | ListBox with AdwActionRow |
| ProjectCard | AdwActionRow | Status badge, tech stack |
| Tabs | AdwTabView + AdwTabBar | Context/Sessions/Compressed |
| Input fields | gtk::Entry | Text input |
| Textarea | gtk::TextView | Multi-line text |
| Select dropdown | gtk::DropDown | Status/type selection |
| Button | gtk::Button | With icon support |
| Progress bar | gtk::ProgressBar | Token usage |
| Toast notifications | adw::Toast | User feedback |
| Modal dialogs | adw::AlertDialog | Confirmations |
| Sidebar | gtk::Box (sidebar CSS) | Facts & session info |

## UI/UX Features

### Dashboard
- Filter projects by status (Active, Paused, Idea, Archived)
- Click project to view details
- Create new projects with form dialog
- Status badges with color coding

### Project Detail
- Tabbed interface for Context, Sessions, and Compressed views
- Right sidebar with Session Monitor and Facts List
- Export to CLAUDE.md
- Copy context to clipboard

### Context Editor
- List of context sections by type
- Add/edit/delete sections
- Markdown preview
- Reorder sections (drag-and-drop in future)

### Facts List
- Top 10 most important facts
- Importance stars (1-5)
- Fact type badges
- Age indicators
- Stale fact styling

### Session Monitor
- Token usage progress bar (0-200K)
- Session duration
- Facts extracted count
- Warning at 85% token usage

## Styling

Custom CSS is in `resources/style.css` with:
- Status badges (active, paused, idea, archived)
- Fact type badges (decision, blocker, todo, etc.)
- Project cards with hover effects
- Token usage progress bar states
- Empty states and loading spinners

All styling follows libadwaita color scheme for automatic dark mode support.

## Integration with Existing Components

### PocketBase Backend
- Uses the same PocketBase instance as the web app
- Same collections: `projects`, `context_sections`, `session_history`, `extracted_facts`
- REST API client implemented in Rust

### Go Daemon
- No changes needed - daemon runs independently
- Monitors Claude Code logs and updates PocketBase
- GTK4 app reads from PocketBase in real-time

### CLI Tool
- Can run alongside GTK4 app
- Both use the same backend
- CLI for automation, GUI for interactive use

## Future Enhancements

- [ ] Real-time updates via WebSocket subscriptions
- [ ] Drag-and-drop section reordering
- [ ] Rich text editing with syntax highlighting
- [ ] Search and filter within facts
- [ ] Export to multiple formats (PDF, HTML)
- [ ] Keyboard shortcuts for all actions
- [ ] Diff viewer for session changes
- [ ] Compressed context view implementation
- [ ] Project templates
- [ ] Settings panel for configuration

## Packaging

### Flatpak (Recommended for GNOME)

Create a Flatpak manifest to distribute on Flathub:

```yaml
# com.github.claudecontexttracker.yml
id: com.github.claudecontexttracker
runtime: org.gnome.Platform
runtime-version: '45'
sdk: org.gnome.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable
command: claude-context-tracker
finish-args:
  - --share=network
  - --socket=wayland
  - --socket=fallback-x11
  - --filesystem=home
modules:
  - name: claude-context-tracker
    buildsystem: simple
    build-commands:
      - cargo --offline fetch --manifest-path Cargo.toml
      - cargo --offline build --release
      - install -Dm755 target/release/claude-context-tracker -t /app/bin/
```

### .deb Package

Use `cargo-deb`:

```bash
cargo install cargo-deb
cargo deb
```

### AppImage

Use `cargo-appimage`:

```bash
cargo install cargo-appimage
cargo appimage
```

## Development

### Code Style

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Tests
cargo test
```

### GTK4 Development

- Use GTK Inspector for UI debugging: `Ctrl+Shift+D` while app is running
- Check GTK warnings in terminal output
- Use `RUST_LOG=debug` for detailed logging

## Troubleshooting

### PocketBase Connection Error

```
Error: Failed to initialize PocketBase client
```

**Solution**: Ensure PocketBase is running on `http://localhost:8090` or set `POCKETBASE_URL` environment variable.

### GTK Build Errors

```
error: failed to run custom build command for `gtk4-sys`
```

**Solution**: Install GTK4 development libraries (see Prerequisites).

### Missing libadwaita

```
Package libadwaita-1 was not found
```

**Solution**: Install libadwaita development package for your distribution.

## Contributing

This is the native GTK4 port of the Claude Context Tracker. Contributions welcome!

1. Follow Rust naming conventions
2. Use clippy recommendations
3. Add tests for new functionality
4. Update this README for new features

## License

MIT License - See main project LICENSE file

## Credits

- Original web app: React + TypeScript + Tailwind CSS
- GTK4 conversion: Rust + gtk4-rs + libadwaita
- Backend: PocketBase (unchanged)
- Daemon: Go (unchanged)
