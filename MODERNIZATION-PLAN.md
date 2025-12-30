# Claude Context Tracker - Modernization Plan

## Status: Ready for Implementation

This document outlines the complete modernization of the Claude Context Tracker to be a polished, production-ready native application.

## Changes Required

### 1. Remove All Async PocketBase Code

**Files to Update:**
- `src/views/dashboard.rs` - Replace async HTTP with sync Repository
- `src/views/project_detail.rs` - Replace async HTTP with sync Repository
- `src/views/context_editor.rs` - Replace async HTTP with sync Repository
- `src/views/facts_list.rs` - Replace async HTTP with sync Repository
- `src/views/session_monitor.rs` - Replace async HTTP with sync Repository

**Pattern:**
```rust
// OLD (Async):
glib::spawn_future_local(async move {
    match pb_client.list_projects(filter).await {
        Ok(projects) => update_ui(projects),
        Err(e) => show_error(e),
    }
});

// NEW (Sync):
match repository.list_projects(filter) {
    Ok(projects) => {
        *self.projects.borrow_mut() = projects.clone();
        Self::update_ui(&projects);
    }
    Err(e) => {
        log::error!("Error: {}", e);
        Self::show_error(&e.to_string());
    }
}
```

### 2. Add Background Monitoring Toggle

**Location:** Main window header bar

**Features:**
- Toggle switch in header bar
- Start/stop background monitoring for active project
- Visual indicator when monitoring is active
- Auto-start option in preferences
- Monitor status in status bar

**Implementation:**
```rust
// In window.rs
let monitor_toggle = gtk::Switch::new();
monitor_toggle.set_tooltip_text(Some("Background Monitoring"));

monitor_toggle.connect_state_set(move |switch, enabled| {
    if enabled {
        start_background_monitor(project_id, repository);
    } else {
        stop_background_monitor();
    }
    glib::Propagation::Proceed
});
```

### 3. UI Modernization (Claude App Style)

#### Color Scheme
```css
/* Claude-inspired colors */
@define-color claude_orange #D97757;
@define-color claude_beige #F5F1ED;
@define-color claude_dark #2C2416;
@define-color claude_accent #CC785C;
```

#### Typography
- Larger, cleaner fonts
- Better hierarchy (title-1, title-2, title-3)
- Improved readability with proper line-height

#### Layout Improvements
- **Sidebar Navigation** - Project switcher in left sidebar
- **Content Area** - Cleaner, more spacious
- **Cards** - Rounded corners, subtle shadows
- **Empty States** - Friendly illustrations and messages
- **Loading States** - Smooth skeleton screens

#### Component Updates

**Header Bar:**
```
[≡ Menu] [Project Name] [Monitoring: ●OFF] [New Project] [⟳]
```

**Dashboard:**
- Grid layout instead of list (when space allows)
- Larger project cards with preview
- Quick actions on hover
- Recent activity indicator

**Project Detail:**
- Split view (navigation + content)
- Breadcrumb navigation
- Floating action buttons for common tasks
- Inline editing

**Facts List:**
- Group by type with collapsible sections
- Color-coded fact types
- Quick filters (blocker, todo, etc.)
- Search/filter bar

**Session Monitor:**
- Radial progress indicator for tokens
- Timeline view of sessions
- Quick session comparison

###4. GitHub Actions for .deb Releases

**File:** `.github/workflows/release.yml`

```yaml
name: Release .deb Package

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-deb:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-4-dev libadwaita-1-dev build-essential

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Install cargo-deb
        run: cargo install cargo-deb

      - name: Build .deb
        run: |
          cd gtk4-app
          cargo deb

      - name: Upload Release
        uses: softprops/action-gh-release@v1
        with:
          files: gtk4-app/target/debian/*.deb
```

**Cargo.toml** additions:
```toml
[package.metadata.deb]
maintainer = "Claude Context Tracker Contributors"
copyright = "2024, MIT License"
license-file = ["../LICENSE", "0"]
extended-description = "Native GTK4 application for managing Claude Code context"
depends = "$auto, libgtk-4-1, libadwaita-1-0"
section = "devel"
priority = "optional"
assets = [
    ["target/release/claude-context-tracker", "usr/bin/", "755"],
    ["resources/com.github.claudecontexttracker.desktop", "usr/share/applications/", "644"],
]
```

### 5. Additional Improvements

#### Settings/Preferences Dialog
- Database location
- Auto-start monitoring
- Claude Code logs directory
- Theme (system, light, dark)
- Token warning threshold

#### Keyboard Shortcuts
- `Ctrl+N` - New project
- `Ctrl+F` - Search/filter
- `Ctrl+,` - Preferences
- `Ctrl+Q` - Quit
- `F5` - Refresh

#### Context Menus
- Right-click on projects for quick actions
- Right-click on facts to edit/delete
- Right-click on sessions for export

#### Notifications
- Desktop notifications for:
  - New facts extracted
  - Token threshold reached
  - Monitoring status changes

#### Export Options
- Export to PDF
- Export to HTML
- Export as JSON (data portability)

### 6. Polish & Performance

#### Performance
- Lazy load project details
- Virtual scrolling for large fact lists
- Debounce search inputs
- Cache rendered markdown

#### Accessibility
- Proper ARIA labels
- Keyboard navigation throughout
- High contrast mode support
- Screen reader compatibility

#### Error Handling
- Graceful degradation
- Helpful error messages
- Recovery suggestions
- Error reporting dialog

## Implementation Priority

### Phase 1: Core Functionality (This Commit)
1. ✅ Remove async PocketBase code
2. ✅ Update all views to use Repository
3. ✅ Add monitoring toggle
4. ✅ Basic UI improvements

### Phase 2: Polish (Next Commit)
1. Settings dialog
2. Keyboard shortcuts
3. Context menus
4. Notifications

### Phase 3: Packaging (Final Commit)
1. GitHub Actions workflow
2. .deb package metadata
3. Desktop file improvements
4. Icon set

## Testing Checklist

- [ ] Dashboard loads projects correctly
- [ ] Project detail shows all data
- [ ] Context editor saves changes
- [ ] Facts list displays correctly
- [ ] Session monitor updates
- [ ] Monitoring toggle works
- [ ] No async PocketBase calls remain
- [ ] No compilation errors
- [ ] UI looks polished and native
- [ ] GitHub Actions builds .deb successfully

## Success Criteria

✅ Single codebase in Rust
✅ No external dependencies
✅ Native GTK4 UI
✅ Embedded SQLite database
✅ Background monitoring
✅ CLI commands
✅ Polished, Claude-like UI
✅ Automated .deb releases
✅ Production-ready quality

---

**Next Steps:** Implement Phase 1 changes in a single comprehensive commit.
