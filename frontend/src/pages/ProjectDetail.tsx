import { useParams, useNavigate } from 'react-router-dom';
import { useState } from 'react';
import { useProject } from '../hooks/useProjects';
import ContextEditor from '../components/ContextEditor';
import SessionMonitor from '../components/SessionMonitor';
import FactsList from '../components/FactsList';
import DiffViewer from '../components/DiffViewer';
import CompressedContext from '../components/CompressedContext';
import { ArrowLeft, Settings, FileText, GitCompare, Zap } from 'lucide-react';

export default function ProjectDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { project, loading, error } = useProject(id!);
  const [activeTab, setActiveTab] = useState<'context' | 'diff' | 'compressed'>('context');

  if (loading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-gray-500">Loading project...</div>
      </div>
    );
  }

  if (error || !project) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-red-500">Error: {error || 'Project not found'}</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="bg-white border-b border-gray-200 sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <button
                onClick={() => navigate('/')}
                className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
              >
                <ArrowLeft size={20} />
              </button>
              <div>
                <h1 className="text-2xl font-bold text-gray-900">{project.name}</h1>
                <p className="text-sm text-gray-600">{project.repo_path}</p>
              </div>
            </div>
            <button className="p-2 hover:bg-gray-100 rounded-lg transition-colors">
              <Settings size={20} />
            </button>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 py-8">
        {/* Tab Navigation */}
        <div className="mb-6 flex items-center gap-2 border-b border-gray-200">
          <button
            onClick={() => setActiveTab('context')}
            className={`px-4 py-2 flex items-center gap-2 border-b-2 transition-colors ${
              activeTab === 'context'
                ? 'border-blue-600 text-blue-600'
                : 'border-transparent text-gray-600 hover:text-gray-900'
            }`}
          >
            <FileText size={18} />
            Context Editor
          </button>
          <button
            onClick={() => setActiveTab('diff')}
            className={`px-4 py-2 flex items-center gap-2 border-b-2 transition-colors ${
              activeTab === 'diff'
                ? 'border-blue-600 text-blue-600'
                : 'border-transparent text-gray-600 hover:text-gray-900'
            }`}
          >
            <GitCompare size={18} />
            Session Diffs
          </button>
          <button
            onClick={() => setActiveTab('compressed')}
            className={`px-4 py-2 flex items-center gap-2 border-b-2 transition-colors ${
              activeTab === 'compressed'
                ? 'border-blue-600 text-blue-600'
                : 'border-transparent text-gray-600 hover:text-gray-900'
            }`}
          >
            <Zap size={18} />
            Compressed View
          </button>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2">
            {activeTab === 'context' && <ContextEditor projectId={project.id} />}
            {activeTab === 'diff' && <DiffViewer projectId={project.id} />}
            {activeTab === 'compressed' && <CompressedContext projectId={project.id} />}
          </div>

          <div className="space-y-6">
            <SessionMonitor projectId={project.id} />
            <FactsList projectId={project.id} />
          </div>
        </div>
      </div>
    </div>
  );
}
