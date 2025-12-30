use crate::db::Repository;
use crate::models::{ProjectPayload, ProjectStatus, SessionPayload};
use crate::utils::generate_claude_md;
use anyhow::{bail, Context, Result};
use std::path::Path;

/// Execute the pull command
pub fn pull_command(repository: &Repository, project: &str, output: Option<String>) -> Result<()> {
    // Find project by name or ID
    let proj = find_project(repository, project)?;

    // Get context sections
    let sections = repository.list_context_sections(&proj.id)?;

    // Generate markdown
    let markdown = generate_claude_md(&proj, &sections);

    // Write to file
    let output_path = output.unwrap_or_else(|| "./CLAUDE.md".to_string());
    std::fs::write(&output_path, markdown)
        .context("Failed to write CLAUDE.md")?;

    println!("✓ Pulled context for '{}' to {}", proj.name, output_path);
    println!("  {} sections", sections.len());

    // Send notification
    let path = Path::new(&output_path).to_path_buf();
    crate::notifications::notify_context_pulled(&proj.name, Some(&path));

    Ok(())
}

/// Execute the push command
pub fn push_command(
    repository: &Repository,
    project: &str,
    summary: String,
    tokens: Option<i64>,
) -> Result<()> {
    let proj = find_project(repository, project)?;

    let payload = SessionPayload {
        project: proj.id.clone(),
        summary,
        facts_extracted: Some(0),
        token_count: tokens,
        session_start: Some(chrono::Utc::now()),
        session_end: Some(chrono::Utc::now()),
    };

    let session = repository.create_session(payload)?;

    println!("✓ Pushed session for '{}'", proj.name);
    println!("  Session ID: {}", session.id);
    if let Some(t) = tokens {
        println!("  Tokens: {}", t);
    }

    // Send notification
    crate::notifications::notify_context_pushed(&proj.name, tokens.map(|t| t as usize));

    Ok(())
}

/// Execute the status command
pub fn status_command(repository: &Repository, project: Option<String>) -> Result<()> {
    match project {
        Some(proj_name) => {
            let proj = find_project(repository, &proj_name)?;
            show_project_status(repository, &proj)?;
        }
        None => {
            let projects = repository.list_projects(Some(ProjectStatus::Active))?;
            if projects.is_empty() {
                println!("No active projects");
            } else {
                println!("Active Projects:");
                for proj in projects {
                    println!("\n{}", proj.name);
                    show_project_status(repository, &proj)?;
                }
            }
        }
    }

    Ok(())
}

fn show_project_status(repository: &Repository, proj: &crate::models::Project) -> Result<()> {
    let sessions = repository.list_sessions(&proj.id)?;
    let facts = repository.list_facts(&proj.id, false)?;

    println!("  Status: {}", proj.status);
    println!("  Sessions: {}", sessions.len());
    println!("  Facts: {}", facts.len());

    if let Some(latest) = sessions.first() {
        println!("  Latest: {} tokens", latest.token_count);
        println!("  Usage: {:.1}%", latest.token_percentage());
    }

    Ok(())
}

/// Execute the list command
pub fn list_command(repository: &Repository, status: Option<String>) -> Result<()> {
    let status_filter = status.as_ref().map(|s| match s.as_str() {
        "active" => ProjectStatus::Active,
        "paused" => ProjectStatus::Paused,
        "idea" => ProjectStatus::Idea,
        "archived" => ProjectStatus::Archived,
        _ => ProjectStatus::Active,
    });

    let projects = repository.list_projects(status_filter)?;

    if projects.is_empty() {
        println!("No projects found");
        return Ok(());
    }

    println!("Projects:");
    for proj in projects {
        println!("  {} [{}]", proj.name, proj.status);
        if let Some(desc) = &proj.description {
            println!("    {}", desc);
        }
        if !proj.tech_stack.is_empty() {
            println!("    Tech: {}", proj.tech_stack.join(", "));
        }
    }

    Ok(())
}

/// Execute the new command
pub fn new_command(
    repository: &Repository,
    name: String,
    repo: Option<String>,
    tech: Option<String>,
    description: Option<String>,
) -> Result<()> {
    let tech_stack = tech
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let payload = ProjectPayload {
        name: name.clone(),
        slug: name.to_lowercase().replace(' ', "-"),
        repo_path: repo,
        status: ProjectStatus::Active,
        priority: 0,
        tech_stack,
        description,
    };

    let project = repository.create_project(payload)?;

    println!("✓ Created project '{}'", project.name);
    println!("  ID: {}", project.id);

    // Send notification
    crate::notifications::notify_project_created(&project.name);

    Ok(())
}

/// Execute the diff command
pub fn diff_command(
    repository: &Repository,
    project: &str,
    from: Option<String>,
    to: Option<String>,
) -> Result<()> {
    let proj = find_project(repository, project)?;
    let sessions = repository.list_sessions(&proj.id)?;

    if sessions.len() < 2 {
        println!("Need at least 2 sessions to compare");
        return Ok(());
    }

    let from_session = match from {
        Some(id) => repository.get_session(&id)?,
        None => sessions.get(1).context("No previous session")?.clone(),
    };

    let to_session = match to {
        Some(id) => repository.get_session(&id)?,
        None => sessions.first().context("No latest session")?.clone(),
    };

    println!("Diff: {} -> {}", from_session.id, to_session.id);
    println!("\nFrom: {}", from_session.summary);
    println!("  {} tokens, {} facts", from_session.token_count, from_session.facts_extracted);

    println!("\nTo: {}", to_session.summary);
    println!("  {} tokens, {} facts", to_session.token_count, to_session.facts_extracted);

    let token_diff = to_session.token_count - from_session.token_count;
    let fact_diff = to_session.facts_extracted - from_session.facts_extracted;

    println!("\nChanges:");
    println!("  Tokens: {:+}", token_diff);
    println!("  Facts: {:+}", fact_diff);

    Ok(())
}

/// Find project by name or ID
pub fn find_project(repository: &Repository, name_or_id: &str) -> Result<crate::models::Project> {
    // Try by ID first
    if let Ok(proj) = repository.get_project(name_or_id) {
        return Ok(proj);
    }

    // Try by name
    let projects = repository.list_projects(None)?;
    for proj in projects {
        if proj.name.to_lowercase() == name_or_id.to_lowercase() {
            return Ok(proj);
        }
    }

    bail!("Project not found: {}", name_or_id)
}
