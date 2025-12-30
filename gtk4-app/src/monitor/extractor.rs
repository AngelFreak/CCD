use crate::models::{ExtractedFact, ExtractedFactPayload, FactType};
use anyhow::Result;
use regex::Regex;
use std::sync::OnceLock;

/// Regex patterns for fact extraction (compiled once)
static DECISION_PATTERN: OnceLock<Regex> = OnceLock::new();
static BLOCKER_PATTERN: OnceLock<Regex> = OnceLock::new();
static TODO_PATTERN: OnceLock<Regex> = OnceLock::new();
static FILE_CHANGE_PATTERN: OnceLock<Regex> = OnceLock::new();
static DEPENDENCY_PATTERN: OnceLock<Regex> = OnceLock::new();
static INSIGHT_PATTERN: OnceLock<Regex> = OnceLock::new();

/// Initialize regex patterns (called once)
fn init_patterns() {
    DECISION_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)(decided to|chose to|going with|will use|opted for)").unwrap()
    });

    BLOCKER_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)(blocked by|can't proceed|cannot continue|error:|failed to|exception)").unwrap()
    });

    TODO_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)(TODO:|FIXME:|need to|should|must|have to)").unwrap()
    });

    FILE_CHANGE_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)(created?|modified?|updated?|deleted?|removed?)\s+.*\.(rs|ts|tsx|js|jsx|py|go|java|cpp|h|c|cs)").unwrap()
    });

    DEPENDENCY_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)(installed|added|npm install|cargo add|pip install|go get)").unwrap()
    });

    INSIGHT_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)(discovered|found that|learned that|note that|important:)").unwrap()
    });
}

/// Fact extractor for Claude Code conversation logs
pub struct FactExtractor {
    project_id: String,
}

impl FactExtractor {
    /// Create a new fact extractor for a project
    pub fn new(project_id: String) -> Self {
        init_patterns();
        Self { project_id }
    }

    /// Extract facts from a message
    pub fn extract_from_message(&self, content: &str, session_id: Option<String>) -> Vec<ExtractedFactPayload> {
        let mut facts = Vec::new();

        // Split into lines for better extraction
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Try to extract each fact type
            if let Some(fact) = self.try_extract_decision(line, session_id.clone()) {
                facts.push(fact);
            }
            if let Some(fact) = self.try_extract_blocker(line, session_id.clone()) {
                facts.push(fact);
            }
            if let Some(fact) = self.try_extract_todo(line, session_id.clone()) {
                facts.push(fact);
            }
            if let Some(fact) = self.try_extract_file_change(line, session_id.clone()) {
                facts.push(fact);
            }
            if let Some(fact) = self.try_extract_dependency(line, session_id.clone()) {
                facts.push(fact);
            }
            if let Some(fact) = self.try_extract_insight(line, session_id.clone()) {
                facts.push(fact);
            }
        }

        facts
    }

    fn try_extract_decision(&self, line: &str, session_id: Option<String>) -> Option<ExtractedFactPayload> {
        if DECISION_PATTERN.get()?.is_match(line) {
            Some(ExtractedFactPayload {
                project: self.project_id.clone(),
                session: session_id,
                fact_type: FactType::Decision,
                content: line.to_string(),
                importance: 4, // Decisions are high importance
                stale: None,
            })
        } else {
            None
        }
    }

    fn try_extract_blocker(&self, line: &str, session_id: Option<String>) -> Option<ExtractedFactPayload> {
        if BLOCKER_PATTERN.get()?.is_match(line) {
            Some(ExtractedFactPayload {
                project: self.project_id.clone(),
                session: session_id,
                fact_type: FactType::Blocker,
                content: line.to_string(),
                importance: 5, // Blockers are highest importance
                stale: None,
            })
        } else {
            None
        }
    }

    fn try_extract_todo(&self, line: &str, session_id: Option<String>) -> Option<ExtractedFactPayload> {
        if TODO_PATTERN.get()?.is_match(line) {
            Some(ExtractedFactPayload {
                project: self.project_id.clone(),
                session: session_id,
                fact_type: FactType::Todo,
                content: line.to_string(),
                importance: 3, // Todos are medium importance
                stale: None,
            })
        } else {
            None
        }
    }

    fn try_extract_file_change(&self, line: &str, session_id: Option<String>) -> Option<ExtractedFactPayload> {
        if FILE_CHANGE_PATTERN.get()?.is_match(line) {
            Some(ExtractedFactPayload {
                project: self.project_id.clone(),
                session: session_id,
                fact_type: FactType::FileChange,
                content: line.to_string(),
                importance: 3, // File changes are medium importance
                stale: None,
            })
        } else {
            None
        }
    }

    fn try_extract_dependency(&self, line: &str, session_id: Option<String>) -> Option<ExtractedFactPayload> {
        if DEPENDENCY_PATTERN.get()?.is_match(line) {
            Some(ExtractedFactPayload {
                project: self.project_id.clone(),
                session: session_id,
                fact_type: FactType::Dependency,
                content: line.to_string(),
                importance: 4, // Dependencies are high importance
                stale: None,
            })
        } else {
            None
        }
    }

    fn try_extract_insight(&self, line: &str, session_id: Option<String>) -> Option<ExtractedFactPayload> {
        if INSIGHT_PATTERN.get()?.is_match(line) {
            Some(ExtractedFactPayload {
                project: self.project_id.clone(),
                session: session_id,
                fact_type: FactType::Insight,
                content: line.to_string(),
                importance: 3, // Insights are medium importance
                stale: None,
            })
        } else {
            None
        }
    }
}

/// Parse a Claude Code conversation log file
pub fn parse_conversation_log(content: &str) -> Result<ConversationLog> {
    let log: ConversationLog = serde_json::from_str(content)?;
    Ok(log)
}

/// Simplified conversation log structure
#[derive(Debug, serde::Deserialize)]
pub struct ConversationLog {
    pub conversation_id: Option<String>,
    pub messages: Vec<Message>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl ConversationLog {
    /// Count total tokens (simplified estimation)
    pub fn estimate_tokens(&self) -> i64 {
        // Rough estimate: 1 token ≈ 4 characters
        let total_chars: usize = self.messages.iter()
            .map(|m| m.content.len())
            .sum();
        (total_chars / 4) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_decision() {
        let extractor = FactExtractor::new("test-project".to_string());
        let facts = extractor.extract_from_message(
            "I decided to use Rust for this project",
            None,
        );
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].fact_type, FactType::Decision);
        assert_eq!(facts[0].importance, 4);
    }

    #[test]
    fn test_extract_blocker() {
        let extractor = FactExtractor::new("test-project".to_string());
        let facts = extractor.extract_from_message(
            "Error: failed to connect to database",
            None,
        );
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].fact_type, FactType::Blocker);
        assert_eq!(facts[0].importance, 5);
    }

    #[test]
    fn test_extract_todo() {
        let extractor = FactExtractor::new("test-project".to_string());
        let facts = extractor.extract_from_message(
            "TODO: implement error handling",
            None,
        );
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].fact_type, FactType::Todo);
    }

    #[test]
    fn test_extract_file_change() {
        let extractor = FactExtractor::new("test-project".to_string());
        let facts = extractor.extract_from_message(
            "Created new file: src/main.rs",
            None,
        );
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].fact_type, FactType::FileChange);
    }

    #[test]
    fn test_extract_multiple() {
        let extractor = FactExtractor::new("test-project".to_string());
        let facts = extractor.extract_from_message(
            "I decided to use SQLite. TODO: add migrations. Created database.rs file.",
            None,
        );
        assert_eq!(facts.len(), 3);
    }
}
