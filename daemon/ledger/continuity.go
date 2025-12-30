package ledger

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

// LedgerEntry represents a snapshot of project state
type LedgerEntry struct {
	Timestamp   time.Time              `json:"timestamp"`
	SessionID   string                 `json:"session_id"`
	ProjectID   string                 `json:"project_id"`
	TokenCount  int                    `json:"token_count"`
	Facts       []Fact                 `json:"facts"`
	Context     map[string]interface{} `json:"context"`
	Decisions   []string               `json:"decisions"`
	NextSteps   []string               `json:"next_steps"`
	Blockers    []string               `json:"blockers"`
	FileChanges []string               `json:"file_changes"`
}

type Fact struct {
	Type       string    `json:"type"`
	Content    string    `json:"content"`
	Importance int       `json:"importance"`
	Timestamp  time.Time `json:"timestamp"`
}

type Ledger struct {
	ledgerPath string
	projectID  string
}

func NewLedger(projectID, repoPath string) *Ledger {
	ledgerPath := filepath.Join(repoPath, "thoughts", "ledgers")
	os.MkdirAll(ledgerPath, 0755)

	return &Ledger{
		ledgerPath: ledgerPath,
		projectID:  projectID,
	}
}

// AppendEntry adds a new entry to the continuity ledger
func (l *Ledger) AppendEntry(entry LedgerEntry) error {
	filename := fmt.Sprintf("CONTINUITY_%s.jsonl", time.Now().Format("2006-01-02"))
	path := filepath.Join(l.ledgerPath, filename)

	file, err := os.OpenFile(path, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	data, err := json.Marshal(entry)
	if err != nil {
		return err
	}

	_, err = file.Write(append(data, '\n'))
	return err
}

// GetLatestEntry retrieves the most recent ledger entry
func (l *Ledger) GetLatestEntry() (*LedgerEntry, error) {
	files, err := filepath.Glob(filepath.Join(l.ledgerPath, "CONTINUITY_*.jsonl"))
	if err != nil || len(files) == 0 {
		return nil, err
	}

	// Read the most recent file
	latestFile := files[len(files)-1]
	data, err := os.ReadFile(latestFile)
	if err != nil {
		return nil, err
	}

	// Parse last line
	lines := splitLines(string(data))
	if len(lines) == 0 {
		return nil, fmt.Errorf("empty ledger file")
	}

	var entry LedgerEntry
	err = json.Unmarshal([]byte(lines[len(lines)-1]), &entry)
	return &entry, err
}

// CreateHandoff generates a handoff document before context clearing
func (l *Ledger) CreateHandoff(sessionID string, summary string, facts []Fact) error {
	handoffPath := filepath.Join(filepath.Dir(l.ledgerPath), "shared", "handoffs")
	os.MkdirAll(handoffPath, 0755)

	filename := fmt.Sprintf("handoff_%s_%s.md", sessionID, time.Now().Format("20060102_150405"))
	path := filepath.Join(handoffPath, filename)

	content := fmt.Sprintf(`# Session Handoff

**Session ID**: %s
**Timestamp**: %s
**Project**: %s

## Summary
%s

## Key Facts
`, sessionID, time.Now().Format(time.RFC3339), l.projectID, summary)

	for _, fact := range facts {
		content += fmt.Sprintf("- [%s] %s (importance: %d)\n", fact.Type, fact.Content, fact.Importance)
	}

	content += "\n## Next Steps\n"
	for _, fact := range facts {
		if fact.Type == "todo" {
			content += fmt.Sprintf("- [ ] %s\n", fact.Content)
		}
	}

	content += "\n## Blockers\n"
	for _, fact := range facts {
		if fact.Type == "blocker" {
			content += fmt.Sprintf("- ⚠️ %s\n", fact.Content)
		}
	}

	return os.WriteFile(path, []byte(content), 0644)
}

func splitLines(s string) []string {
	var lines []string
	var line string
	for _, c := range s {
		if c == '\n' {
			if line != "" {
				lines = append(lines, line)
			}
			line = ""
		} else {
			line += string(c)
		}
	}
	if line != "" {
		lines = append(lines, line)
	}
	return lines
}
