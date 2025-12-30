import { useState, useEffect } from 'react';
import pb from '../lib/pocketbase';
import { GitCompare, TrendingUp, TrendingDown, Minus } from 'lucide-react';

interface DiffViewerProps {
  projectId: string;
}

interface SessionDiff {
  added: number;
  removed: number;
  tokenDelta: number;
  timestamp: string;
  summary: string;
}

export default function DiffViewer({ projectId }: DiffViewerProps) {
  const [diffs, setDiffs] = useState<SessionDiff[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedPeriod, setSelectedPeriod] = useState<'day' | 'week' | 'month'>('week');

  useEffect(() => {
    loadDiffs();
  }, [projectId, selectedPeriod]);

  async function loadDiffs() {
    try {
      // Get session history
      const records = await pb.collection('session_history').getFullList({
        filter: `project="${projectId}"`,
        sort: '-created',
        limit: getPeriodLimit(selectedPeriod),
      });

      // Calculate diffs between consecutive sessions
      const calculatedDiffs: SessionDiff[] = [];
      for (let i = 1; i < records.length; i++) {
        const current = records[i - 1];
        const previous = records[i];

        calculatedDiffs.push({
          added: 0, // Would need fact comparison
          removed: 0,
          tokenDelta: (current.token_count || 0) - (previous.token_count || 0),
          timestamp: current.created,
          summary: current.summary || 'No summary',
        });
      }

      setDiffs(calculatedDiffs);
    } catch (err) {
      console.error('Failed to load diffs:', err);
    } finally {
      setLoading(false);
    }
  }

  function getPeriodLimit(period: string): number {
    switch (period) {
      case 'day':
        return 5;
      case 'week':
        return 10;
      case 'month':
        return 30;
      default:
        return 10;
    }
  }

  if (loading) {
    return <div className="text-gray-500">Loading diff history...</div>;
  }

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <GitCompare size={20} />
          <h3 className="text-lg font-semibold">Session Changes</h3>
        </div>

        <select
          value={selectedPeriod}
          onChange={(e) => setSelectedPeriod(e.target.value as 'day' | 'week' | 'month')}
          className="input py-1.5 text-sm"
        >
          <option value="day">Last Day</option>
          <option value="week">Last Week</option>
          <option value="month">Last Month</option>
        </select>
      </div>

      {diffs.length === 0 ? (
        <p className="text-gray-500 text-sm">No session history available</p>
      ) : (
        <div className="space-y-3">
          {diffs.map((diff, index) => (
            <div
              key={index}
              className="p-3 bg-gray-50 rounded-lg border border-gray-200"
            >
              <div className="flex items-start justify-between mb-2">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-900">{diff.summary}</p>
                  <p className="text-xs text-gray-500 mt-1">
                    {new Date(diff.timestamp).toLocaleString()}
                  </p>
                </div>

                <div className="flex items-center gap-2 ml-4">
                  {diff.tokenDelta > 0 ? (
                    <div className="flex items-center gap-1 text-blue-600 text-sm">
                      <TrendingUp size={14} />
                      <span>+{diff.tokenDelta}</span>
                    </div>
                  ) : diff.tokenDelta < 0 ? (
                    <div className="flex items-center gap-1 text-green-600 text-sm">
                      <TrendingDown size={14} />
                      <span>{diff.tokenDelta}</span>
                    </div>
                  ) : (
                    <div className="flex items-center gap-1 text-gray-500 text-sm">
                      <Minus size={14} />
                      <span>0</span>
                    </div>
                  )}
                </div>
              </div>

              {(diff.added > 0 || diff.removed > 0) && (
                <div className="flex items-center gap-4 text-xs">
                  {diff.added > 0 && (
                    <span className="text-green-600">+{diff.added} facts</span>
                  )}
                  {diff.removed > 0 && (
                    <span className="text-red-600">-{diff.removed} facts</span>
                  )}
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
