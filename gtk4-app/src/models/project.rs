use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Project status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Active,
    Paused,
    Idea,
    Archived,
}

impl ProjectStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Idea => "idea",
            Self::Archived => "archived",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Active => "Active",
            Self::Paused => "Paused",
            Self::Idea => "Idea",
            Self::Archived => "Archived",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Active, Self::Paused, Self::Idea, Self::Archived]
    }
}

impl Default for ProjectStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Project model representing a development project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub repo_path: Option<String>,
    pub status: ProjectStatus,
    pub priority: i32,
    pub tech_stack: Vec<String>,
    pub description: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Project {
    /// Create a new project with defaults
    pub fn new(name: String) -> Self {
        let slug = name.to_lowercase().replace(' ', "-");
        Self {
            id: String::new(), // Will be set by PocketBase
            name,
            slug,
            repo_path: None,
            status: ProjectStatus::Active,
            priority: 0,
            tech_stack: Vec::new(),
            description: None,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    /// Get a display string for tech stack
    pub fn tech_stack_display(&self) -> String {
        if self.tech_stack.is_empty() {
            String::from("No tech stack specified")
        } else {
            self.tech_stack.join(", ")
        }
    }

    /// Get status badge color class
    pub fn status_color(&self) -> &str {
        match self.status {
            ProjectStatus::Active => "success",
            ProjectStatus::Paused => "warning",
            ProjectStatus::Idea => "accent",
            ProjectStatus::Archived => "error",
        }
    }
}

/// Request payload for creating/updating projects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPayload {
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_path: Option<String>,
    pub status: ProjectStatus,
    pub priority: i32,
    pub tech_stack: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<&Project> for ProjectPayload {
    fn from(project: &Project) -> Self {
        Self {
            name: project.name.clone(),
            slug: project.slug.clone(),
            repo_path: project.repo_path.clone(),
            status: project.status,
            priority: project.priority,
            tech_stack: project.tech_stack.clone(),
            description: project.description.clone(),
        }
    }
}
