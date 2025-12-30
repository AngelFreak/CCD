import { Project } from '../types';
import { FolderOpen, Calendar, Star } from 'lucide-react';

interface ProjectCardProps {
  project: Project;
  onClick: () => void;
}

export default function ProjectCard({ project, onClick }: ProjectCardProps) {
  const statusColors = {
    active: 'bg-green-100 text-green-800',
    paused: 'bg-yellow-100 text-yellow-800',
    idea: 'bg-blue-100 text-blue-800',
    archived: 'bg-gray-100 text-gray-800',
  };

  return (
    <div
      onClick={onClick}
      className="card hover:shadow-md cursor-pointer transition-shadow"
    >
      <div className="flex items-start justify-between mb-3">
        <h3 className="text-lg font-semibold text-gray-900">{project.name}</h3>
        <span className={`px-2 py-1 rounded text-xs font-medium ${statusColors[project.status]}`}>
          {project.status}
        </span>
      </div>

      {project.description && (
        <p className="text-sm text-gray-600 mb-3 line-clamp-2">{project.description}</p>
      )}

      <div className="flex items-center gap-4 text-sm text-gray-500">
        <div className="flex items-center gap-1">
          <FolderOpen size={14} />
          <span className="font-mono text-xs">{project.slug}</span>
        </div>
        <div className="flex items-center gap-1">
          <Star size={14} />
          <span>{project.priority}</span>
        </div>
      </div>

      {project.tech_stack && project.tech_stack.length > 0 && (
        <div className="flex flex-wrap gap-1 mt-3">
          {project.tech_stack.slice(0, 5).map((tech) => (
            <span
              key={tech}
              className="px-2 py-0.5 bg-gray-100 text-gray-700 text-xs rounded"
            >
              {tech}
            </span>
          ))}
          {project.tech_stack.length > 5 && (
            <span className="px-2 py-0.5 text-gray-500 text-xs">
              +{project.tech_stack.length - 5} more
            </span>
          )}
        </div>
      )}

      <div className="mt-3 pt-3 border-t border-gray-100 text-xs text-gray-400">
        Updated {new Date(project.updated).toLocaleDateString()}
      </div>
    </div>
  );
}
