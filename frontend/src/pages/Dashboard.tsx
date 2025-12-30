import ProjectList from '../components/ProjectList';

export default function Dashboard() {
  return (
    <div className="max-w-7xl mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Claude Context Tracker
        </h1>
        <p className="text-gray-600">
          Manage your project contexts and prevent context loss across Claude Code sessions
        </p>
      </div>

      <ProjectList />
    </div>
  );
}
