package monitor

import (
	"log"
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

type EnhancedWatcher struct {
	config            WatcherConfig
	watcher           *fsnotify.Watcher
	parser            *Parser
	ledger            *ledger.Ledger
	importanceScorer  *smart.ImportanceScorer
	staleDetector     *smart.StaleDetector
	compressor        *smart.ContextCompressor
	compactDetector   *smart.PreCompactDetector
	diffGenerator     *smart.DiffGenerator
	currentTokenCount int
	sessionID         string
	lastHandoff       time.Time
}

func NewWatcherWithConfig(config WatcherConfig) (*EnhancedWatcher, error) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return nil, err
	}

	sessionID := time.Now().Format("20060102_150405")

	return &EnhancedWatcher{
		config:           config,
		watcher:          watcher,
		parser:           NewParser(),
		ledger:           ledger.NewLedger(config.ProjectID, config.RepoPath),
		importanceScorer: smart.NewImportanceScorer(),
		staleDetector:    smart.NewStaleDetector(),
		compressor:       smart.NewContextCompressor(10), // Keep top 10 per fact type
		compactDetector:  smart.NewPreCompactDetector(config.CompactThreshold),
		diffGenerator:    smart.NewDiffGenerator(),
		sessionID:        sessionID,
		lastHandoff:      time.Now(),
	}, nil
}

func (w *EnhancedWatcher) Start() error {
	// Use the original Start method from base Watcher
	baseWatcher := &Watcher{
		logPath:   w.config.LogPath,
		projectID: w.config.ProjectID,
		client:    w.config.Client,
		watcher:   w.watcher,
		verbose:   w.config.Verbose,
		parser:    w.parser,
	}

	return baseWatcher.Start()
}

func (w *EnhancedWatcher) Stop() {
	w.watcher.Close()

	// Create final handoff before stopping
	w.createHandoffIfNeeded(true)
}

func (w *EnhancedWatcher) ProcessWithSmartFeatures(facts []extractor.Fact, tokenCount int) {
	w.currentTokenCount = tokenCount

	// Check if we should create pre-compact handoff
	if w.compactDetector.ShouldCreateHandoff(tokenCount) {
		w.createHandoffIfNeeded(false)
	}

	// Apply importance scoring
	enhancedFacts := make([]ledger.Fact, len(facts))
	for i, fact := range facts {
		importance := w.importanceScorer.CalculateImportance(
			fact.Type,
			fact.Content,
			time.Now(),
		)

		enhancedFacts[i] = ledger.Fact{
			Type:       fact.Type,
			Content:    fact.Content,
			Importance: importance,
			Timestamp:  time.Now(),
		}

		// Update fact with calculated importance
		fact.Importance = importance
		w.config.Client.CreateFact(w.config.ProjectID, fact)
	}

	// Update continuity ledger
	entry := ledger.LedgerEntry{
		Timestamp:   time.Now(),
		SessionID:   w.sessionID,
		ProjectID:   w.config.ProjectID,
		TokenCount:  tokenCount,
		Facts:       enhancedFacts,
		Context:     make(map[string]interface{}),
		Decisions:   w.filterFactsByType(enhancedFacts, "decision"),
		NextSteps:   w.filterFactsByType(enhancedFacts, "todo"),
		Blockers:    w.filterFactsByType(enhancedFacts, "blocker"),
		FileChanges: w.filterFactsByType(enhancedFacts, "file_change"),
	}

	if err := w.ledger.AppendEntry(entry); err != nil {
		if w.config.Verbose {
			log.Printf("Failed to update ledger: %v", err)
		}
	}

	// Log progress
	if w.config.Verbose {
		remaining := w.compactDetector.TimeUntilCompact(tokenCount)
		log.Printf("Token usage: %d/%d (remaining: %d)",
			tokenCount, w.config.CompactThreshold, remaining)
	}
}

func (w *EnhancedWatcher) createHandoffIfNeeded(force bool) {
	// Don't create handoffs too frequently (minimum 30 min apart)
	if !force && time.Since(w.lastHandoff) < 30*time.Minute {
		return
	}

	// Get latest ledger entry
	latest, err := w.ledger.GetLatestEntry()
	if err != nil {
		log.Printf("Failed to get latest ledger entry: %v", err)
		return
	}

	// Create handoff document
	summary := w.generateHandoffSummary(latest)
	if err := w.ledger.CreateHandoff(w.sessionID, summary, latest.Facts); err != nil {
		log.Printf("Failed to create handoff: %v", err)
		return
	}

	w.lastHandoff = time.Now()

	if w.config.Verbose || force {
		log.Printf("✓ Handoff created (tokens: %d, facts: %d)",
			latest.TokenCount, len(latest.Facts))
	}
}

func (w *EnhancedWatcher) generateHandoffSummary(entry *ledger.LedgerEntry) string {
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

func (w *EnhancedWatcher) filterFactsByType(facts []ledger.Fact, factType string) []string {
	var result []string
	for _, fact := range facts {
		if fact.Type == factType {
			result = append(result, fact.Content)
		}
	}
	return result
}
