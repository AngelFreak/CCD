import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useProjects } from '../hooks/useProjects';
import ProjectCard from './ProjectCard';
import { Plus, Filter } from 'lucide-react';
import { ProjectStatus } from '../types';

export default function ProjectList() {
  const { projects, loading, error } = useProjects();
  const navigate = useNavigate();
  const [statusFilter, setStatusFilter] = useState<ProjectStatus | 'all'>('all');

  const filteredProjects = statusFilter === 'all'
    ? projects
    : projects.filter(p => p.status === statusFilter);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">Loading projects...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-red-500">Error: {error}</div>
      </div>
    );
  }

  return (
    <div>
      <div className="mb-6 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Filter size={20} className="text-gray-500" />
          <select
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value as ProjectStatus | 'all')}
            className="input py-1.5"
          >
            <option value="all">All Projects</option>
            <option value="active">Active</option>
            <option value="paused">Paused</option>
            <option value="idea">Ideas</option>
            <option value="archived">Archived</option>
          </select>
        </div>

        <button
          onClick={() => navigate('/projects/new')}
          className="btn btn-primary flex items-center gap-2"
        >
          <Plus size={18} />
          New Project
        </button>
      </div>

      {filteredProjects.length === 0 ? (
        <div className="text-center py-12">
          <p className="text-gray-500 mb-4">No projects found</p>
          <button
            onClick={() => navigate('/projects/new')}
            className="btn btn-primary"
          >
            Create your first project
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredProjects.map((project) => (
            <ProjectCard
              key={project.id}
              project={project}
              onClick={() => navigate(`/projects/${project.id}`)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
