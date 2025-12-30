use crate::models::{ExtractedFact, FactType};
use chrono::{DateTime, Duration, Utc};

/// Importance scorer for extracted facts
pub struct ImportanceScorer;

impl ImportanceScorer {
    /// Calculate final importance score (1-5) for a fact
    pub fn calculate_score(fact: &ExtractedFact) -> i32 {
        let base_score = Self::base_score_for_type(fact.fact_type);
        let content_bonus = Self::analyze_content(&fact.content);
        let recency_bonus = Self::recency_bonus(&fact.created);

        let total = base_score + content_bonus + recency_bonus;

        // Clamp to 1-5 range
        total.clamp(1, 5)
    }

    /// Base score by fact type
    fn base_score_for_type(fact_type: FactType) -> i32 {
        match fact_type {
            FactType::Blocker => 5,      // Blockers are always high priority
            FactType::Decision => 4,     // Decisions are very important
            FactType::Dependency => 4,   // New dependencies are important
            FactType::FileChange => 3,   // File changes are medium
            FactType::Todo => 3,         // Todos are medium
            FactType::Insight => 3,      // Insights are medium
        }
    }

    /// Analyze content for importance keywords
    fn analyze_content(content: &str) -> i32 {
        let content_lower = content.to_lowercase();
        let mut bonus = 0;

        // Critical keywords add importance
        if content_lower.contains("critical") ||
           content_lower.contains("urgent") ||
           content_lower.contains("blocker") ||
           content_lower.contains("security") {
            bonus += 1;
        }

        // Breaking changes are important
        if content_lower.contains("breaking") ||
           content_lower.contains("incompatible") {
            bonus += 1;
        }

        // Performance issues are notable
        if content_lower.contains("slow") ||
           content_lower.contains("performance") ||
           content_lower.contains("optimization") {
            bonus += 1;
        }

        // Longer content might be more important (cap at +1)
        if content.len() > 200 {
            bonus += 1;
        }

        bonus.min(2) // Cap content bonus at +2
    }

    /// Recency bonus (newer facts are more important)
    fn recency_bonus(created: &DateTime<Utc>) -> i32 {
        let now = Utc::now();
        let age = now.signed_duration_since(*created);

        if age < Duration::hours(1) {
            1 // Very recent: +1
        } else if age < Duration::hours(24) {
            0 // Today: no bonus
        } else {
            -1 // Older: -1
        }
    }
}

/// Staleness detector for facts
pub struct StalenessDetector;

impl StalenessDetector {
    /// Check if a fact should be marked as stale
    pub fn is_stale(fact: &ExtractedFact) -> bool {
        let now = Utc::now();
        let age = now.signed_duration_since(fact.created);

        // Content-based staleness
        if Self::has_completion_keywords(&fact.content) {
            return true;
        }

        // Time-based staleness by type
        let stale_threshold = match fact.fact_type {
            FactType::Blocker => Duration::days(3),       // Blockers should be resolved quickly
            FactType::Todo => Duration::days(14),         // Todos have 2 weeks
            FactType::FileChange => Duration::days(30),   // File changes are relevant for a month
            FactType::Dependency => Duration::days(90),   // Dependencies stay relevant longer
            FactType::Decision => Duration::days(180),    // Decisions are long-lived
            FactType::Insight => Duration::days(90),      // Insights stay relevant
        };

        age > stale_threshold
    }

    /// Check for keywords indicating completion/resolution
    fn has_completion_keywords(content: &str) -> bool {
        let content_lower = content.to_lowercase();

        content_lower.contains("resolved") ||
        content_lower.contains("fixed") ||
        content_lower.contains("done") ||
        content_lower.contains("completed") ||
        content_lower.contains("finished") ||
        content_lower.contains("merged") ||
        content_lower.contains("closed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocker_has_high_score() {
        let fact = ExtractedFact {
            id: "test".to_string(),
            project: "proj".to_string(),
            session: None,
            fact_type: FactType::Blocker,
            content: "Error in production".to_string(),
            importance: 0,
            stale: false,
            created: Utc::now(),
            updated: Utc::now(),
        };

        let score = ImportanceScorer::calculate_score(&fact);
        assert!(score >= 4, "Blockers should have high score");
    }

    #[test]
    fn test_critical_keyword_bonus() {
        let fact = ExtractedFact {
            id: "test".to_string(),
            project: "proj".to_string(),
            session: None,
            fact_type: FactType::Todo,
            content: "CRITICAL: Fix security vulnerability".to_string(),
            importance: 0,
            stale: false,
            created: Utc::now(),
            updated: Utc::now(),
        };

        let score = ImportanceScorer::calculate_score(&fact);
        assert!(score >= 4, "Critical todos should get bonus");
    }

    #[test]
    fn test_old_blocker_is_stale() {
        let fact = ExtractedFact {
            id: "test".to_string(),
            project: "proj".to_string(),
            session: None,
            fact_type: FactType::Blocker,
            content: "Some old blocker".to_string(),
            importance: 5,
            stale: false,
            created: Utc::now() - Duration::days(5),
            updated: Utc::now() - Duration::days(5),
        };

        assert!(StalenessDetector::is_stale(&fact), "Old blocker should be stale");
    }

    #[test]
    fn test_resolved_is_stale() {
        let fact = ExtractedFact {
            id: "test".to_string(),
            project: "proj".to_string(),
            session: None,
            fact_type: FactType::Todo,
            content: "TODO: Fix bug - RESOLVED".to_string(),
            importance: 3,
            stale: false,
            created: Utc::now(),
            updated: Utc::now(),
        };

        assert!(StalenessDetector::is_stale(&fact), "Resolved fact should be stale");
    }
}
