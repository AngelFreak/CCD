import { useState } from 'react';
import { useContextSections } from '../hooks/useContext';
import SectionEditor from './SectionEditor';
import { Plus, Download, Copy, Check } from 'lucide-react';
import { SectionType } from '../types';
import { generateMarkdown, copyToClipboard, downloadMarkdown } from '../lib/markdown';
import { useProject } from '../hooks/useProjects';

interface ContextEditorProps {
  projectId: string;
}

export default function ContextEditor({ projectId }: ContextEditorProps) {
  const { project } = useProject(projectId);
  const { sections, loading, createSection, updateSection, deleteSection } = useContextSections(projectId);
  const [addingNew, setAddingNew] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleAddSection = async () => {
    const order = sections.length;
    await createSection({
      project: projectId,
      section_type: 'custom',
      title: 'New Section',
      content: '',
      order,
      auto_extracted: false,
    });
    setAddingNew(false);
  };

  const handleCopyToClipboard = async () => {
    if (!project) return;
    const markdown = generateMarkdown(project, sections);
    await copyToClipboard(markdown);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownload = () => {
    if (!project) return;
    const markdown = generateMarkdown(project, sections);
    downloadMarkdown(`${project.slug}-context.md`, markdown);
  };

  if (loading) {
    return <div className="text-gray-500">Loading context...</div>;
  }

  return (
    <div>
      <div className="mb-6 flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-900">Context Sections</h2>
        <div className="flex items-center gap-2">
          <button
            onClick={handleCopyToClipboard}
            className="btn btn-secondary flex items-center gap-2"
          >
            {copied ? <Check size={18} /> : <Copy size={18} />}
            {copied ? 'Copied!' : 'Copy to Clipboard'}
          </button>
          <button
            onClick={handleDownload}
            className="btn btn-secondary flex items-center gap-2"
          >
            <Download size={18} />
            Export Markdown
          </button>
          <button
            onClick={() => setAddingNew(true)}
            className="btn btn-primary flex items-center gap-2"
          >
            <Plus size={18} />
            Add Section
          </button>
        </div>
      </div>

      {addingNew && (
        <div className="mb-4">
          <SectionEditor
            section={{
              id: '',
              project: projectId,
              section_type: 'custom',
              title: 'New Section',
              content: '',
              order: sections.length,
              auto_extracted: false,
              created: new Date().toISOString(),
              updated: new Date().toISOString(),
            }}
            onSave={async (data) => {
              await createSection({
                project: projectId,
                section_type: data.section_type || 'custom',
                title: data.title || 'New Section',
                content: data.content || '',
                order: sections.length,
                auto_extracted: false,
              });
              setAddingNew(false);
            }}
            onDelete={async () => setAddingNew(false)}
            onCancel={() => setAddingNew(false)}
          />
        </div>
      )}

      {sections.length === 0 && !addingNew ? (
        <div className="text-center py-12 card">
          <p className="text-gray-500 mb-4">No context sections yet</p>
          <button onClick={() => setAddingNew(true)} className="btn btn-primary">
            Add your first section
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          {sections.map((section) => (
            <SectionEditor
              key={section.id}
              section={section}
              onSave={(data) => updateSection(section.id, data)}
              onDelete={() => deleteSection(section.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
