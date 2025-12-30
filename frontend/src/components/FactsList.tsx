import { useState, useEffect } from 'react';
import pb from '../lib/pocketbase';
import { ExtractedFact, FactType } from '../types';
import { AlertCircle, FileCode, Lightbulb, Ban, Package, CheckSquare } from 'lucide-react';

interface FactsListProps {
  projectId: string;
}

export default function FactsList({ projectId }: FactsListProps) {
  const [facts, setFacts] = useState<ExtractedFact[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadFacts();
  }, [projectId]);

  async function loadFacts() {
    try {
      const records = await pb.collection('extracted_facts').getFullList<ExtractedFact>({
        filter: `project="${projectId}" && stale=false`,
        sort: '-importance,-created',
      });
      setFacts(records);
    } catch (err) {
      console.error('Failed to load facts:', err);
    } finally {
      setLoading(false);
    }
  }

  const getFactIcon = (type: FactType) => {
    switch (type) {
      case 'decision':
        return <CheckSquare size={16} />;
      case 'blocker':
        return <Ban size={16} />;
      case 'file_change':
        return <FileCode size={16} />;
      case 'dependency':
        return <Package size={16} />;
      case 'todo':
        return <AlertCircle size={16} />;
      case 'insight':
        return <Lightbulb size={16} />;
    }
  };

  const getFactColor = (type: FactType) => {
    switch (type) {
      case 'decision':
        return 'bg-blue-100 text-blue-800';
      case 'blocker':
        return 'bg-red-100 text-red-800';
      case 'file_change':
        return 'bg-purple-100 text-purple-800';
      case 'dependency':
        return 'bg-green-100 text-green-800';
      case 'todo':
        return 'bg-yellow-100 text-yellow-800';
      case 'insight':
        return 'bg-indigo-100 text-indigo-800';
    }
  };

  if (loading) {
    return <div className="text-gray-500">Loading facts...</div>;
  }

  return (
    <div className="card">
      <h3 className="text-lg font-semibold mb-4">Extracted Facts</h3>

      {facts.length === 0 ? (
        <p className="text-gray-500 text-sm">No facts extracted yet</p>
      ) : (
        <div className="space-y-2">
          {facts.map((fact) => (
            <div
              key={fact.id}
              className="flex items-start gap-3 p-3 bg-gray-50 rounded-lg"
            >
              <div className={`p-1.5 rounded ${getFactColor(fact.fact_type)}`}>
                {getFactIcon(fact.fact_type)}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-xs font-medium text-gray-500 uppercase">
                    {fact.fact_type}
                  </span>
                  <span className="text-xs text-gray-400">
                    {'★'.repeat(fact.importance)}
                  </span>
                </div>
                <p className="text-sm text-gray-700">{fact.content}</p>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
