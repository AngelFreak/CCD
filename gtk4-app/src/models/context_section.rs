use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Section type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionType {
    Architecture,
    CurrentState,
    NextSteps,
    Gotchas,
    Decisions,
    Custom,
}

impl SectionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Architecture => "architecture",
            Self::CurrentState => "current_state",
            Self::NextSteps => "next_steps",
            Self::Gotchas => "gotchas",
            Self::Decisions => "decisions",
            Self::Custom => "custom",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Architecture => "Architecture",
            Self::CurrentState => "Current State",
            Self::NextSteps => "Next Steps",
            Self::Gotchas => "Gotchas",
            Self::Decisions => "Decisions",
            Self::Custom => "Custom",
        }
    }

    pub fn icon_name(&self) -> &str {
        match self {
            Self::Architecture => "builder-build-symbolic",
            Self::CurrentState => "view-list-symbolic",
            Self::NextSteps => "go-next-symbolic",
            Self::Gotchas => "dialog-warning-symbolic",
            Self::Decisions => "emblem-ok-symbolic",
            Self::Custom => "text-editor-symbolic",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Architecture,
            Self::CurrentState,
            Self::NextSteps,
            Self::Gotchas,
            Self::Decisions,
            Self::Custom,
        ]
    }
}

impl Default for SectionType {
    fn default() -> Self {
        Self::Custom
    }
}

impl std::fmt::Display for SectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Context section model representing structured project context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSection {
    pub id: String,
    pub project: String, // Project ID
    pub section_type: SectionType,
    pub title: String,
    pub content: String,
    pub order: i32,
    pub auto_extracted: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ContextSection {
    /// Create a new context section
    pub fn new(project_id: String, section_type: SectionType, title: String) -> Self {
        Self {
            id: String::new(), // Will be set by PocketBase
            project: project_id,
            section_type,
            title,
            content: String::new(),
            order: 0,
            auto_extracted: false,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    /// Get markdown representation
    pub fn to_markdown(&self) -> String {
        format!("## {}\n\n{}\n\n", self.title, self.content)
    }

    /// Get a preview of the content (first 100 chars)
    pub fn content_preview(&self) -> String {
        if self.content.len() <= 100 {
            self.content.clone()
        } else {
            format!("{}...", &self.content[..97])
        }
    }
}

/// Request payload for creating/updating context sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSectionPayload {
    pub project: String,
    pub section_type: SectionType,
    pub title: String,
    pub content: String,
    pub order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_extracted: Option<bool>,
}

impl From<&ContextSection> for ContextSectionPayload {
    fn from(section: &ContextSection) -> Self {
        Self {
            project: section.project.clone(),
            section_type: section.section_type,
            title: section.title.clone(),
            content: section.content.clone(),
            order: section.order,
            auto_extracted: Some(section.auto_extracted),
        }
    }
}
