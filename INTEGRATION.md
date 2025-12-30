# Integration Guide

## Complete System Integration

This guide explains how all components work together in the Claude Context Tracker.

## Architecture Flow

```
┌──────────────────────────────────────────────────────────────┐
│                        User Workflow                          │
└──────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Web UI      │     │     CLI      │     │   Daemon     │
│  (React)     │     │  (Go Tool)   │     │  (Monitor)   │
└──────────────┘     └──────────────┘     └──────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │   PocketBase     │
                    │   (API/Storage)  │
                    └──────────────────┘
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
            ┌──────────────┐    ┌──────────────┐
            │   Database   │    │  File System │
            │  (SQLite)    │    │   (Ledger)   │
            └──────────────┘    └──────────────┘
```

## Component Integration

### 1. PocketBase Setup

**Start PocketBase:**
```bash
cd pocketbase
./pocketbase serve --http=0.0.0.0:8090
```

**On first run:**
1. Open http://localhost:8090/_/
2. Create admin account
3. Migrations auto-apply
4. Collections created automatically

### 2. Frontend Setup

**Install dependencies:**
```bash
cd frontend
npm install
```

**Configure environment:**
```bash
# frontend/.env already exists with:
VITE_POCKETBASE_URL=http://localhost:8090
```

**Start development server:**
```bash
npm run dev
# Runs on http://localhost:5173
```

**Features enabled:**
- Real-time updates via PocketBase subscriptions
- Context editing with 6 section types
- Session monitoring with token tracking
- Diff viewer (Session Diffs tab)
- Compressed context view (Compressed View tab)
- Facts list with importance stars

### 3. Daemon Integration

**Build daemon:**
```bash
cd daemon
go mod download
go build -o cct-daemon
```

**Run with smart features:**
```bash
# Basic mode (no smart features)
./cct-daemon -project <project-id> -v

# Smart mode (recommended)
./cct-daemon -project <project-id> -smart -v

# With custom settings
./cct-daemon \
  -project <project-id> \
  -smart \
  -compact-threshold 170000 \
  -repo /path/to/repo \
  -logs /path/to/claude/logs \
  -v
```

**What the daemon does:**
1. **Watches** Claude Code logs directory
2. **Parses** conversation logs (JSON or text format)
3. **Extracts** facts using pattern matching
4. **Scores** importance (1-5 stars) if smart mode enabled
5. **Updates** continuity ledger in `thoughts/ledgers/`
6. **Creates** pre-compact handoffs at 85% token threshold
7. **Pushes** facts to PocketBase in real-time

**Smart Features:**
- ✅ Importance scoring algorithm
- ✅ Stale fact detection
- ✅ Pre-compact handoff generation
- ✅ Continuity ledger updates
- ✅ Token tracking and warnings

### 4. CLI Tool Integration

**Build CLI:**
```bash
cd cli
go mod download
go build -o cct
```

**Install globally (optional):**
```bash
sudo mv cct /usr/local/bin/
```

**Usage examples:**

```bash
# Pull context to CLAUDE.md
cct pull my-project

# Push session summary
cct push my-project "Implemented smart context features"

# Show status
cct status

# View session diffs
cct diff my-project -n 10

# Switch projects
cct switch another-project
```

## Data Flow Example

### Scenario: Developer working on a project

**Step 1: Start daemon**
```bash
./cct-daemon -project abc123 -smart -v
```
Output:
```
Starting Claude Context Tracker daemon
PocketBase URL: http://localhost:8090
Project ID: abc123
Repo Path: /home/user/my-project
Smart mode: true
Compact threshold: 170000 tokens
Daemon started successfully.
```

**Step 2: Work in Claude Code**
- Developer writes code
- Claude Code logs conversation to `~/.claude/logs/session.log`
- Daemon detects file change
- Parses conversation
- Extracts facts: "Added authentication middleware" (file_change)

**Step 3: Smart processing**
```
Created fact (importance: 3): Added authentication middleware (file_change)
Token count: 15234
Smart features: 1 facts processed, 154766 tokens remaining until compact
```

**Step 4: Ledger update**
File created: `thoughts/ledgers/CONTINUITY_2025-12-30.jsonl`
```json
{
  "timestamp": "2025-12-30T10:15:00Z",
  "session_id": "20251230_101500",
  "project_id": "abc123",
  "token_count": 15234,
  "facts": [
    {
      "type": "file_change",
      "content": "Added authentication middleware",
      "importance": 3,
      "timestamp": "2025-12-30T10:15:00Z"
    }
  ]
}
```

**Step 5: Approaching threshold**
When tokens reach 144500 (85% of 170000):
```
✓ Handoff created: Modified codebase. (tokens: 144500, facts: 47)
```

File created: `thoughts/shared/handoffs/handoff_20251230_101500_20251230_143000.md`

**Step 6: View in frontend**
1. Open http://localhost:5173
2. Click on project
3. See facts with star ratings
4. Switch to "Session Diffs" tab
5. View token trends and changes
6. Switch to "Compressed View" tab
7. See top 5 facts per category

**Step 7: Use CLI**
```bash
# Check status
cct status
# Output:
# 📂 Current Project: My Project (my-project)
# 📍 Path: /home/user/my-project
# 🟢 Status: active
#
# 📝 Last Session:
#    Summary: Modified codebase
#    Tokens: 144500

# View diffs
cct diff my-project
# Output:
# 📊 Session Diff for my-project
#
# Session: Dec 30, 2025 2:30 PM
# Summary: Modified codebase.
# Tokens:  +5234 (increased)
```

## Troubleshooting Integration

### Daemon can't find PocketBase

**Problem:** `Failed to verify project: connection refused`

**Solution:**
```bash
# Check PocketBase is running
curl http://localhost:8090/api/health

# If not running:
cd pocketbase && ./pocketbase serve
```

### Frontend can't connect

**Problem:** CORS errors or connection refused

**Solution:**
1. Verify PocketBase is running
2. Check `.env` has correct URL
3. Restart frontend dev server
4. Check PocketBase CORS settings in admin UI

### Daemon creates facts but frontend doesn't update

**Problem:** Real-time subscriptions not working

**Solution:**
1. Check browser console for WebSocket errors
2. Verify PocketBase allows WebSocket connections
3. Refresh the page (subscriptions re-establish)

### Ledger files not created

**Problem:** Smart mode enabled but no files in `thoughts/ledgers/`

**Solution:**
1. Verify `-repo` path is correct
2. Check directory permissions: `chmod 755 thoughts/ledgers/`
3. Ensure smart mode is enabled: `-smart`
4. Check daemon logs for errors

### CLI can't find project

**Problem:** `project not found: my-project`

**Solution:**
```bash
# List all projects via PocketBase admin
# Or create project via frontend first
# Then use exact slug name in CLI
```

## Best Practices

### For Development

1. **Always run PocketBase first** - other components depend on it
2. **Enable verbose logging** during setup: `-v` flag
3. **Use smart mode** for better context management: `-smart`
4. **Set proper repo path** so ledgers are created in project: `-repo`

### For Production

1. **Run daemon as systemd service** (see README for setup)
2. **Use environment variables** for configuration
3. **Monitor ledger disk usage** (grows over time)
4. **Backup PocketBase** regularly: `pocketbase backup`
5. **Archive old sessions** to keep database performant

### For Context Preservation

1. **Check handoffs regularly**: `ls thoughts/shared/handoffs/`
2. **Read ledger before resuming** after `/clear`
3. **Use compressed view** for quick context overview
4. **Export CLAUDE.md** before major changes: `cct pull <project>`

## Verification Checklist

After setup, verify integration:

- [ ] PocketBase running on port 8090
- [ ] Admin account created
- [ ] Frontend accessible at localhost:5173
- [ ] Can create a project via UI
- [ ] Daemon connects to PocketBase
- [ ] Daemon creates ledger files (if smart mode)
- [ ] CLI commands work (`cct status`)
- [ ] Real-time updates visible in frontend
- [ ] Session monitoring shows token count
- [ ] Diff viewer shows session changes
- [ ] Compressed view shows top facts

## Performance Notes

**Database Size:**
- ~1MB per 1000 facts
- ~500KB per ledger file (daily)
- Optimize with periodic cleanup

**Memory Usage:**
- Frontend: ~50MB
- Daemon: ~10-20MB
- PocketBase: ~30-50MB
- Total: ~100MB typical

**Token Counting:**
- Estimate: 4 chars ≈ 1 token
- Actual may vary by tokenizer
- Adjust `-compact-threshold` if needed

## Next Steps

Once integration is verified:

1. Create your first project via frontend
2. Add context sections
3. Start daemon with smart mode
4. Work in Claude Code
5. Monitor in frontend
6. Use CLI for quick operations
7. Check ledger/handoff files
8. Review compressed context regularly

The system is now fully integrated and operational! 🎉
