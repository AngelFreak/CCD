import { useState, useEffect } from 'react';
import pb from '../lib/pocketbase';
import { ExtractedFact } from '../types';
import { Zap, Star } from 'lucide-react';

interface CompressedContextProps {
  projectId: string;
}

export default function CompressedContext({ projectId }: CompressedContextProps) {
  const [topFacts, setTopFacts] = useState<ExtractedFact[]>([]);
  const [loading, setLoading] = useState(true);
  const [maxPerType, setMaxPerType] = useState(5);

  useEffect(() => {
    loadCompressedContext();
  }, [projectId, maxPerType]);

  async function loadCompressedContext() {
    try {
      // Get all non-stale facts
      const facts = await pb.collection('extracted_facts').getFullList<ExtractedFact>({
        filter: `project="${projectId}" && stale=false`,
        sort: '-importance,-created',
      });

      // Group by type and take top N per type
      const grouped = facts.reduce((acc, fact) => {
        if (!acc[fact.fact_type]) {
          acc[fact.fact_type] = [];
        }
        if (acc[fact.fact_type].length < maxPerType) {
          acc[fact.fact_type].push(fact);
        }
        return acc;
      }, {} as Record<string, ExtractedFact[]>);

      // Flatten back to array
      const compressed = Object.values(grouped).flat();

      setTopFacts(compressed);
    } catch (err) {
      console.error('Failed to load compressed context:', err);
    } finally {
      setLoading(false);
    }
  }

  const factTypeLabels: Record<string, string> = {
    decision: 'Decisions',
    blocker: 'Blockers',
    todo: 'TODOs',
    file_change: 'File Changes',
    dependency: 'Dependencies',
    insight: 'Insights',
  };

  const factTypeColors: Record<string, string> = {
    decision: 'bg-blue-100 text-blue-800',
    blocker: 'bg-red-100 text-red-800',
    todo: 'bg-yellow-100 text-yellow-800',
    file_change: 'bg-purple-100 text-purple-800',
    dependency: 'bg-green-100 text-green-800',
    insight: 'bg-indigo-100 text-indigo-800',
  };

  if (loading) {
    return <div className="text-gray-500">Loading context...</div>;
  }

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Zap size={20} className="text-yellow-500" />
          <h3 className="text-lg font-semibold">Compressed Context</h3>
        </div>

        <select
          value={maxPerType}
          onChange={(e) => setMaxPerType(Number(e.target.value))}
          className="input py-1.5 text-sm"
        >
          <option value="3">Top 3 per type</option>
          <option value="5">Top 5 per type</option>
          <option value="10">Top 10 per type</option>
        </select>
      </div>

      <div className="text-xs text-gray-500 mb-3">
        Showing {topFacts.length} most important facts
      </div>

      {topFacts.length === 0 ? (
        <p className="text-gray-500 text-sm">No facts available</p>
      ) : (
        <div className="space-y-2 max-h-96 overflow-y-auto">
          {topFacts.map((fact) => (
            <div
              key={fact.id}
              className="p-2 bg-gray-50 rounded border border-gray-200 hover:border-gray-300 transition-colors"
            >
              <div className="flex items-start gap-2">
                <div className="flex items-center gap-2 flex-shrink-0">
                  <span
                    className={`px-2 py-0.5 rounded text-xs font-medium ${factTypeColors[fact.fact_type]}`}
                  >
                    {factTypeLabels[fact.fact_type] || fact.fact_type}
                  </span>
                  <div className="flex items-center gap-0.5">
                    {Array.from({ length: fact.importance }).map((_, i) => (
                      <Star key={i} size={10} className="fill-yellow-400 text-yellow-400" />
                    ))}
                  </div>
                </div>
                <p className="text-sm text-gray-700 flex-1">{fact.content}</p>
              </div>
            </div>
          ))}
        </div>
      )}

      <div className="mt-4 pt-3 border-t border-gray-200 text-xs text-gray-500">
        💡 This view shows the most important facts per category, helping you quickly understand project context
      </div>
    </div>
  );
}
