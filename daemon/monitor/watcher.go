package monitor

import (
	"encoding/json"
	"log"
	"os"
	"path/filepath"
	"time"

	"github.com/angelfreak/ccd/daemon/api"
	"github.com/angelfreak/ccd/daemon/extractor"
	"github.com/fsnotify/fsnotify"
)

type Watcher struct {
	logPath   string
	projectID string
	client    *api.Client
	watcher   *fsnotify.Watcher
	verbose   bool
	parser    *Parser
}

func NewWatcher(logPath, projectID string, client *api.Client, verbose bool) (*Watcher, error) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return nil, err
	}

	return &Watcher{
		logPath:   logPath,
		projectID: projectID,
		client:    client,
		watcher:   watcher,
		verbose:   verbose,
		parser:    NewParser(),
	}, nil
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

	// Send to PocketBase
	for _, fact := range facts {
		if err := w.client.CreateFact(w.projectID, fact); err != nil {
			log.Printf("Failed to create fact: %v", err)
		} else if w.verbose {
			log.Printf("Created fact: %s (%s)", fact.Content, fact.Type)
		}
	}

	// Update token count
	tokenCount := w.parser.CountTokens(conversation)
	if w.verbose {
		log.Printf("Token count: %d", tokenCount)
	}
}

type Conversation struct {
	Messages []Message `json:"messages"`
}

type Message struct {
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}
