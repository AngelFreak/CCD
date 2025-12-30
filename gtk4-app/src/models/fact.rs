use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Fact type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactType {
    Decision,
    Blocker,
    FileChange,
    Dependency,
    Todo,
    Insight,
}

impl FactType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Decision => "decision",
            Self::Blocker => "blocker",
            Self::FileChange => "file_change",
            Self::Dependency => "dependency",
            Self::Todo => "todo",
            Self::Insight => "insight",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Decision => "Decision",
            Self::Blocker => "Blocker",
            Self::FileChange => "File Change",
            Self::Dependency => "Dependency",
            Self::Todo => "Todo",
            Self::Insight => "Insight",
        }
    }

    pub fn icon_name(&self) -> &str {
        match self {
            Self::Decision => "emblem-ok-symbolic",
            Self::Blocker => "dialog-error-symbolic",
            Self::FileChange => "document-edit-symbolic",
            Self::Dependency => "package-x-generic-symbolic",
            Self::Todo => "checkbox-symbolic",
            Self::Insight => "dialog-information-symbolic",
        }
    }

    pub fn color_class(&self) -> &str {
        match self {
            Self::Decision => "success",
            Self::Blocker => "error",
            Self::FileChange => "accent",
            Self::Dependency => "warning",
            Self::Todo => "default",
            Self::Insight => "accent",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Decision,
            Self::Blocker,
            Self::FileChange,
            Self::Dependency,
            Self::Todo,
            Self::Insight,
        ]
    }
}

impl Default for FactType {
    fn default() -> Self {
        Self::Insight
    }
}

impl std::fmt::Display for FactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Extracted fact model representing auto-extracted knowledge from sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFact {
    pub id: String,
    pub project: String, // Project ID
    pub session: Option<String>, // Session ID (optional)
    pub fact_type: FactType,
    pub content: String,
    pub importance: i32, // 1-5 scale
    pub stale: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ExtractedFact {
    /// Create a new extracted fact
    pub fn new(project_id: String, fact_type: FactType, content: String) -> Self {
        Self {
            id: String::new(), // Will be set by PocketBase
            project: project_id,
            session: None,
            fact_type,
            content,
            importance: 3, // Default middle importance
            stale: false,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    /// Get importance as star rating (★★★★★)
    pub fn importance_stars(&self) -> String {
        let filled = "★".repeat(self.importance.clamp(1, 5) as usize);
        let empty = "☆".repeat((5 - self.importance.clamp(1, 5)) as usize);
        format!("{}{}", filled, empty)
    }

    /// Get a preview of the content (first 80 chars)
    pub fn content_preview(&self) -> String {
        if self.content.len() <= 80 {
            self.content.clone()
        } else {
            format!("{}...", &self.content[..77])
        }
    }

    /// Check if high importance (4-5 stars)
    pub fn is_high_importance(&self) -> bool {
        self.importance >= 4
    }

    /// Check if low importance (1-2 stars)
    pub fn is_low_importance(&self) -> bool {
        self.importance <= 2
    }

    /// Get age in days
    pub fn age_days(&self) -> i64 {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.created);
        duration.num_days()
    }

    /// Get human-readable age
    pub fn age_display(&self) -> String {
        let days = self.age_days();
        match days {
            0 => String::from("Today"),
            1 => String::from("Yesterday"),
            2..=6 => format!("{} days ago", days),
            7..=13 => String::from("1 week ago"),
            14..=29 => format!("{} weeks ago", days / 7),
            30..=59 => String::from("1 month ago"),
            _ => format!("{} months ago", days / 30),
        }
    }
}

/// Request payload for creating/updating facts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFactPayload {
    pub project: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    pub fact_type: FactType,
    pub content: String,
    pub importance: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale: Option<bool>,
}

impl From<&ExtractedFact> for ExtractedFactPayload {
    fn from(fact: &ExtractedFact) -> Self {
        Self {
            project: fact.project.clone(),
            session: fact.session.clone(),
            fact_type: fact.fact_type,
            content: fact.content.clone(),
            importance: fact.importance,
            stale: Some(fact.stale),
        }
    }
}

/// Fact statistics for display
#[derive(Debug, Clone, Default)]
pub struct FactStats {
    pub total: usize,
    pub by_type: std::collections::HashMap<FactType, usize>,
    pub high_importance: usize,
    pub stale: usize,
}

impl FactStats {
    /// Calculate statistics from a list of facts
    pub fn from_facts(facts: &[ExtractedFact]) -> Self {
        let mut stats = Self::default();
        stats.total = facts.len();

        for fact in facts {
            *stats.by_type.entry(fact.fact_type).or_insert(0) += 1;

            if fact.is_high_importance() {
                stats.high_importance += 1;
            }

            if fact.stale {
                stats.stale += 1;
            }
        }

        stats
    }

    /// Get count for a specific fact type
    pub fn count_for_type(&self, fact_type: FactType) -> usize {
        self.by_type.get(&fact_type).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_importance_stars() {
        let mut fact = ExtractedFact::new("test".to_string(), FactType::Decision, "Test".to_string());

        fact.importance = 5;
        assert_eq!(fact.importance_stars(), "★★★★★");

        fact.importance = 3;
        assert_eq!(fact.importance_stars(), "★★★☆☆");

        fact.importance = 1;
        assert_eq!(fact.importance_stars(), "★☆☆☆☆");
    }

    #[test]
    fn test_fact_stats() {
        let facts = vec![
            ExtractedFact {
                id: "1".to_string(),
                project: "test".to_string(),
                session: None,
                fact_type: FactType::Decision,
                content: "Test".to_string(),
                importance: 5,
                stale: false,
                created: Utc::now(),
                updated: Utc::now(),
            },
            ExtractedFact {
                id: "2".to_string(),
                project: "test".to_string(),
                session: None,
                fact_type: FactType::Blocker,
                content: "Test".to_string(),
                importance: 4,
                stale: true,
                created: Utc::now(),
                updated: Utc::now(),
            },
        ];

        let stats = FactStats::from_facts(&facts);
        assert_eq!(stats.total, 2);
        assert_eq!(stats.high_importance, 2);
        assert_eq!(stats.stale, 1);
        assert_eq!(stats.count_for_type(FactType::Decision), 1);
    }
}
