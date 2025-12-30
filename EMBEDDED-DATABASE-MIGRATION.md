# Embedded Database Migration

## Overview

The Claude Context Tracker GTK4 application has been migrated from using an external PocketBase server to an **embedded SQLite database**. This makes the application truly standalone with no external dependencies.

## What Changed

### Architecture

**Before:**
- Web/Desktop UI → HTTP REST API → PocketBase Server → SQLite Database
- Required PocketBase server running separately
- Async HTTP requests via reqwest

**After:**
- Desktop UI → Rust Repository → Embedded SQLite Database
- Self-contained, single executable
- Direct synchronous database operations

### Dependencies Replaced

**Removed:**
- `reqwest` - HTTP client
- `tokio` - Async runtime (was only needed for HTTP)
- `futures` - Async utilities
- `urlencoding` - URL encoding

**Added:**
- `rusqlite` - SQLite database bindings (with bundled SQLite)
- `r2d2` - Connection pooling
- `r2d2_sqlite` - SQLite connection manager
- `dirs` - XDG directory support
- `uuid` - ID generation

### File Structure

```
gtk4-app/src/
├── db/
│   ├── mod.rs              # Database module exports
│   ├── schema.rs           # Table definitions and migrations
│   ├── connection.rs       # Database initialization and pooling
│   └── repository.rs       # CRUD operations for all models
├── models/                 # Unchanged - same data models
├── views/                  # Need updates for sync operations
├── main.rs                 # Updated to use Database + Repository
└── window.rs               # Updated to use Repository
```

## Database Schema

The embedded database exactly matches the PocketBase schema:

### Tables

1. **projects**
   - `id`, `name`, `slug`, `repo_path`, `status`, `priority`
   - `tech_stack` (JSON array), `description`
   - `created`, `updated` (RFC3339 timestamps)

2. **context_sections**
   - `id`, `project` (FK), `section_type`, `title`, `content`
   - `order`, `auto_extracted`
   - `created`, `updated`

3. **session_history**
   - `id`, `project` (FK), `summary`
   - `facts_extracted`, `token_count`
   - `session_start`, `session_end`
   - `created`, `updated`

4. **extracted_facts**
   - `id`, `project` (FK), `session` (FK nullable)
   - `fact_type`, `content`, `importance`, `stale`
   - `created`, `updated`

5. **schema_version**
   - `version`, `applied_at`
   - Tracks database migrations

### Features

- ✅ Foreign key constraints with CASCADE delete
- ✅ Indexes on commonly queried columns
- ✅ Automatic schema initialization
- ✅ Migration system for future updates

## Database Location

The database is stored following XDG Base Directory Specification:

**Linux:**
```
~/.local/share/claude-context-tracker/tracker.db
```

**macOS:**
```
~/Library/Application Support/claude-context-tracker/tracker.db
```

**Windows:**
```
%APPDATA%\claude-context-tracker\tracker.db
```

Override with custom path:
```rust
Database::new(Some(PathBuf::from("/custom/path/tracker.db")))
```

## Repository API

The `Repository` struct provides all CRUD operations:

```rust
let repository = Repository::new(pool);

// Projects
repository.list_projects(Some(ProjectStatus::Active))?;
repository.get_project(id)?;
repository.create_project(payload)?;
repository.update_project(id, payload)?;
repository.delete_project(id)?;

// Context Sections
repository.list_context_sections(project_id)?;
repository.create_context_section(payload)?;
// ... etc

// Session History
repository.list_sessions(project_id)?;
// ... etc

// Extracted Facts
repository.list_facts(project_id, include_stale)?;
repository.list_facts_by_type(project_id, FactType::Decision)?;
repository.mark_fact_stale(id)?;
// ... etc
```

All operations are **synchronous** and return `Result<T, anyhow::Error>`.

## Connection Pooling

Uses `r2d2` connection pool with:
- Max 5 concurrent connections
- Thread-safe `Arc<DbPool>` sharing
- Automatic connection recycling
- Foreign key constraints enabled

## Migration Status

### ✅ Completed

- [x] Database schema creation
- [x] Connection management and pooling
- [x] Repository with full CRUD operations
- [x] UUID-based ID generation
- [x] XDG directory support
- [x] Schema version tracking
- [x] Foreign key constraints
- [x] Indexes for performance
- [x] Updated `main.rs` to use embedded database
- [x] Updated `window.rs` to use Repository

### 🔄 In Progress

- [ ] Update `views/dashboard.rs` for sync operations
- [ ] Update `views/project_detail.rs` for sync operations
- [ ] Update `views/context_editor.rs` for sync operations
- [ ] Update `views/facts_list.rs` for sync operations
- [ ] Update `views/session_monitor.rs` for sync operations

### View Update Pattern

**Old (Async with PocketBase):**
```rust
glib::spawn_future_local(async move {
    match pb_client.list_projects(filter).await {
        Ok(projects) => update_ui(projects),
        Err(e) => show_error(e),
    }
});
```

**New (Sync with Repository):**
```rust
match repository.list_projects(filter) {
    Ok(projects) => {
        *self.projects.borrow_mut() = projects.clone();
        Self::update_project_list(&self.project_list, &projects);
    }
    Err(e) => {
        log::error!("Failed to load projects: {}", e);
        Self::show_error_state(&self.project_list, &e.to_string());
    }
}
```

## Benefits

### 1. **No External Dependencies**
- Single executable, no server required
- Works offline by default
- Faster startup (no network checks)

### 2. **Simpler Architecture**
- Removed async complexity where not needed
- Direct database access
- Easier to debug and maintain

### 3. **Better Performance**
- No network latency
- No HTTP serialization overhead
- Connection pooling for concurrent access

### 4. **Data Portability**
- Single `.db` file
- Easy backup (just copy the file)
- SQLite format is universal

### 5. **Privacy**
- All data stored locally
- No network communication
- Complete user control

## Testing

The database module includes comprehensive tests:

```bash
cd gtk4-app
cargo test db::
```

Tests cover:
- Schema initialization
- Version tracking
- Table creation
- Connection pooling

## Migration from PocketBase

If you have existing data in PocketBase, you can export and import:

1. **Export from PocketBase:**
   ```bash
   # PocketBase admin UI → Collections → Export JSON
   ```

2. **Import to SQLite:**
   ```rust
   // Custom script to read JSON and insert via Repository
   let projects: Vec<Project> = serde_json::from_str(&json)?;
   for project in projects {
       repository.create_project(ProjectPayload::from(&project))?;
   }
   ```

## Compatibility Notes

### Go Daemon

The Go daemon currently writes to PocketBase. To work with the embedded database:

**Option 1:** Keep daemon writing to PocketBase, periodically sync to SQLite
**Option 2:** Update daemon to write directly to SQLite (requires Rust rewrite or CGo)
**Option 3:** Remove daemon, implement monitoring in Rust within the GTK app

**Recommendation:** Option 3 - Implement monitoring as a background thread in the GTK app.

### CLI Tool

Similar situation as daemon. Options:

**Option 1:** Rewrite CLI in Rust, share database code
**Option 2:** Keep Go CLI, add SQLite support via Go's `database/sql`
**Option 3:** Integrate CLI functionality into GTK app as menu actions

**Recommendation:** Option 1 - Single Rust codebase for GUI and CLI.

## Future Enhancements

- [ ] Background vacuum/optimize on startup
- [ ] Database backup functionality in UI
- [ ] Import/Export JSON for data portability
- [ ] Full-text search using FTS5
- [ ] Statistics and analytics queries
- [ ] Implement daemon monitoring in Rust
- [ ] Shared Rust library for CLI tool

## Building

The application now builds faster without reqwest/tokio:

```bash
cd gtk4-app

# Debug build
cargo build

# Release build (faster, smaller)
cargo build --release

# Run
cargo run
```

## Database File Management

### Backup

```bash
cp ~/.local/share/claude-context-tracker/tracker.db ~/backups/tracker-$(date +%Y%m%d).db
```

### Reset

```bash
rm ~/.local/share/claude-context-tracker/tracker.db
# App will create fresh database on next launch
```

### View with SQLite CLI

```bash
sqlite3 ~/.local/share/claude-context-tracker/tracker.db
.schema
SELECT * FROM projects;
```

## Conclusion

The migration to an embedded database makes the Claude Context Tracker a truly native, standalone desktop application with no external service dependencies. The application is simpler, faster, and more private while maintaining full compatibility with the existing data model.

---

**Status:** Core database infrastructure complete. Views need updates for synchronous operations.
**Next Steps:** Update all view components to remove async HTTP and use Repository directly.
