package smart

import (
	"fmt"
	"strings"
	"time"
)

// DiffGenerator creates human-readable diffs between sessions
type DiffGenerator struct{}

type SessionSnapshot struct {
	SessionID   string
	Timestamp   time.Time
	Facts       []CompressibleFact
	TokenCount  int
	FileChanges []string
}

type Diff struct {
	Added      []CompressibleFact
	Removed    []CompressibleFact
	Modified   []CompressibleFact
	Summary    string
	TokenDelta int
}

func NewDiffGenerator() *DiffGenerator {
	return &DiffGenerator{}
}

// GenerateDiff compares two session snapshots
func (d *DiffGenerator) GenerateDiff(previous, current SessionSnapshot) Diff {
	diff := Diff{
		TokenDelta: current.TokenCount - previous.TokenCount,
	}

	// Create maps for quick lookup
	prevMap := make(map[string]CompressibleFact)
	currMap := make(map[string]CompressibleFact)

	for _, fact := range previous.Facts {
		key := fact.Type + ":" + fact.Content
		prevMap[key] = fact
	}

	for _, fact := range current.Facts {
		key := fact.Type + ":" + fact.Content
		currMap[key] = fact
	}

	// Find added facts
	for key, fact := range currMap {
		if _, exists := prevMap[key]; !exists {
			diff.Added = append(diff.Added, fact)
		}
	}

	// Find removed facts
	for key, fact := range prevMap {
		if _, exists := currMap[key]; !exists {
			diff.Removed = append(diff.Removed, fact)
		}
	}

	// Generate summary
	diff.Summary = d.generateSummary(diff)

	return diff
}

func (d *DiffGenerator) generateSummary(diff Diff) string {
	var parts []string

	if len(diff.Added) > 0 {
		parts = append(parts, fmt.Sprintf("%d new facts", len(diff.Added)))
	}

	if len(diff.Removed) > 0 {
		parts = append(parts, fmt.Sprintf("%d resolved", len(diff.Removed)))
	}

	if diff.TokenDelta > 0 {
		parts = append(parts, fmt.Sprintf("+%d tokens", diff.TokenDelta))
	} else if diff.TokenDelta < 0 {
		parts = append(parts, fmt.Sprintf("%d tokens", diff.TokenDelta))
	}

	if len(parts) == 0 {
		return "No significant changes"
	}

	return strings.Join(parts, ", ")
}

// FormatDiff creates a markdown representation of the diff
func (d *DiffGenerator) FormatDiff(diff Diff, previous, current SessionSnapshot) string {
	var md strings.Builder

	md.WriteString(fmt.Sprintf("# Session Diff\n\n"))
	md.WriteString(fmt.Sprintf("**Previous**: %s (%s)\n", previous.SessionID, previous.Timestamp.Format(time.RFC3339)))
	md.WriteString(fmt.Sprintf("**Current**: %s (%s)\n\n", current.SessionID, current.Timestamp.Format(time.RFC3339)))
	md.WriteString(fmt.Sprintf("**Summary**: %s\n\n", diff.Summary))

	if len(diff.Added) > 0 {
		md.WriteString("## ➕ Added Facts\n\n")
		for _, fact := range diff.Added {
			md.WriteString(fmt.Sprintf("- **[%s]** %s (importance: %d)\n", fact.Type, fact.Content, fact.Importance))
		}
		md.WriteString("\n")
	}

	if len(diff.Removed) > 0 {
		md.WriteString("## ➖ Removed/Resolved Facts\n\n")
		for _, fact := range diff.Removed {
			md.WriteString(fmt.Sprintf("- **[%s]** %s\n", fact.Type, fact.Content))
		}
		md.WriteString("\n")
	}

	if diff.TokenDelta != 0 {
		md.WriteString(fmt.Sprintf("## 📊 Token Usage\n\n"))
		md.WriteString(fmt.Sprintf("Change: %+d tokens\n\n", diff.TokenDelta))
	}

	return md.String()
}

// PreCompactDetector monitors token usage and triggers handoff before compacting
type PreCompactDetector struct {
	threshold      int
	warningPercent float64
}

func NewPreCompactDetector(threshold int) *PreCompactDetector {
	return &PreCompactDetector{
		threshold:      threshold,
		warningPercent: 0.85, // Warn at 85% of threshold
	}
}

// ShouldCreateHandoff determines if we're approaching compact threshold
func (d *PreCompactDetector) ShouldCreateHandoff(currentTokens int) bool {
	return float64(currentTokens) >= float64(d.threshold)*d.warningPercent
}

// TimeUntilCompact estimates remaining tokens before compacting
func (d *PreCompactDetector) TimeUntilCompact(currentTokens int) int {
	remaining := d.threshold - currentTokens
	if remaining < 0 {
		return 0
	}
	return remaining
}
