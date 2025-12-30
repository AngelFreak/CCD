export type ProjectStatus = 'active' | 'paused' | 'idea' | 'archived';

export type SectionType = 'architecture' | 'current_state' | 'next_steps' | 'gotchas' | 'decisions' | 'custom';

export type FactType = 'decision' | 'blocker' | 'file_change' | 'dependency' | 'todo' | 'insight';

export interface Project {
  id: string;
  name: string;
  slug: string;
  repo_path: string;
  status: ProjectStatus;
  priority: number;
  tech_stack: string[];
  description?: string;
  created: string;
  updated: string;
}

export interface ContextSection {
  id: string;
  project: string;
  section_type: SectionType;
  title: string;
  content: string;
  order: number;
  auto_extracted: boolean;
  created: string;
  updated: string;
}

export interface SessionHistory {
  id: string;
  project: string;
  summary: string;
  facts_extracted?: Record<string, unknown>;
  token_count?: number;
  session_start: string;
  session_end?: string;
  created: string;
}

export interface ExtractedFact {
  id: string;
  project: string;
  session?: string;
  fact_type: FactType;
  content: string;
  importance: number;
  stale: boolean;
  created: string;
}
