# Claude Context Tracker (CCT)

Track and manage context for your Claude Code projects. Never lose important details when conversations get compacted.

## What It Does

- **Web Dashboard**: Manage multiple projects, edit context sections, view session history
- **CLI Tool**: Quick commands to pull/push context, check status, switch projects
- **Daemon**: Monitors Claude Code conversations, extracts facts automatically
- **Smart Context**: Scores importance, detects stale facts, creates handoff documents

## Tech Stack

- **Frontend**: React + Vite (web UI)
- **Backend**: PocketBase (single binary database)
- **Daemon/CLI**: Go binaries

## Installation (Ubuntu/Linux)

### Prerequisites

- Node.js 18+
- Go 1.21+

### 1. Setup PocketBase

```bash
cd pocketbase
wget https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_linux_amd64.zip
unzip pocketbase_linux_amd64.zip
./pocketbase serve
```

Open http://localhost:8090/_/ and create an admin account.

### 2. Setup Frontend

```bash
cd frontend
npm install
npm run dev  # Development
# or
npm run build  # Production build
```

### 3. Build CLI

```bash
cd cli
go build -o cct
sudo mv cct /usr/local/bin/
```

### 4. Build Daemon

```bash
cd daemon
go build -o cct-daemon
sudo mv cct-daemon /usr/local/bin/
```

## Usage

### Starting the Services

```bash
# Terminal 1: Start PocketBase
cd pocketbase && ./pocketbase serve

# Terminal 2: Start Frontend
cd frontend && npm run dev

# Terminal 3: Start Daemon (optional)
cct-daemon -project <project-id> -repo /path/to/repo
```

Access the dashboard at http://localhost:5173

### CLI Commands

```bash
cct pull <project>              # Write context to CLAUDE.md
cct push <project> "summary"    # Save session summary
cct status                      # Show active project & token usage
cct switch <project>            # Change active project
cct diff <project>              # Show what changed
```

## How It Works

### Smart Context Features

- **Importance Scoring**: Facts auto-scored 1-5 based on type (blockers=5, decisions=4, etc.)
- **Stale Detection**: Marks old TODOs, blockers, and file changes as stale
- **Context Compression**: Keeps only top N most important facts per type
- **Pre-Compact Handoff**: At 85% token threshold, creates handoff document with key facts
- **Continuity Ledger**: Maintains lossless state record in `thoughts/ledgers/CONTINUITY_*.jsonl`

### Running as System Service

To run the daemon automatically, create `/etc/systemd/system/cct-daemon.service`:

```ini
[Unit]
Description=Claude Context Tracker Daemon
After=network.target

[Service]
Type=simple
User=yourusername
ExecStart=/usr/local/bin/cct-daemon -project YOUR_PROJECT_ID -repo /path/to/repo
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable cct-daemon
sudo systemctl start cct-daemon
```

## License

MIT License
