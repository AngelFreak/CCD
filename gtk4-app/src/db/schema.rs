/// Database schema for Claude Context Tracker
/// Matches the PocketBase collections structure

/// SQL for creating the projects table
pub const CREATE_PROJECTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    repo_path TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    priority INTEGER NOT NULL DEFAULT 0,
    tech_stack TEXT NOT NULL DEFAULT '[]',
    description TEXT,
    created TEXT NOT NULL,
    updated TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
CREATE INDEX IF NOT EXISTS idx_projects_updated ON projects(updated DESC);
"#;

/// SQL for creating the context_sections table
pub const CREATE_CONTEXT_SECTIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS context_sections (
    id TEXT PRIMARY KEY NOT NULL,
    project TEXT NOT NULL,
    section_type TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    "order" INTEGER NOT NULL DEFAULT 0,
    auto_extracted INTEGER NOT NULL DEFAULT 0,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    FOREIGN KEY (project) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_context_sections_project ON context_sections(project);
CREATE INDEX IF NOT EXISTS idx_context_sections_order ON context_sections("order");
"#;

/// SQL for creating the session_history table
pub const CREATE_SESSION_HISTORY_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS session_history (
    id TEXT PRIMARY KEY NOT NULL,
    project TEXT NOT NULL,
    summary TEXT NOT NULL,
    facts_extracted INTEGER NOT NULL DEFAULT 0,
    token_count INTEGER NOT NULL DEFAULT 0,
    session_start TEXT NOT NULL,
    session_end TEXT,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    FOREIGN KEY (project) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_session_history_project ON session_history(project);
CREATE INDEX IF NOT EXISTS idx_session_history_session_start ON session_history(session_start DESC);
"#;

/// SQL for creating the extracted_facts table
pub const CREATE_EXTRACTED_FACTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS extracted_facts (
    id TEXT PRIMARY KEY NOT NULL,
    project TEXT NOT NULL,
    session TEXT,
    fact_type TEXT NOT NULL,
    content TEXT NOT NULL,
    importance INTEGER NOT NULL DEFAULT 3,
    stale INTEGER NOT NULL DEFAULT 0,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    FOREIGN KEY (project) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (session) REFERENCES session_history(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_extracted_facts_project ON extracted_facts(project);
CREATE INDEX IF NOT EXISTS idx_extracted_facts_session ON extracted_facts(session);
CREATE INDEX IF NOT EXISTS idx_extracted_facts_importance ON extracted_facts(importance DESC);
CREATE INDEX IF NOT EXISTS idx_extracted_facts_type ON extracted_facts(fact_type);
CREATE INDEX IF NOT EXISTS idx_extracted_facts_stale ON extracted_facts(stale);
"#;

/// All table creation statements in order
pub const ALL_TABLES: &[&str] = &[
    CREATE_PROJECTS_TABLE,
    CREATE_CONTEXT_SECTIONS_TABLE,
    CREATE_SESSION_HISTORY_TABLE,
    CREATE_EXTRACTED_FACTS_TABLE,
];

/// Database version for migrations
pub const SCHEMA_VERSION: i32 = 1;

/// SQL for creating the schema_version table
pub const CREATE_VERSION_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY NOT NULL,
    applied_at TEXT NOT NULL
);
"#;
