# Claude Context Tracker Daemon

Go daemon for monitoring Claude Code sessions and automatically extracting facts.

## Features

- **File Watching**: Monitors Claude Code log directory for changes
- **Conversation Parsing**: Parses JSON and text-based conversation logs
- **Fact Extraction**: Automatically identifies decisions, blockers, TODOs, file changes, dependencies, and insights
- **Real-time Updates**: Pushes facts to PocketBase in real-time
- **Token Counting**: Estimates token usage from conversations

## Usage

```bash
# Basic usage
./cct-daemon -project <project-id>

# With verbose logging
./cct-daemon -project <project-id> -v

# Custom PocketBase URL
./cct-daemon -project <project-id> -pb-url http://localhost:8090

# Custom log path
./cct-daemon -project <project-id> -logs /path/to/claude/logs
```

## Command Line Flags

- `-project` (required): Project ID to track
- `-pb-url`: PocketBase URL (default: http://localhost:8090)
- `-logs`: Claude Code logs directory (auto-detected by default)
- `-v`: Enable verbose logging

## How It Works

1. **Watches** the Claude Code logs directory for file changes
2. **Parses** conversation logs when they're modified
3. **Extracts** facts using pattern matching:
   - **Decisions**: "decided to", "chose to", "going with", "will use"
   - **Blockers**: "blocked by", "can't proceed", "error:", "failed to"
   - **TODOs**: "TODO:", "need to", "should", "must"
   - **File Changes**: "created", "modified", "updated", "deleted" + file extensions
   - **Dependencies**: "installed", "added dependency", "npm install", "go get"
   - **Insights**: "discovered", "found that", "interesting", "note that"
4. **Pushes** extracted facts to PocketBase
5. **Tracks** token usage for context window monitoring

## Building

```bash
go build -o cct-daemon
```

## Running as a Service

### systemd (Linux)

Create `/etc/systemd/system/cct-daemon.service`:

```ini
[Unit]
Description=Claude Context Tracker Daemon
After=network.target

[Service]
Type=simple
User=yourusername
ExecStart=/usr/local/bin/cct-daemon -project YOUR_PROJECT_ID -v
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable cct-daemon
sudo systemctl start cct-daemon
sudo systemctl status cct-daemon
```

### launchd (macOS)

Create `~/Library/LaunchAgents/com.cct.daemon.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.cct.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/cct-daemon</string>
        <string>-project</string>
        <string>YOUR_PROJECT_ID</string>
        <string>-v</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

Load and start:

```bash
launchctl load ~/Library/LaunchAgents/com.cct.daemon.plist
launchctl start com.cct.daemon
```

## Development

```bash
# Run with verbose logging
go run main.go -project <id> -v

# Format code
go fmt ./...

# Run tests
go test ./...

# Update dependencies
go mod tidy
```

## Architecture

```
main.go
  └─> monitor/watcher.go (file watching)
        └─> monitor/parser.go (conversation parsing)
              └─> extractor/facts.go (fact extraction)
                    └─> api/pocketbase.go (API client)
```

## Configuration

The daemon auto-detects Claude Code log locations:

- Linux: `~/.claude/logs`, `~/.config/claude/logs`
- macOS: `~/Library/Application Support/Claude/logs`
- Windows: `%APPDATA%\Claude\logs`

Override with `-logs` flag if needed.

## Troubleshooting

### Daemon won't start

- Verify project ID exists in PocketBase
- Check PocketBase is running and accessible
- Ensure log path exists and is readable

### No facts extracted

- Enable verbose mode (`-v`) to see parsing details
- Check log file format is supported
- Verify conversation contains extractable patterns

### High CPU usage

- Reduce log file size
- Optimize fact extraction patterns
- Consider batching API calls
