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
	logPath    = flag.String("logs", getDefaultLogPath(), "Claude Code logs directory")
	verbose    = flag.Bool("v", false, "Verbose logging")
)

func main() {
	flag.Parse()

	if *projectID == "" {
		log.Fatal("Project ID is required. Use -project flag.")
	}

	// Initialize PocketBase client
	client := api.NewClient(*pbURL)

	// Verify project exists
	if err := client.VerifyProject(*projectID); err != nil {
		log.Fatalf("Failed to verify project: %v", err)
	}

	log.Printf("Starting Claude Context Tracker daemon")
	log.Printf("PocketBase URL: %s", *pbURL)
	log.Printf("Project ID: %s", *projectID)
	log.Printf("Logs path: %s", *logPath)

	// Create watcher
	watcher, err := monitor.NewWatcher(*logPath, *projectID, client, *verbose)
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
