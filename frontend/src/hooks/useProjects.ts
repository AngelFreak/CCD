import { useState, useEffect } from 'react';
import pb from '../lib/pocketbase';
import { Project, ProjectStatus } from '../types';

export function useProjects() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadProjects();

    // Subscribe to realtime updates
    pb.collection('projects').subscribe('*', () => {
      loadProjects();
    });

    return () => {
      pb.collection('projects').unsubscribe('*');
    };
  }, []);

  async function loadProjects() {
    try {
      const records = await pb.collection('projects').getFullList<Project>({
        sort: '-priority,-updated',
      });
      setProjects(records);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load projects');
    } finally {
      setLoading(false);
    }
  }

  async function createProject(data: Omit<Project, 'id' | 'created' | 'updated'>) {
    try {
      const record = await pb.collection('projects').create<Project>(data);
      return record;
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to create project');
    }
  }

  async function updateProject(id: string, data: Partial<Project>) {
    try {
      const record = await pb.collection('projects').update<Project>(id, data);
      return record;
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to update project');
    }
  }

  async function deleteProject(id: string) {
    try {
      await pb.collection('projects').delete(id);
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to delete project');
    }
  }

  return {
    projects,
    loading,
    error,
    createProject,
    updateProject,
    deleteProject,
    reload: loadProjects,
  };
}

export function useProject(id: string) {
  const [project, setProject] = useState<Project | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!id) {
      setLoading(false);
      return;
    }

    loadProject();

    pb.collection('projects').subscribe(id, () => {
      loadProject();
    });

    return () => {
      pb.collection('projects').unsubscribe(id);
    };
  }, [id]);

  async function loadProject() {
    try {
      const record = await pb.collection('projects').getOne<Project>(id);
      setProject(record);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load project');
    } finally {
      setLoading(false);
    }
  }

  return { project, loading, error, reload: loadProject };
}
