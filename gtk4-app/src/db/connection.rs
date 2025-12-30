use crate::db::schema;
use anyhow::{Context, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Connection pool type
pub type DbPool = Pool<SqliteConnectionManager>;

/// Shared database pool
pub type SharedDbPool = Arc<DbPool>;

/// Database manager for Claude Context Tracker
pub struct Database {
    pool: DbPool,
    db_path: PathBuf,
}

impl Database {
    /// Create a new database connection
    ///
    /// If db_path is None, uses XDG data directory
    pub fn new(db_path: Option<PathBuf>) -> Result<Self> {
        let path = db_path.unwrap_or_else(Self::default_db_path);

        log::info!("Opening database at: {}", path.display());

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create database directory")?;
        }

        // Create connection pool
        let manager = SqliteConnectionManager::file(&path);
        let pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .context("Failed to create connection pool")?;

        let db = Self {
            pool,
            db_path: path,
        };

        // Initialize schema
        db.initialize_schema()?;

        Ok(db)
    }

    /// Get the default database path using XDG directories
    fn default_db_path() -> PathBuf {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("claude-context-tracker");

        std::fs::create_dir_all(&data_dir).ok();

        data_dir.join("tracker.db")
    }

    /// Get the database file path
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    /// Get a connection from the pool
    pub fn get_connection(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.pool.get().context("Failed to get database connection")
    }

    /// Get the connection pool
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    /// Initialize the database schema
    fn initialize_schema(&self) -> Result<()> {
        let conn = self.get_connection()?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        // Create version table
        conn.execute_batch(schema::CREATE_VERSION_TABLE)?;

        // Check current version
        let current_version: Option<i32> = conn
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .ok();

        match current_version {
            Some(version) if version >= schema::SCHEMA_VERSION => {
                log::info!("Database schema is up to date (version {})", version);
                return Ok(());
            }
            Some(version) => {
                log::info!(
                    "Migrating database from version {} to {}",
                    version,
                    schema::SCHEMA_VERSION
                );
                // Migrations would go here
            }
            None => {
                log::info!("Initializing database schema (version {})", schema::SCHEMA_VERSION);
            }
        }

        // Create all tables
        for table_sql in schema::ALL_TABLES {
            conn.execute_batch(table_sql)
                .context("Failed to create table")?;
        }

        // Record schema version
        conn.execute(
            "INSERT INTO schema_version (version, applied_at) VALUES (?, datetime('now'))",
            [schema::SCHEMA_VERSION],
        )?;

        log::info!("Database schema initialized successfully");

        Ok(())
    }

    /// Create a shared database pool
    pub fn into_shared(self) -> SharedDbPool {
        Arc::new(self.pool)
    }
}

/// Create a new in-memory database for testing
#[cfg(test)]
pub fn create_test_db() -> Result<Database> {
    let manager = SqliteConnectionManager::memory();
    let pool = Pool::builder()
        .max_size(1)
        .build(manager)
        .context("Failed to create test connection pool")?;

    let db = Database {
        pool,
        db_path: PathBuf::from(":memory:"),
    };

    db.initialize_schema()?;

    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_in_memory_db() {
        let db = create_test_db().expect("Failed to create test database");
        let conn = db.get_connection().expect("Failed to get connection");

        // Verify tables exist
        let table_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('projects', 'context_sections', 'session_history', 'extracted_facts')",
                [],
                |row| row.get(0),
            )
            .expect("Failed to count tables");

        assert_eq!(table_count, 4, "All tables should be created");
    }

    #[test]
    fn test_schema_version() {
        let db = create_test_db().expect("Failed to create test database");
        let conn = db.get_connection().expect("Failed to get connection");

        let version: i32 = conn
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .expect("Failed to get schema version");

        assert_eq!(version, schema::SCHEMA_VERSION);
    }
}
