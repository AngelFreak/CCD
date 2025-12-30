// Initial schema migration for Claude Context Tracker
migrate((db) => {
  // Projects collection
  const projectsCollection = new Collection({
    name: 'projects',
    type: 'base',
    schema: [
      {
        name: 'name',
        type: 'text',
        required: true,
      },
      {
        name: 'slug',
        type: 'text',
        required: true,
        options: {
          pattern: '^[a-z0-9-]+$',
        },
      },
      {
        name: 'repo_path',
        type: 'text',
        required: true,
      },
      {
        name: 'status',
        type: 'select',
        required: true,
        options: {
          maxSelect: 1,
          values: ['active', 'paused', 'idea', 'archived'],
        },
      },
      {
        name: 'priority',
        type: 'number',
        required: true,
      },
      {
        name: 'tech_stack',
        type: 'json',
        required: false,
      },
      {
        name: 'description',
        type: 'text',
        required: false,
      },
    ],
    indexes: [
      'CREATE UNIQUE INDEX idx_slug ON projects(slug)',
    ],
  });

  // Context sections collection
  const contextSectionsCollection = new Collection({
    name: 'context_sections',
    type: 'base',
    schema: [
      {
        name: 'project',
        type: 'relation',
        required: true,
        options: {
          collectionId: projectsCollection.id,
          cascadeDelete: true,
        },
      },
      {
        name: 'section_type',
        type: 'select',
        required: true,
        options: {
          maxSelect: 1,
          values: ['architecture', 'current_state', 'next_steps', 'gotchas', 'decisions', 'custom'],
        },
      },
      {
        name: 'title',
        type: 'text',
        required: true,
      },
      {
        name: 'content',
        type: 'text',
        required: true,
      },
      {
        name: 'order',
        type: 'number',
        required: true,
      },
      {
        name: 'auto_extracted',
        type: 'bool',
        required: true,
      },
    ],
    indexes: [
      'CREATE INDEX idx_project ON context_sections(project)',
    ],
  });

  // Session history collection
  const sessionHistoryCollection = new Collection({
    name: 'session_history',
    type: 'base',
    schema: [
      {
        name: 'project',
        type: 'relation',
        required: true,
        options: {
          collectionId: projectsCollection.id,
          cascadeDelete: true,
        },
      },
      {
        name: 'summary',
        type: 'text',
        required: true,
      },
      {
        name: 'facts_extracted',
        type: 'json',
        required: false,
      },
      {
        name: 'token_count',
        type: 'number',
        required: false,
      },
      {
        name: 'session_start',
        type: 'date',
        required: true,
      },
      {
        name: 'session_end',
        type: 'date',
        required: false,
      },
    ],
    indexes: [
      'CREATE INDEX idx_project_session ON session_history(project)',
    ],
  });

  // Extracted facts collection
  const extractedFactsCollection = new Collection({
    name: 'extracted_facts',
    type: 'base',
    schema: [
      {
        name: 'project',
        type: 'relation',
        required: true,
        options: {
          collectionId: projectsCollection.id,
          cascadeDelete: true,
        },
      },
      {
        name: 'session',
        type: 'relation',
        required: false,
        options: {
          collectionId: sessionHistoryCollection.id,
          cascadeDelete: true,
        },
      },
      {
        name: 'fact_type',
        type: 'select',
        required: true,
        options: {
          maxSelect: 1,
          values: ['decision', 'blocker', 'file_change', 'dependency', 'todo', 'insight'],
        },
      },
      {
        name: 'content',
        type: 'text',
        required: true,
      },
      {
        name: 'importance',
        type: 'number',
        required: true,
        options: {
          min: 1,
          max: 5,
        },
      },
      {
        name: 'stale',
        type: 'bool',
        required: true,
      },
    ],
    indexes: [
      'CREATE INDEX idx_project_facts ON extracted_facts(project)',
      'CREATE INDEX idx_session_facts ON extracted_facts(session)',
    ],
  });

  return Dao(db).saveCollection(projectsCollection);
}, (db) => {
  // Revert
  const dao = new Dao(db);
  dao.deleteCollection('extracted_facts');
  dao.deleteCollection('session_history');
  dao.deleteCollection('context_sections');
  dao.deleteCollection('projects');
});
