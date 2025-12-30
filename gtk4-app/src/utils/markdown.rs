use crate::models::{ContextSection, Project};
use anyhow::Result;
use std::path::Path;

/// Generate markdown content from project and sections
pub fn generate_claude_md(project: &Project, sections: &[ContextSection]) -> String {
    let mut markdown = String::new();

    // Header
    markdown.push_str(&format!("# {}\n\n", project.name));

    // Project overview section
    markdown.push_str("## Project Overview\n");
    if let Some(desc) = &project.description {
        markdown.push_str(desc);
        markdown.push_str("\n\n");
    }

    // Tech stack
    if !project.tech_stack.is_empty() {
        markdown.push_str("## Tech Stack\n");
        for tech in &project.tech_stack {
            markdown.push_str(&format!("- {}\n", tech));
        }
        markdown.push('\n');
    }

    // Sorted sections by order
    let mut sorted_sections = sections.to_vec();
    sorted_sections.sort_by_key(|s| s.order);

    // Add each section
    for section in sorted_sections {
        markdown.push_str(&section.to_markdown());
    }

    // Footer
    markdown.push_str("---\n");
    markdown.push_str(&format!("_Last updated: {}_\n", chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")));

    markdown
}

/// Save markdown content to a file
pub fn save_markdown_to_file(content: &str, path: &Path) -> Result<()> {
    std::fs::write(path, content)?;
    Ok(())
}

/// Copy markdown content to clipboard
pub fn copy_to_clipboard(content: &str, clipboard: &gtk::gdk::Clipboard) {
    clipboard.set_text(content);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{SectionType, ProjectStatus};

    #[test]
    fn test_generate_claude_md() {
        let project = Project {
            id: "test".to_string(),
            name: "Test Project".to_string(),
            slug: "test-project".to_string(),
            repo_path: None,
            status: ProjectStatus::Active,
            priority: 0,
            tech_stack: vec!["Rust".to_string(), "GTK4".to_string()],
            description: Some("A test project".to_string()),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
        };

        let sections = vec![
            ContextSection {
                id: "1".to_string(),
                project: "test".to_string(),
                section_type: SectionType::Architecture,
                title: "Architecture".to_string(),
                content: "Test architecture content".to_string(),
                order: 0,
                auto_extracted: false,
                created: chrono::Utc::now(),
                updated: chrono::Utc::now(),
            },
        ];

        let md = generate_claude_md(&project, &sections);

        assert!(md.contains("# Test Project"));
        assert!(md.contains("## Tech Stack"));
        assert!(md.contains("- Rust"));
        assert!(md.contains("## Architecture"));
        assert!(md.contains("Test architecture content"));
    }
}
