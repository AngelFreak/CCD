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

### Option 1: Download Pre-built Package (Easiest) ⭐

**Prerequisites:** None! Everything is included and ready to use.

```bash
# Download the latest .deb package from GitHub releases
wget https://github.com/AngelFreak/CCD/releases/latest/download/cct_1.0.0_amd64.deb

# Install the package
sudo apt install ./cct_1.0.0_amd64.deb
```

Or visit the [Releases page](https://github.com/AngelFreak/CCD/releases) to download manually.

**What you get:**
- ✅ Complete native Ubuntu application
- ✅ Desktop launcher in Applications menu (Development → Claude Context Tracker)
- ✅ Web interface pre-built and ready (no npm setup needed!)
- ✅ PocketBase backend running as systemd service
- ✅ CLI tools: `cct`, `cct-daemon`, `cct-launcher`

**First-time setup (30 seconds):**
1. Launch CCT from Applications menu → **Development** → **Claude Context Tracker**
   - Or run: `cct-launcher`
   - Or visit: http://localhost:8090
2. Create your admin account at http://localhost:8090/_/
3. Start tracking your projects! 🚀

---

### Option 2: Build from Source

**Prerequisites:** Node.js 18+, Go 1.21+

```bash
# Clone the repository
git clone https://github.com/AngelFreak/CCD.git
cd CCD

# Build the .deb package (includes frontend build)
./build-deb.sh

# Install the package
sudo apt install ./build/cct_1.0.0_amd64.deb
```

**What gets installed:**
- `cct` - CLI tool
- `cct-daemon` - Monitoring daemon
- `cct-pocketbase` - Database (auto-starts as systemd service)
- `cct-launcher` - Desktop launcher script
- Pre-built frontend in `/usr/share/cct/frontend` (served by PocketBase)
- Desktop integration files (.desktop, icon)

**Optional - Configure daemon:**
```bash
sudo cp /usr/share/cct/cct-daemon.service.template /etc/systemd/system/cct-daemon.service
sudo nano /etc/systemd/system/cct-daemon.service  # Edit PROJECT_ID and repo path
sudo systemctl enable cct-daemon
sudo systemctl start cct-daemon
```

---

### Option 3: Manual Installation (Advanced)

**Prerequisites:** Node.js 18+, Go 1.21+

```bash
# 1. Setup PocketBase
cd pocketbase
wget https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_linux_amd64.zip
unzip pocketbase_linux_amd64.zip
./pocketbase serve

# 2. Setup Frontend
cd frontend
npm install
npm run dev

# 3. Build CLI
cd cli
go build -o cct
sudo mv cct /usr/local/bin/

# 4. Build Daemon
cd daemon
go build -o cct-daemon
sudo mv cct-daemon /usr/local/bin/
```

## Usage

### Launching CCT

**If you installed via .deb (recommended):**
```bash
# Option 1: Launch from Applications menu
# Navigate to: Applications → Development → Claude Context Tracker

# Option 2: Use the launcher command
cct-launcher

# Option 3: Direct browser access
# Visit: http://localhost:8090
```

PocketBase is already running as a system service, and the web interface is pre-built and ready to use!

**If you installed manually:**
```bash
# Terminal 1: Start PocketBase
cd pocketbase && ./pocketbase serve

# Terminal 2: Start Frontend (development mode)
cd frontend && npm run dev

# Terminal 3: Start Daemon (optional)
cct-daemon -project <project-id> -repo /path/to/repo
```

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

**Note:** If you installed via .deb, PocketBase is already running as a system service.

To configure the daemon as a system service:

```bash
# Copy the template (already done if you used .deb)
sudo cp /usr/share/cct/cct-daemon.service.template /etc/systemd/system/cct-daemon.service

# Edit with your project ID and repo path
sudo nano /etc/systemd/system/cct-daemon.service

# Enable and start
sudo systemctl enable cct-daemon
sudo systemctl start cct-daemon

# Check status
sudo systemctl status cct-daemon
```

## License

MIT License
