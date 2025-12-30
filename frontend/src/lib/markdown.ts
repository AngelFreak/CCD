import { Project, ContextSection } from '../types';

export function generateMarkdown(project: Project, sections: ContextSection[]): string {
  const sortedSections = [...sections].sort((a, b) => a.order - b.order);

  let markdown = `# ${project.name}\n\n`;

  if (project.description) {
    markdown += `${project.description}\n\n`;
  }

  markdown += `## Project Info\n`;
  markdown += `- **Status**: ${project.status}\n`;
  markdown += `- **Priority**: ${project.priority}\n`;
  markdown += `- **Repo Path**: ${project.repo_path}\n`;

  if (project.tech_stack && project.tech_stack.length > 0) {
    markdown += `- **Tech Stack**: ${project.tech_stack.join(', ')}\n`;
  }

  markdown += `\n`;

  for (const section of sortedSections) {
    markdown += `## ${section.title}\n\n`;
    markdown += `${section.content}\n\n`;
  }

  return markdown;
}

export function copyToClipboard(text: string): Promise<void> {
  return navigator.clipboard.writeText(text);
}

export function downloadMarkdown(filename: string, content: string): void {
  const blob = new Blob([content], { type: 'text/markdown' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}
