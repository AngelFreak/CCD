import { useState } from 'react';
import { ContextSection, SectionType } from '../types';
import { Trash2, Save, X } from 'lucide-react';

interface SectionEditorProps {
  section: ContextSection;
  onSave: (data: Partial<ContextSection>) => Promise<void>;
  onDelete: () => Promise<void>;
  onCancel?: () => void;
}

export default function SectionEditor({
  section,
  onSave,
  onDelete,
  onCancel,
}: SectionEditorProps) {
  const [title, setTitle] = useState(section.title);
  const [content, setContent] = useState(section.content);
  const [sectionType, setSectionType] = useState<SectionType>(section.section_type);
  const [saving, setSaving] = useState(false);

  const handleSave = async () => {
    setSaving(true);
    try {
      await onSave({ title, content, section_type: sectionType });
    } finally {
      setSaving(false);
    }
  };

  const hasChanges =
    title !== section.title ||
    content !== section.content ||
    sectionType !== section.section_type;

  return (
    <div className="card">
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-1">
          Section Type
        </label>
        <select
          value={sectionType}
          onChange={(e) => setSectionType(e.target.value as SectionType)}
          className="input"
        >
          <option value="architecture">Architecture</option>
          <option value="current_state">Current State</option>
          <option value="next_steps">Next Steps</option>
          <option value="gotchas">Gotchas</option>
          <option value="decisions">Decisions</option>
          <option value="custom">Custom</option>
        </select>
      </div>

      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-1">
          Title
        </label>
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          className="input"
          placeholder="Section title"
        />
      </div>

      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-1">
          Content
        </label>
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          className="textarea"
          rows={10}
          placeholder="Section content (supports Markdown)"
        />
      </div>

      <div className="flex items-center justify-between">
        <button
          onClick={onDelete}
          className="btn bg-red-100 text-red-700 hover:bg-red-200 flex items-center gap-2"
        >
          <Trash2 size={16} />
          Delete
        </button>

        <div className="flex items-center gap-2">
          {onCancel && (
            <button onClick={onCancel} className="btn btn-secondary flex items-center gap-2">
              <X size={16} />
              Cancel
            </button>
          )}
          <button
            onClick={handleSave}
            disabled={!hasChanges || saving}
            className="btn btn-primary flex items-center gap-2 disabled:opacity-50"
          >
            <Save size={16} />
            {saving ? 'Saving...' : 'Save'}
          </button>
        </div>
      </div>
    </div>
  );
}
