import { useState, useEffect } from 'react';
import { Activity, Clock } from 'lucide-react';

interface SessionMonitorProps {
  projectId: string;
}

export default function SessionMonitor({ projectId }: SessionMonitorProps) {
  const [tokenCount, setTokenCount] = useState(0);
  const [sessionStart] = useState(new Date());
  const [elapsed, setElapsed] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setElapsed(Math.floor((Date.now() - sessionStart.getTime()) / 1000));
    }, 1000);

    return () => clearInterval(interval);
  }, [sessionStart]);

  const formatTime = (seconds: number) => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  };

  return (
    <div className="card">
      <h3 className="text-lg font-semibold mb-4">Current Session</h3>

      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-gray-600">
            <Clock size={18} />
            <span>Duration</span>
          </div>
          <span className="font-mono text-lg">{formatTime(elapsed)}</span>
        </div>

        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-gray-600">
            <Activity size={18} />
            <span>Token Usage</span>
          </div>
          <span className="font-mono text-lg">{tokenCount.toLocaleString()}</span>
        </div>

        <div className="pt-3 border-t border-gray-200">
          <div className="text-sm text-gray-500">
            Started {sessionStart.toLocaleTimeString()}
          </div>
        </div>
      </div>

      <div className="mt-4">
        <div className="text-xs text-gray-500 mb-2">Context Window</div>
        <div className="w-full bg-gray-200 rounded-full h-2">
          <div
            className="bg-blue-600 h-2 rounded-full transition-all"
            style={{ width: `${Math.min((tokenCount / 200000) * 100, 100)}%` }}
          />
        </div>
        <div className="text-xs text-gray-500 mt-1 text-right">
          {tokenCount.toLocaleString()} / 200,000
        </div>
      </div>
    </div>
  );
}
