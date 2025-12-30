package smart

import (
	"math"
	"strings"
	"time"
)

// ImportanceScorer calculates importance scores for facts
type ImportanceScorer struct {
	weights map[string]float64
}

func NewImportanceScorer() *ImportanceScorer {
	return &ImportanceScorer{
		weights: map[string]float64{
			"blocker":     1.0,  // Highest priority
			"decision":    0.9,  // Critical architectural choices
			"dependency":  0.7,  // Important but not urgent
			"todo":        0.6,  // Task tracking
			"insight":     0.5,  // Learning outcomes
			"file_change": 0.4,  // Implementation details
		},
	}
}

// CalculateImportance returns a score from 1-5
func (s *ImportanceScorer) CalculateImportance(factType, content string, recency time.Time) int {
	score := 0.0

	// Base weight from type
	if w, ok := s.weights[factType]; ok {
		score += w * 3.0 // Max 3 points from type
	}

	// Content analysis (max 1.5 points)
	score += s.analyzeContent(content)

	// Recency bonus (max 0.5 points)
	score += s.recencyBonus(recency)

	// Convert to 1-5 scale
	normalized := int(math.Round(score))
	if normalized < 1 {
		return 1
	}
	if normalized > 5 {
		return 5
	}
	return normalized
}

func (s *ImportanceScorer) analyzeContent(content string) float64 {
	score := 0.0
	lower := strings.ToLower(content)

	// High-value keywords
	highValue := []string{"critical", "breaking", "urgent", "security", "bug", "crash", "error"}
	for _, keyword := range highValue {
		if strings.Contains(lower, keyword) {
			score += 0.3
		}
	}

	// Medium-value keywords
	mediumValue := []string{"important", "major", "refactor", "optimize", "performance"}
	for _, keyword := range mediumValue {
		if strings.Contains(lower, keyword) {
			score += 0.2
		}
	}

	// Length bonus (longer = more detailed = more important)
	if len(content) > 100 {
		score += 0.3
	} else if len(content) > 50 {
		score += 0.2
	}

	return math.Min(score, 1.5)
}

func (s *ImportanceScorer) recencyBonus(t time.Time) float64 {
	hours := time.Since(t).Hours()
	if hours < 1 {
		return 0.5
	} else if hours < 24 {
		return 0.3
	} else if hours < 168 { // 1 week
		return 0.1
	}
	return 0.0
}

// StaleDetector identifies facts that are no longer relevant
type StaleDetector struct {
	staleDays map[string]int
}

func NewStaleDetector() *StaleDetector {
	return &StaleDetector{
		staleDays: map[string]int{
			"blocker":     3,   // Blockers resolved quickly or abandoned
			"todo":        7,   // Todos either done or deprioritized
			"file_change": 14,  // Implementation details fade
			"dependency":  30,  // Dependencies stable after install
			"decision":    90,  // Decisions remain relevant longer
			"insight":     60,  // Insights useful for a while
		},
	}
}

// IsStale checks if a fact is outdated
func (d *StaleDetector) IsStale(factType string, created time.Time, content string) bool {
	days, ok := d.staleDays[factType]
	if !ok {
		days = 30 // Default
	}

	age := time.Since(created)
	threshold := time.Duration(days) * 24 * time.Hour

	// Special cases
	if factType == "blocker" && strings.Contains(strings.ToLower(content), "resolved") {
		return true
	}

	if factType == "todo" && strings.Contains(strings.ToLower(content), "done") {
		return true
	}

	return age > threshold
}

// ContextCompressor summarizes facts for efficient storage
type ContextCompressor struct {
	maxFactsPerType int
}

func NewContextCompressor(maxFactsPerType int) *ContextCompressor {
	return &ContextCompressor{
		maxFactsPerType: maxFactsPerType,
	}
}

type CompressibleFact struct {
	Type       string
	Content    string
	Importance int
	Created    time.Time
	Stale      bool
}

// Compress reduces fact count while preserving important information
func (c *ContextCompressor) Compress(facts []CompressibleFact) []CompressibleFact {
	// Group by type
	grouped := make(map[string][]CompressibleFact)
	for _, fact := range facts {
		if !fact.Stale {
			grouped[fact.Type] = append(grouped[fact.Type], fact)
		}
	}

	// Keep top N per type by importance and recency
	var compressed []CompressibleFact
	for _, typeFacts := range grouped {
		// Sort by importance (desc) then recency
		sorted := c.sortByImportance(typeFacts)

		// Take top N
		limit := c.maxFactsPerType
		if len(sorted) < limit {
			limit = len(sorted)
		}
		compressed = append(compressed, sorted[:limit]...)
	}

	return compressed
}

func (c *ContextCompressor) sortByImportance(facts []CompressibleFact) []CompressibleFact {
	// Simple bubble sort (sufficient for small datasets)
	sorted := make([]CompressibleFact, len(facts))
	copy(sorted, facts)

	for i := 0; i < len(sorted); i++ {
		for j := i + 1; j < len(sorted); j++ {
			if sorted[i].Importance < sorted[j].Importance ||
				(sorted[i].Importance == sorted[j].Importance && sorted[i].Created.Before(sorted[j].Created)) {
				sorted[i], sorted[j] = sorted[j], sorted[i]
			}
		}
	}

	return sorted
}
