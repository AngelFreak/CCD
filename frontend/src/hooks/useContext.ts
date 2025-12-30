import { useState, useEffect } from 'react';
import pb from '../lib/pocketbase';
import { ContextSection, SectionType } from '../types';

export function useContextSections(projectId: string) {
  const [sections, setSections] = useState<ContextSection[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!projectId) {
      setLoading(false);
      return;
    }

    loadSections();

    pb.collection('context_sections').subscribe('*', (e) => {
      if (e.record?.project === projectId) {
        loadSections();
      }
    });

    return () => {
      pb.collection('context_sections').unsubscribe('*');
    };
  }, [projectId]);

  async function loadSections() {
    try {
      const records = await pb.collection('context_sections').getFullList<ContextSection>({
        filter: `project="${projectId}"`,
        sort: 'order',
      });
      setSections(records);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load sections');
    } finally {
      setLoading(false);
    }
  }

  async function createSection(data: Omit<ContextSection, 'id' | 'created' | 'updated'>) {
    try {
      const record = await pb.collection('context_sections').create<ContextSection>(data);
      return record;
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to create section');
    }
  }

  async function updateSection(id: string, data: Partial<ContextSection>) {
    try {
      const record = await pb.collection('context_sections').update<ContextSection>(id, data);
      return record;
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to update section');
    }
  }

  async function deleteSection(id: string) {
    try {
      await pb.collection('context_sections').delete(id);
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to delete section');
    }
  }

  async function reorderSections(sectionIds: string[]) {
    try {
      await Promise.all(
        sectionIds.map((id, index) =>
          pb.collection('context_sections').update(id, { order: index })
        )
      );
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to reorder sections');
    }
  }

  return {
    sections,
    loading,
    error,
    createSection,
    updateSection,
    deleteSection,
    reorderSections,
    reload: loadSections,
  };
}
