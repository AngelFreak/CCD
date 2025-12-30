use crate::db::DbPool;
use crate::models::*;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use std::sync::Arc;
use uuid::Uuid;

/// Database repository for all CRUD operations
#[derive(Clone)]
pub struct Repository {
    pool: Arc<DbPool>,
}

impl Repository {
    /// Create a new repository
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    /// Get a database connection from the pool
    fn conn(&self) -> Result<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>> {
        self.pool.get().context("Failed to get database connection")
    }

    // ==================== PROJECT OPERATIONS ====================

    /// List all projects with optional status filter
    pub fn list_projects(&self, status_filter: Option<ProjectStatus>) -> Result<Vec<Project>> {
        let conn = self.conn()?;

        let (sql, params): (String, Vec<String>) = match status_filter {
            Some(status) => (
                "SELECT * FROM projects WHERE status = ? ORDER BY updated DESC".to_string(),
                vec![status.as_str().to_string()],
            ),
            None => (
                "SELECT * FROM projects ORDER BY updated DESC".to_string(),
                vec![],
            ),
        };

        let mut stmt = conn.prepare(&sql)?;
        let projects = stmt
            .query_map(rusqlite::params_from_iter(params), Self::project_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    /// Get a single project by ID
    pub fn get_project(&self, id: &str) -> Result<Project> {
        let conn = self.conn()?;
        let project = conn.query_row(
            "SELECT * FROM projects WHERE id = ?",
            params![id],
            Self::project_from_row,
        )?;
        Ok(project)
    }

    /// Create a new project
    pub fn create_project(&self, payload: ProjectPayload) -> Result<Project> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let tech_stack_json = serde_json::to_string(&payload.tech_stack)?;

        conn.execute(
            "INSERT INTO projects (id, name, slug, repo_path, status, priority, tech_stack, description, created, updated)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                id,
                payload.name,
                payload.slug,
                payload.repo_path,
                payload.status.as_str(),
                payload.priority,
                tech_stack_json,
                payload.description,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        self.get_project(&id)
    }

    /// Update a project
    pub fn update_project(&self, id: &str, payload: ProjectPayload) -> Result<Project> {
        let conn = self.conn()?;
        let now = Utc::now();
        let tech_stack_json = serde_json::to_string(&payload.tech_stack)?;

        conn.execute(
            "UPDATE projects SET name = ?, slug = ?, repo_path = ?, status = ?, priority = ?,
             tech_stack = ?, description = ?, updated = ? WHERE id = ?",
            params![
                payload.name,
                payload.slug,
                payload.repo_path,
                payload.status.as_str(),
                payload.priority,
                tech_stack_json,
                payload.description,
                now.to_rfc3339(),
                id,
            ],
        )?;

        self.get_project(id)
    }

    /// Delete a project
    pub fn delete_project(&self, id: &str) -> Result<()> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM projects WHERE id = ?", params![id])?;
        Ok(())
    }

    // ==================== CONTEXT SECTION OPERATIONS ====================

    /// List context sections for a project
    pub fn list_context_sections(&self, project_id: &str) -> Result<Vec<ContextSection>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM context_sections WHERE project = ? ORDER BY \"order\"",
        )?;
        let sections = stmt
            .query_map(params![project_id], Self::context_section_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sections)
    }

    /// Get a single context section by ID
    pub fn get_context_section(&self, id: &str) -> Result<ContextSection> {
        let conn = self.conn()?;
        let section = conn.query_row(
            "SELECT * FROM context_sections WHERE id = ?",
            params![id],
            Self::context_section_from_row,
        )?;
        Ok(section)
    }

    /// Create a new context section
    pub fn create_context_section(&self, payload: ContextSectionPayload) -> Result<ContextSection> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO context_sections (id, project, section_type, title, content, \"order\", auto_extracted, created, updated)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                id,
                payload.project,
                payload.section_type.as_str(),
                payload.title,
                payload.content,
                payload.order,
                payload.auto_extracted.unwrap_or(false) as i32,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        self.get_context_section(&id)
    }

    /// Update a context section
    pub fn update_context_section(&self, id: &str, payload: ContextSectionPayload) -> Result<ContextSection> {
        let conn = self.conn()?;
        let now = Utc::now();

        conn.execute(
            "UPDATE context_sections SET project = ?, section_type = ?, title = ?, content = ?,
             \"order\" = ?, auto_extracted = ?, updated = ? WHERE id = ?",
            params![
                payload.project,
                payload.section_type.as_str(),
                payload.title,
                payload.content,
                payload.order,
                payload.auto_extracted.unwrap_or(false) as i32,
                now.to_rfc3339(),
                id,
            ],
        )?;

        self.get_context_section(id)
    }

    /// Delete a context section
    pub fn delete_context_section(&self, id: &str) -> Result<()> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM context_sections WHERE id = ?", params![id])?;
        Ok(())
    }

    // ==================== SESSION HISTORY OPERATIONS ====================

    /// List session history for a project
    pub fn list_sessions(&self, project_id: &str) -> Result<Vec<SessionHistory>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM session_history WHERE project = ? ORDER BY session_start DESC",
        )?;
        let sessions = stmt
            .query_map(params![project_id], Self::session_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    /// Get a single session by ID
    pub fn get_session(&self, id: &str) -> Result<SessionHistory> {
        let conn = self.conn()?;
        let session = conn.query_row(
            "SELECT * FROM session_history WHERE id = ?",
            params![id],
            Self::session_from_row,
        )?;
        Ok(session)
    }

    /// Create a new session
    pub fn create_session(&self, payload: SessionPayload) -> Result<SessionHistory> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO session_history (id, project, summary, facts_extracted, token_count, session_start, session_end, created, updated)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                id,
                payload.project,
                payload.summary,
                payload.facts_extracted.unwrap_or(0),
                payload.token_count.unwrap_or(0),
                payload.session_start.unwrap_or(now).to_rfc3339(),
                payload.session_end.map(|t| t.to_rfc3339()),
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        self.get_session(&id)
    }

    /// Update a session
    pub fn update_session(&self, id: &str, payload: SessionPayload) -> Result<SessionHistory> {
        let conn = self.conn()?;
        let now = Utc::now();

        conn.execute(
            "UPDATE session_history SET project = ?, summary = ?, facts_extracted = ?, token_count = ?,
             session_start = ?, session_end = ?, updated = ? WHERE id = ?",
            params![
                payload.project,
                payload.summary,
                payload.facts_extracted.unwrap_or(0),
                payload.token_count.unwrap_or(0),
                payload.session_start.unwrap_or(now).to_rfc3339(),
                payload.session_end.map(|t| t.to_rfc3339()),
                now.to_rfc3339(),
                id,
            ],
        )?;

        self.get_session(id)
    }

    /// Delete a session
    pub fn delete_session(&self, id: &str) -> Result<()> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM session_history WHERE id = ?", params![id])?;
        Ok(())
    }

    // ==================== EXTRACTED FACTS OPERATIONS ====================

    /// List extracted facts for a project
    pub fn list_facts(&self, project_id: &str, include_stale: bool) -> Result<Vec<ExtractedFact>> {
        let conn = self.conn()?;

        let sql = if include_stale {
            "SELECT * FROM extracted_facts WHERE project = ? ORDER BY importance DESC, created DESC"
        } else {
            "SELECT * FROM extracted_facts WHERE project = ? AND stale = 0 ORDER BY importance DESC, created DESC"
        };

        let mut stmt = conn.prepare(sql)?;
        let facts = stmt
            .query_map(params![project_id], Self::fact_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(facts)
    }

    /// Get facts by type for a project
    pub fn list_facts_by_type(&self, project_id: &str, fact_type: FactType) -> Result<Vec<ExtractedFact>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM extracted_facts WHERE project = ? AND fact_type = ?
             ORDER BY importance DESC, created DESC",
        )?;
        let facts = stmt
            .query_map(params![project_id, fact_type.as_str()], Self::fact_from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(facts)
    }

    /// Get a single fact by ID
    pub fn get_fact(&self, id: &str) -> Result<ExtractedFact> {
        let conn = self.conn()?;
        let fact = conn.query_row(
            "SELECT * FROM extracted_facts WHERE id = ?",
            params![id],
            Self::fact_from_row,
        )?;
        Ok(fact)
    }

    /// Create a new fact
    pub fn create_fact(&self, payload: ExtractedFactPayload) -> Result<ExtractedFact> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO extracted_facts (id, project, session, fact_type, content, importance, stale, created, updated)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                id,
                payload.project,
                payload.session,
                payload.fact_type.as_str(),
                payload.content,
                payload.importance,
                payload.stale.unwrap_or(false) as i32,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        self.get_fact(&id)
    }

    /// Update a fact
    pub fn update_fact(&self, id: &str, payload: ExtractedFactPayload) -> Result<ExtractedFact> {
        let conn = self.conn()?;
        let now = Utc::now();

        conn.execute(
            "UPDATE extracted_facts SET project = ?, session = ?, fact_type = ?, content = ?,
             importance = ?, stale = ?, updated = ? WHERE id = ?",
            params![
                payload.project,
                payload.session,
                payload.fact_type.as_str(),
                payload.content,
                payload.importance,
                payload.stale.unwrap_or(false) as i32,
                now.to_rfc3339(),
                id,
            ],
        )?;

        self.get_fact(id)
    }

    /// Mark a fact as stale
    pub fn mark_fact_stale(&self, id: &str) -> Result<ExtractedFact> {
        let conn = self.conn()?;
        let now = Utc::now();

        conn.execute(
            "UPDATE extracted_facts SET stale = 1, updated = ? WHERE id = ?",
            params![now.to_rfc3339(), id],
        )?;

        self.get_fact(id)
    }

    /// Delete a fact
    pub fn delete_fact(&self, id: &str) -> Result<()> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM extracted_facts WHERE id = ?", params![id])?;
        Ok(())
    }

    // ==================== ROW MAPPING FUNCTIONS ====================

    fn project_from_row(row: &Row) -> rusqlite::Result<Project> {
        let tech_stack_json: String = row.get(6)?;
        let tech_stack: Vec<String> = serde_json::from_str(&tech_stack_json).unwrap_or_default();

        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            slug: row.get(2)?,
            repo_path: row.get(3)?,
            status: ProjectStatus::from_str(&row.get::<_, String>(4)?),
            priority: row.get(5)?,
            tech_stack,
            description: row.get(7)?,
            created: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        })
    }

    fn context_section_from_row(row: &Row) -> rusqlite::Result<ContextSection> {
        Ok(ContextSection {
            id: row.get(0)?,
            project: row.get(1)?,
            section_type: SectionType::from_str(&row.get::<_, String>(2)?),
            title: row.get(3)?,
            content: row.get(4)?,
            order: row.get(5)?,
            auto_extracted: row.get::<_, i32>(6)? != 0,
            created: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        })
    }

    fn session_from_row(row: &Row) -> rusqlite::Result<SessionHistory> {
        let session_end_str: Option<String> = row.get(6)?;
        let session_end = session_end_str
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Ok(SessionHistory {
            id: row.get(0)?,
            project: row.get(1)?,
            summary: row.get(2)?,
            facts_extracted: row.get(3)?,
            token_count: row.get(4)?,
            session_start: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            session_end,
            created: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        })
    }

    fn fact_from_row(row: &Row) -> rusqlite::Result<ExtractedFact> {
        Ok(ExtractedFact {
            id: row.get(0)?,
            project: row.get(1)?,
            session: row.get(2)?,
            fact_type: FactType::from_str(&row.get::<_, String>(3)?),
            content: row.get(4)?,
            importance: row.get(5)?,
            stale: row.get::<_, i32>(6)? != 0,
            created: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        })
    }
}

// Helper trait for parsing enums from strings
trait FromStr: Sized {
    fn from_str(s: &str) -> Self;
}

impl FromStr for ProjectStatus {
    fn from_str(s: &str) -> Self {
        match s {
            "active" => ProjectStatus::Active,
            "paused" => ProjectStatus::Paused,
            "idea" => ProjectStatus::Idea,
            "archived" => ProjectStatus::Archived,
            _ => ProjectStatus::Active,
        }
    }
}

impl FromStr for SectionType {
    fn from_str(s: &str) -> Self {
        match s {
            "architecture" => SectionType::Architecture,
            "current_state" => SectionType::CurrentState,
            "next_steps" => SectionType::NextSteps,
            "gotchas" => SectionType::Gotchas,
            "decisions" => SectionType::Decisions,
            _ => SectionType::Custom,
        }
    }
}

impl FromStr for FactType {
    fn from_str(s: &str) -> Self {
        match s {
            "decision" => FactType::Decision,
            "blocker" => FactType::Blocker,
            "file_change" => FactType::FileChange,
            "dependency" => FactType::Dependency,
            "todo" => FactType::Todo,
            _ => FactType::Insight,
        }
    }
}
