package monitor

import (
	"encoding/json"
	"log"
	"os"
	"path/filepath"
	"time"

	"github.com/angelfreak/ccd/daemon/api"
	"github.com/angelfreak/ccd/daemon/extractor"
	"github.com/angelfreak/ccd/daemon/ledger"
	"github.com/angelfreak/ccd/daemon/smart"
	"github.com/fsnotify/fsnotify"
)

type WatcherConfig struct {
	LogPath          string
	ProjectID        string
	RepoPath         string
	Client           *api.Client
	Verbose          bool
	SmartMode        bool
	CompactThreshold int
}

type Watcher struct {
	logPath          string
	projectID        string
	repoPath         string
	client           *api.Client
	watcher          *fsnotify.Watcher
	verbose          bool
	smartMode        bool
	parser           *Parser
	ledger           *ledger.Ledger
	importanceScorer *smart.ImportanceScorer
	staleDetector    *smart.StaleDetector
	compactDetector  *smart.PreCompactDetector
	currentTokens    int
	sessionID        string
	lastHandoff      time.Time
}

func NewWatcher(logPath, projectID string, client *api.Client, verbose bool) (*Watcher, error) {
	return NewWatcherWithConfig(WatcherConfig{
		LogPath:          logPath,
		ProjectID:        projectID,
		Client:           client,
		Verbose:          verbose,
		SmartMode:        false,
		CompactThreshold: 170000,
	})
}

func NewWatcherWithConfig(config WatcherConfig) (*Watcher, error) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return nil, err
	}

	w := &Watcher{
		logPath:      config.LogPath,
		projectID:    config.ProjectID,
		repoPath:     config.RepoPath,
		client:       config.Client,
		watcher:      watcher,
		verbose:      config.Verbose,
		smartMode:    config.SmartMode,
		parser:       NewParser(),
		sessionID:    time.Now().Format("20060102_150405"),
		lastHandoff:  time.Now(),
	}

	// Initialize smart features if enabled
	if config.SmartMode {
		w.ledger = ledger.NewLedger(config.ProjectID, config.RepoPath)
		w.importanceScorer = smart.NewImportanceScorer()
		w.staleDetector = smart.NewStaleDetector()
		w.compactDetector = smart.NewPreCompactDetector(config.CompactThreshold)
	}

	return w, nil
}

func (w *Watcher) Start() error {
	// Watch the logs directory
	if err := w.watcher.Add(w.logPath); err != nil {
		return err
	}

	// Process existing log files
	if err := w.processExistingLogs(); err != nil {
		log.Printf("Warning: failed to process existing logs: %v", err)
	}

	// Start watching for new events
	go w.watch()

	return nil
}

func (w *Watcher) Stop() {
	// Create final handoff if smart mode enabled
	if w.smartMode {
		w.createHandoffIfNeeded(true)
	}
	w.watcher.Close()
}

func (w *Watcher) watch() {
	for {
		select {
		case event, ok := <-w.watcher.Events:
			if !ok {
				return
			}

			if event.Op&fsnotify.Write == fsnotify.Write {
				if w.verbose {
					log.Printf("Modified file: %s", event.Name)
				}
				w.processLogFile(event.Name)
			}

		case err, ok := <-w.watcher.Errors:
			if !ok {
				return
			}
			log.Printf("Watcher error: %v", err)
		}
	}
}

func (w *Watcher) processExistingLogs() error {
	entries, err := os.ReadDir(w.logPath)
	if err != nil {
		return err
	}

	for _, entry := range entries {
		if !entry.IsDir() && filepath.Ext(entry.Name()) == ".log" {
			w.processLogFile(filepath.Join(w.logPath, entry.Name()))
		}
	}

	return nil
}

func (w *Watcher) processLogFile(path string) {
	data, err := os.ReadFile(path)
	if err != nil {
		if w.verbose {
			log.Printf("Failed to read log file: %v", err)
		}
		return
	}

	// Parse conversation
	conversation, err := w.parser.Parse(string(data))
	if err != nil {
		if w.verbose {
			log.Printf("Failed to parse conversation: %v", err)
		}
		return
	}

	// Extract facts
	facts := extractor.ExtractFacts(conversation)

	// Update token count
	tokenCount := w.parser.CountTokens(conversation)
	w.currentTokens = tokenCount

	// Process with smart features if enabled
	if w.smartMode {
		w.processWithSmartFeatures(facts, tokenCount)
	} else {
		// Basic processing without smart features
		for _, fact := range facts {
			if err := w.client.CreateFact(w.projectID, fact); err != nil {
				log.Printf("Failed to create fact: %v", err)
			} else if w.verbose {
				log.Printf("Created fact: %s (%s)", fact.Content, fact.Type)
			}
		}
	}

	if w.verbose {
		log.Printf("Token count: %d", tokenCount)
	}
}

func (w *Watcher) processWithSmartFeatures(facts []extractor.Fact, tokenCount int) {
	// Check if we should create pre-compact handoff
	if w.compactDetector.ShouldCreateHandoff(tokenCount) {
		w.createHandoffIfNeeded(false)
	}

	// Apply importance scoring and create facts
	enhancedFacts := make([]ledger.Fact, 0, len(facts))
	for _, fact := range facts {
		// Calculate importance
		importance := w.importanceScorer.CalculateImportance(
			fact.Type,
			fact.Content,
			time.Now(),
		)
		fact.Importance = importance

		// Create fact in PocketBase
		if err := w.client.CreateFact(w.projectID, fact); err != nil {
			log.Printf("Failed to create fact: %v", err)
		} else if w.verbose {
			log.Printf("Created fact (importance: %d): %s (%s)", importance, fact.Content, fact.Type)
		}

		// Add to enhanced facts for ledger
		enhancedFacts = append(enhancedFacts, ledger.Fact{
			Type:       fact.Type,
			Content:    fact.Content,
			Importance: importance,
			Timestamp:  time.Now(),
		})
	}

	// Update continuity ledger
	entry := ledger.LedgerEntry{
		Timestamp:   time.Now(),
		SessionID:   w.sessionID,
		ProjectID:   w.projectID,
		TokenCount:  tokenCount,
		Facts:       enhancedFacts,
		Context:     make(map[string]interface{}),
		Decisions:   w.filterFactsByType(enhancedFacts, "decision"),
		NextSteps:   w.filterFactsByType(enhancedFacts, "todo"),
		Blockers:    w.filterFactsByType(enhancedFacts, "blocker"),
		FileChanges: w.filterFactsByType(enhancedFacts, "file_change"),
	}

	if err := w.ledger.AppendEntry(entry); err != nil && w.verbose {
		log.Printf("Failed to update ledger: %v", err)
	}

	// Log progress
	if w.verbose {
		remaining := w.compactDetector.TimeUntilCompact(tokenCount)
		log.Printf("Smart features: %d facts processed, %d tokens remaining until compact",
			len(facts), remaining)
	}
}

func (w *Watcher) createHandoffIfNeeded(force bool) {
	// Don't create handoffs too frequently (minimum 30 min apart)
	if !force && time.Since(w.lastHandoff) < 30*time.Minute {
		return
	}

	// Get latest ledger entry
	latest, err := w.ledger.GetLatestEntry()
	if err != nil {
		if w.verbose {
			log.Printf("Failed to get latest ledger entry: %v", err)
		}
		return
	}

	// Create handoff document
	summary := w.generateHandoffSummary(latest)
	if err := w.ledger.CreateHandoff(w.sessionID, summary, latest.Facts); err != nil {
		log.Printf("Failed to create handoff: %v", err)
		return
	}

	w.lastHandoff = time.Now()

	if w.verbose || force {
		log.Printf("✓ Handoff created: %s (tokens: %d, facts: %d)",
			summary, latest.TokenCount, len(latest.Facts))
	}
}

func (w *Watcher) generateHandoffSummary(entry *ledger.LedgerEntry) string {
	summary := ""

	if len(entry.Decisions) > 0 {
		summary += "Made architectural decisions. "
	}

	if len(entry.Blockers) > 0 {
		summary += "Encountered blockers. "
	}

	if len(entry.FileChanges) > 0 {
		summary += "Modified codebase. "
	}

	if summary == "" {
		summary = "Continued development work."
	}

	return summary
}

func (w *Watcher) filterFactsByType(facts []ledger.Fact, factType string) []string {
	var result []string
	for _, fact := range facts {
		if fact.Type == factType {
			result = append(result, fact.Content)
		}
	}
	return result
}

type Conversation struct {
	Messages []Message `json:"messages"`
}

type Message struct {
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}
