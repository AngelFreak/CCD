use crate::db::Repository;
use crate::models::{SessionHistory, SessionPayload};
use crate::monitor::{FactExtractor, ImportanceScorer, StalenessDetector, parse_conversation_log};
use anyhow::{Context, Result};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

/// Claude Code log monitor
pub struct LogMonitor {
    project_id: String,
    repository: Repository,
    logs_dir: PathBuf,
}

impl LogMonitor {
    /// Create a new log monitor
    pub fn new(project_id: String, repository: Repository, logs_dir: Option<PathBuf>) -> Result<Self> {
        let logs_dir = logs_dir.unwrap_or_else(Self::default_logs_dir);

        if !logs_dir.exists() {
            log::warn!("Claude Code logs directory does not exist: {}", logs_dir.display());
        }

        Ok(Self {
            project_id,
            repository,
            logs_dir,
        })
    }

    /// Get default Claude Code logs directory
    fn default_logs_dir() -> PathBuf {
        if let Some(home) = home::home_dir() {
            home.join(".claude").join("logs")
        } else {
            PathBuf::from("./logs")
        }
    }

    /// Start monitoring (blocking)
    pub fn start_monitoring(&self) -> Result<()> {
        log::info!("Starting log monitoring for project: {}", self.project_id);
        log::info!("Watching directory: {}", self.logs_dir.display());

        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            tx,
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;

        watcher.watch(&self.logs_dir, RecursiveMode::Recursive)?;

        log::info!("File watcher initialized successfully");

        // Process existing files first
        self.process_existing_files()?;

        // Watch for new files
        for res in rx {
            match res {
                Ok(event) => self.handle_event(event),
                Err(e) => log::error!("Watch error: {}", e),
            }
        }

        Ok(())
    }

    /// Process all existing log files
    fn process_existing_files(&self) -> Result<()> {
        log::info!("Processing existing log files...");

        if !self.logs_dir.exists() {
            log::warn!("Logs directory does not exist yet");
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.logs_dir)?;
        let mut count = 0;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Err(e) = self.process_log_file(&path) {
                    log::warn!("Failed to process {}: {}", path.display(), e);
                } else {
                    count += 1;
                }
            }
        }

        log::info!("Processed {} existing log files", count);
        Ok(())
    }

    /// Handle file system event
    fn handle_event(&self, event: Event) {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in event.paths {
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        log::info!("New/modified log file detected: {}", path.display());
                        if let Err(e) = self.process_log_file(&path) {
                            log::error!("Failed to process log file: {}", e);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Process a single log file
    fn process_log_file(&self, path: &Path) -> Result<()> {
        log::debug!("Processing log file: {}", path.display());

        let content = std::fs::read_to_string(path)
            .context("Failed to read log file")?;

        let log = parse_conversation_log(&content)
            .context("Failed to parse conversation log")?;

        // Create or update session
        let session_id = self.create_session(&log)?;

        // Extract facts from all messages
        let extractor = FactExtractor::new(self.project_id.clone());
        let mut total_facts = 0;

        for message in &log.messages {
            if message.role == "assistant" {
                let facts = extractor.extract_from_message(&message.content, Some(session_id.clone()));

                for fact in facts {
                    match self.repository.create_fact(fact) {
                        Ok(_) => total_facts += 1,
                        Err(e) => log::warn!("Failed to save fact: {}", e),
                    }
                }
            }
        }

        log::info!("Extracted {} facts from session {}", total_facts, session_id);

        // Update session with fact count
        if let Ok(mut session) = self.repository.get_session(&session_id) {
            session.facts_extracted = total_facts;
            let payload = SessionPayload::from(&session);
            let _ = self.repository.update_session(&session_id, payload);
        }

        // Update staleness for existing facts
        self.update_stale_facts()?;

        Ok(())
    }

    /// Create a session record for this conversation
    fn create_session(&self, log: &crate::monitor::extractor::ConversationLog) -> Result<String> {
        let summary = if log.messages.is_empty() {
            "Empty conversation".to_string()
        } else {
            // Use first user message as summary
            log.messages.iter()
                .find(|m| m.role == "user")
                .map(|m| {
                    let content = &m.content;
                    if content.len() > 100 {
                        format!("{}...", &content[..97])
                    } else {
                        content.clone()
                    }
                })
                .unwrap_or_else(|| "Conversation".to_string())
        };

        let token_count = log.estimate_tokens();

        let payload = SessionPayload {
            project: self.project_id.clone(),
            summary,
            facts_extracted: Some(0),
            token_count: Some(token_count),
            session_start: Some(chrono::Utc::now()),
            session_end: None,
        };

        let session = self.repository.create_session(payload)?;
        Ok(session.id)
    }

    /// Update staleness for all facts
    fn update_stale_facts(&self) -> Result<()> {
        let facts = self.repository.list_facts(&self.project_id, false)?;

        for fact in facts {
            if StalenessDetector::is_stale(&fact) {
                log::debug!("Marking fact {} as stale", fact.id);
                let _ = self.repository.mark_fact_stale(&fact.id);
            }
        }

        Ok(())
    }
}

/// Background monitoring thread
pub fn start_background_monitor(
    project_id: String,
    repository: Repository,
    logs_dir: Option<PathBuf>,
) -> Result<std::thread::JoinHandle<()>> {
    let handle = std::thread::spawn(move || {
        log::info!("Background monitor thread started");

        match LogMonitor::new(project_id, repository, logs_dir) {
            Ok(monitor) => {
                if let Err(e) = monitor.start_monitoring() {
                    log::error!("Monitor error: {}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to create monitor: {}", e);
            }
        }
    });

    Ok(handle)
}
