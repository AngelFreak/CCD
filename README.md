# Claude Context Tracker (CCT)

A comprehensive project context tracker with intelligent Claude Code integration. Manages context across multiple development projects and prevents context loss during conversation compacting.

## Features

### Phase 1: Project Dashboard ✅
- ✅ Project list with status filters (active, paused, idea, archived)
- ✅ Context editor with structured sections
- ✅ Markdown export (for CLAUDE.md sync)
- ✅ Copy context to clipboard
- ✅ Real-time updates via PocketBase
- ✅ Beautiful UI with Tailwind CSS

### Phase 2: Claude Code Daemon ✅
- ✅ Monitor Claude Code conversation logs
- ✅ Track token usage in real-time
- ✅ Auto-extract facts from conversations
- ✅ Push extracted facts to PocketBase
- ✅ File system watcher for log changes

### Phase 3: CLI Integration ✅
- ✅ `cct pull <project>` - write context to CLAUDE.md
- ✅ `cct push "<summary>"` - save session summary
- ✅ `cct status` - show active project, token usage
- ✅ `cct switch <project>` - change active project
- ✅ `cct diff <project>` - show session differences

### Phase 4: Smart Context ✅
- ✅ Importance scoring for facts (1-5 scale with AI-like scoring)
- ✅ Stale fact detection (auto-marks outdated facts)
- ✅ Context compression (keeps top N facts per type)
- ✅ Diff view: what changed since last session
- ✅ Continuity ledger system (inspired by Continuous-Claude-v2)
- ✅ Pre-compact handoff detection (saves state before context clearing)
- ✅ Session resumption with full context restore

## Tech Stack

- **Frontend**: React + Vite + TypeScript + Tailwind CSS
- **Backend**: PocketBase (single Go binary)
- **Daemon**: Go binary for Claude Code monitoring
- **CLI**: Go binary with Cobra
- **Mobile**: Capacitor (optional, later phase)

## Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   React Web UI  │────▶│   PocketBase    │◀────│   Go Daemon     │
│   (Dashboard)   │     │   (Storage)     │     │ (CC Monitor)    │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                                                        ▼
                                                ┌─────────────────┐
                                                │  Claude Code    │
                                                │  (Session)      │
                                                └─────────────────┘
```

## Quick Start

### Prerequisites

- Node.js 18+ (for frontend)
- Go 1.21+ (for daemon and CLI)
- PocketBase binary (downloaded automatically or manually)

### 1. Setup PocketBase

```bash
cd pocketbase

# Download PocketBase (Linux example)
wget https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_linux_amd64.zip
unzip pocketbase_linux_amd64.zip
rm pocketbase_linux_amd64.zip

# Start PocketBase
./pocketbase serve --http=0.0.0.0:8090
```

Open `http://localhost:8090/_/` and create an admin account.

### 2. Setup Frontend

```bash
cd frontend
npm install
npm run dev
```

Open `http://localhost:5173` to access the dashboard.

### 3. Build CLI

```bash
cd cli
go build -o cct
sudo mv cct /usr/local/bin/  # Optional: make it globally available
```

### 4. Build Daemon

```bash
cd daemon
go build -o cct-daemon
```

## Usage

### Web Dashboard

1. Open `http://localhost:5173`
2. Create a new project
3. Add context sections (Architecture, Current State, Next Steps, etc.)
4. Export to markdown or copy to clipboard
5. View extracted facts and session history

### CLI Commands

```bash
# Pull project context to CLAUDE.md
cct pull my-project

# Save a session summary
cct push my-project "Implemented user authentication"

# Show current project status
cct status

# Show session differences (what changed)
cct diff my-project

# Switch to a different project
cct switch another-project

# Show version
cct version
```

### Daemon

```bash
# Start monitoring Claude Code logs for a project
./cct-daemon -project <project-id> -v

# With smart context features enabled (default)
./cct-daemon -project <project-id> -smart -compact-threshold 170000

# Custom log path
./cct-daemon -project <project-id> -logs /path/to/logs

# Custom PocketBase URL
./cct-daemon -project <project-id> -pb-url http://your-server:8090

# Specify repository path for ledger storage
./cct-daemon -project <project-id> -repo /path/to/repo
```

## Project Structure

```
claude-context-tracker/
├── frontend/               # React + Vite frontend
│   ├── src/
│   │   ├── components/    # UI components
│   │   ├── hooks/         # React hooks
│   │   ├── lib/           # Utilities
│   │   ├── pages/         # Page components
│   │   └── types.ts       # TypeScript types
│   └── package.json
├── daemon/                # Go daemon for monitoring
│   ├── monitor/          # File watching and parsing
│   ├── extractor/        # Fact extraction logic
│   ├── api/              # PocketBase client
│   └── main.go
├── cli/                  # Go CLI tool
│   ├── commands/         # CLI commands
│   └── main.go
├── pocketbase/          # PocketBase backend
│   ├── pb_migrations/   # Database migrations
│   └── pb_data/         # Database files
├── CLAUDE.md            # Project context template
└── README.md
```

## Data Model

### Collections

#### Projects
- `name`: Project name
- `slug`: URL-friendly identifier
- `repo_path`: Local repository path
- `status`: active | paused | idea | archived
- `priority`: Priority level (1-5)
- `tech_stack`: Array of technologies
- `description`: Project description

#### Context Sections
- `project`: Relation to project
- `section_type`: architecture | current_state | next_steps | gotchas | decisions | custom
- `title`: Section title
- `content`: Markdown content
- `order`: Display order
- `auto_extracted`: Boolean flag

#### Session History
- `project`: Relation to project
- `summary`: Session summary text
- `facts_extracted`: JSON of extracted facts
- `token_count`: Token usage count
- `session_start`: Start timestamp
- `session_end`: End timestamp

#### Extracted Facts
- `project`: Relation to project
- `session`: Relation to session (optional)
- `fact_type`: decision | blocker | file_change | dependency | todo | insight
- `content`: Fact content
- `importance`: 1-5 scale
- `stale`: Boolean flag

## Development

### Frontend Development

```bash
cd frontend
npm run dev      # Start dev server
npm run build    # Build for production
npm run lint     # Run linter
```

### Backend Development

PocketBase automatically applies migrations on startup. To modify schema:

1. Edit `pocketbase/pb_migrations/*.js`
2. Restart PocketBase
3. Changes are applied automatically

### Daemon Development

```bash
cd daemon
go mod tidy
go run main.go -project <id> -v
```

### CLI Development

```bash
cd cli
go mod tidy
go run main.go status
```

## Environment Variables

### Frontend

Create `frontend/.env`:

```env
VITE_POCKETBASE_URL=http://localhost:8090
```

### Daemon & CLI

```bash
# PocketBase URL (default: http://localhost:8090)
export PB_URL=http://your-server:8090
```

## Deployment

### Frontend

```bash
cd frontend
npm run build
# Deploy dist/ folder to your hosting service
```

### PocketBase

```bash
# Production mode
./pocketbase serve --http=0.0.0.0:8090 --dir=/path/to/pb_data
```

### Daemon (systemd service)

Create `/etc/systemd/system/cct-daemon.service`:

```ini
[Unit]
Description=Claude Context Tracker Daemon
After=network.target

[Service]
Type=simple
User=yourusername
ExecStart=/usr/local/bin/cct-daemon -project YOUR_PROJECT_ID
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable cct-daemon
sudo systemctl start cct-daemon
```

## Troubleshooting

### PocketBase won't start

- Check if port 8090 is available: `lsof -i :8090`
- Check file permissions on `pb_data/` directory

### Frontend can't connect to PocketBase

- Verify `VITE_POCKETBASE_URL` in `.env`
- Check CORS settings in PocketBase admin
- Ensure PocketBase is running

### Daemon can't find logs

- Check Claude Code installation
- Verify logs path: `~/.claude/logs` or `~/.config/claude/logs`
- Use `-logs` flag to specify custom path

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see LICENSE file for details

## Roadmap

- [x] Phase 1: Project Dashboard
- [x] Phase 2: Claude Code Daemon
- [x] Phase 3: CLI Integration
- [x] Phase 4: Smart Context Features
- [ ] Phase 5: Mobile App (Capacitor)
- [ ] Phase 6: Team Collaboration
- [ ] Phase 7: Advanced AI-powered features

## Smart Context Features

### Importance Scoring
Facts are automatically scored 1-5 based on:
- **Fact type** (blockers=5, decisions=4, todos=3, etc.)
- **Content keywords** (critical, urgent, security, etc.)
- **Recency** (recent facts get bonus points)

### Stale Detection
Facts are marked stale when:
- Blockers are older than 3 days (likely resolved)
- TODOs are older than 7 days (likely completed or deprioritized)
- File changes are older than 14 days
- Content indicates completion ("done", "resolved")

### Context Compression
Keeps only top N most important facts per type, reducing noise while preserving critical information.

### Pre-Compact Handoff
When token usage reaches 85% of threshold (default: 170k), the daemon automatically creates a handoff document with:
- Session summary
- Key facts by importance
- Active TODOs
- Current blockers
- File changes

Handoff files are stored in `thoughts/shared/handoffs/` for easy resumption.

### Continuity Ledger
Inspired by [Continuous-Claude-v2](https://github.com/parcadei/Continuous-Claude-v2), the ledger system maintains a lossless state record in `thoughts/ledgers/CONTINUITY_*.jsonl`.

Each entry includes:
- Timestamp and session ID
- Token count
- All facts with importance scores
- Decisions, blockers, next steps
- File changes

This enables "clear and resume" workflows instead of lossy compacting.

## Support

For issues, questions, or suggestions:

- Open an issue on GitHub
- Check the documentation
- Review existing issues

---

**Built with ❤️ for better Claude Code experiences**
