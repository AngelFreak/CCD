package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/signal"
	"path/filepath"
	"syscall"

	"github.com/angelfreak/ccd/daemon/api"
	"github.com/angelfreak/ccd/daemon/monitor"
)

var (
	pbURL      = flag.String("pb-url", "http://localhost:8090", "PocketBase URL")
	projectID  = flag.String("project", "", "Project ID to track")
	repoPath   = flag.String("repo", "", "Repository path for ledger storage")
	logPath    = flag.String("logs", getDefaultLogPath(), "Claude Code logs directory")
	verbose    = flag.Bool("v", false, "Verbose logging")
	smartMode  = flag.Bool("smart", true, "Enable smart context features (importance scoring, compression)")
	compactThreshold = flag.Int("compact-threshold", 170000, "Token threshold for pre-compact handoff")
)

func main() {
	flag.Parse()

	if *projectID == "" {
		log.Fatal("Project ID is required. Use -project flag.")
	}

	// Initialize PocketBase client
	client := api.NewClient(*pbURL)

	// Verify project exists and get repo path
	project, err := client.GetProject(*projectID)
	if err != nil {
		log.Fatalf("Failed to verify project: %v", err)
	}

	// Use repo path from project if not specified
	if *repoPath == "" {
		*repoPath = project.RepoPath
	}

	log.Printf("Starting Claude Context Tracker daemon")
	log.Printf("PocketBase URL: %s", *pbURL)
	log.Printf("Project ID: %s", *projectID)
	log.Printf("Repo Path: %s", *repoPath)
	log.Printf("Logs path: %s", *logPath)
	log.Printf("Smart mode: %v", *smartMode)
	log.Printf("Compact threshold: %d tokens", *compactThreshold)

	// Create watcher with enhanced features
	config := monitor.WatcherConfig{
		LogPath:          *logPath,
		ProjectID:        *projectID,
		RepoPath:         *repoPath,
		Client:           client,
		Verbose:          *verbose,
		SmartMode:        *smartMode,
		CompactThreshold: *compactThreshold,
	}

	watcher, err := monitor.NewWatcherWithConfig(config)
	if err != nil {
		log.Fatalf("Failed to create watcher: %v", err)
	}

	// Start watching
	if err := watcher.Start(); err != nil {
		log.Fatalf("Failed to start watcher: %v", err)
	}

	log.Println("Daemon started successfully. Press Ctrl+C to stop.")

	// Wait for interrupt signal
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan

	log.Println("Shutting down...")
	watcher.Stop()
}

func getDefaultLogPath() string {
	home, err := os.UserHomeDir()
	if err != nil {
		return ""
	}

	// Try common Claude Code log locations
	paths := []string{
		filepath.Join(home, ".claude", "logs"),
		filepath.Join(home, ".config", "claude", "logs"),
		filepath.Join(home, "Library", "Application Support", "Claude", "logs"),
	}

	for _, path := range paths {
		if _, err := os.Stat(path); err == nil {
			return path
		}
	}

	return ""
}
