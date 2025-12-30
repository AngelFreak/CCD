use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Session history model representing a Claude Code conversation session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHistory {
    pub id: String,
    pub project: String, // Project ID
    pub summary: String,
    pub facts_extracted: i32,
    pub token_count: i64,
    pub session_start: DateTime<Utc>,
    pub session_end: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl SessionHistory {
    /// Create a new session
    pub fn new(project_id: String, summary: String) -> Self {
        Self {
            id: String::new(), // Will be set by PocketBase
            project: project_id,
            summary,
            facts_extracted: 0,
            token_count: 0,
            session_start: Utc::now(),
            session_end: None,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    /// Get session duration as a human-readable string
    pub fn duration_display(&self) -> String {
        if let Some(end) = self.session_end {
            let duration = end.signed_duration_since(self.session_start);
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;

            if hours > 0 {
                format!("{}h {}m", hours, minutes)
            } else {
                format!("{}m", minutes)
            }
        } else {
            String::from("In progress")
        }
    }

    /// Get token usage percentage (out of 200K context window)
    pub fn token_percentage(&self) -> f64 {
        const MAX_TOKENS: f64 = 200_000.0;
        (self.token_count as f64 / MAX_TOKENS) * 100.0
    }

    /// Format token count with thousands separator
    pub fn token_count_display(&self) -> String {
        format_number_with_separator(self.token_count)
    }

    /// Check if approaching context limit (> 85%)
    pub fn is_near_limit(&self) -> bool {
        self.token_percentage() > 85.0
    }

    /// Check if session is active (no end time)
    pub fn is_active(&self) -> bool {
        self.session_end.is_none()
    }
}

/// Request payload for creating/updating sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPayload {
    pub project: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facts_extracted: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_end: Option<DateTime<Utc>>,
}

impl From<&SessionHistory> for SessionPayload {
    fn from(session: &SessionHistory) -> Self {
        Self {
            project: session.project.clone(),
            summary: session.summary.clone(),
            facts_extracted: Some(session.facts_extracted),
            token_count: Some(session.token_count),
            session_start: Some(session.session_start),
            session_end: session.session_end,
        }
    }
}

/// Helper function to format numbers with thousands separator
fn format_number_with_separator(num: i64) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_percentage() {
        let mut session = SessionHistory::new("test".to_string(), "Test".to_string());
        session.token_count = 100_000;
        assert_eq!(session.token_percentage(), 50.0);

        session.token_count = 170_000;
        assert_eq!(session.token_percentage(), 85.0);
        assert!(session.is_near_limit());
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number_with_separator(1000), "1,000");
        assert_eq!(format_number_with_separator(100000), "100,000");
        assert_eq!(format_number_with_separator(1234567), "1,234,567");
    }
}
