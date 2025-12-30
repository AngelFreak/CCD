use notify_rust::{Notification, Timeout};
use std::path::PathBuf;

/// App icon name for notifications
const APP_ICON: &str = "com.github.claudecontexttracker";

/// Notification timeout (in milliseconds)
const NOTIFICATION_TIMEOUT: i32 = 5000;

/// Send a notification when new facts are extracted
pub fn notify_facts_extracted(project_name: &str, fact_count: usize) {
    let summary = format!("Facts Extracted: {}", project_name);
    let body = format!(
        "Extracted {} new fact{} from Claude Code conversation",
        fact_count,
        if fact_count == 1 { "" } else { "s" }
    );

    send_notification(&summary, &body);
}

/// Send a notification when token threshold is reached
pub fn notify_token_threshold(project_name: &str, current_tokens: usize, threshold: usize) {
    let summary = format!("⚠ Token Threshold: {}", project_name);
    let body = format!(
        "Context size is {} tokens (threshold: {})\nConsider compacting or exporting context",
        current_tokens, threshold
    );

    send_notification(&summary, &body);
}

/// Send a notification when monitoring starts
pub fn notify_monitoring_started(project_name: &str) {
    let summary = "Monitoring Started".to_string();
    let body = format!(
        "Now monitoring Claude Code logs for \"{}\"",
        project_name
    );

    send_notification(&summary, &body);
}

/// Send a notification when monitoring stops
pub fn notify_monitoring_stopped() {
    let summary = "Monitoring Stopped".to_string();
    let body = "Background monitoring has been disabled".to_string();

    send_notification(&summary, &body);
}

/// Send a notification when context is pulled to CLAUDE.md
pub fn notify_context_pulled(project_name: &str, output_path: Option<&PathBuf>) {
    let summary = format!("Context Pulled: {}", project_name);
    let body = if let Some(path) = output_path {
        format!("Exported to {}", path.display())
    } else {
        "Exported to CLAUDE.md".to_string()
    };

    send_notification(&summary, &body);
}

/// Send a notification when context is pushed
pub fn notify_context_pushed(project_name: &str, tokens: Option<usize>) {
    let summary = format!("Context Saved: {}", project_name);
    let body = if let Some(token_count) = tokens {
        format!("Session saved with {} tokens", token_count)
    } else {
        "Session summary saved".to_string()
    };

    send_notification(&summary, &body);
}

/// Send a notification when a project is created
pub fn notify_project_created(project_name: &str) {
    let summary = "Project Created".to_string();
    let body = format!("New project \"{}\" ready to track", project_name);

    send_notification(&summary, &body);
}

/// Send a notification when export completes
pub fn notify_export_complete(project_name: &str, format: &str) {
    let summary = format!("Export Complete: {}", project_name);
    let body = format!("Exported to {} format", format);

    send_notification(&summary, &body);
}

/// Send a notification for errors
pub fn notify_error(title: &str, message: &str) {
    let summary = format!("⚠ Error: {}", title);

    send_notification(&summary, message);
}

/// Helper function to send a desktop notification
fn send_notification(summary: &str, body: &str) {
    if let Err(e) = Notification::new()
        .summary(summary)
        .body(body)
        .icon(APP_ICON)
        .timeout(Timeout::Milliseconds(NOTIFICATION_TIMEOUT))
        .show()
    {
        log::warn!("Failed to send notification: {}", e);
    } else {
        log::debug!("Notification sent: {}", summary);
    }
}

/// Check if notifications are supported on this system
pub fn notifications_supported() -> bool {
    // Try to send a test notification
    Notification::new()
        .summary("")
        .body("")
        .timeout(Timeout::Milliseconds(0))
        .show()
        .is_ok()
}
