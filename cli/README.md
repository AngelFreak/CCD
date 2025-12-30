# Claude Context Tracker CLI

Command-line tool for managing project contexts and Claude Code sessions.

## Installation

```bash
go build -o cct
sudo mv cct /usr/local/bin/  # Optional: install globally
```

## Commands

### `cct pull <project-slug>`

Pull project context and write to CLAUDE.md file.

```bash
cct pull my-project
cct pull my-project -o context.md  # Custom output file
```

**Options:**
- `-o, --output`: Output file (default: CLAUDE.md)

### `cct push <project-slug> <summary>`

Save session summary to PocketBase.

```bash
cct push my-project "Implemented user authentication with JWT"
cct push my-project "Fixed bug in payment processing"
```

### `cct status`

Show active project and session information.

```bash
cct status
```

Output:
```
📂 Current Project: My Awesome App (my-awesome-app)
📍 Path: /home/user/projects/my-app
🟢 Status: active

📝 Last Session:
   Summary: Implemented user authentication
   Tokens: 15,234
```

### `cct switch <project-slug>`

Switch to a different project and pull its context.

```bash
cct switch another-project
```

This will:
1. Change to the project's directory
2. Pull context to CLAUDE.md
3. Display project information

### `cct version`

Display version information.

```bash
cct version
```

## Global Flags

- `--pb-url`: PocketBase URL (default: http://localhost:8090)

```bash
cct status --pb-url http://your-server:8090
```

## Configuration

### Environment Variables

```bash
export CCT_PB_URL=http://localhost:8090
```

### Config File (Future)

`~/.cct/config.yaml`:
```yaml
pocketbase_url: http://localhost:8090
default_project: my-project
auto_pull: true
```

## Workflow Examples

### Starting Work on a Project

```bash
# Switch to project and get context
cct switch my-project

# Work with Claude Code...

# Save session summary when done
cct push my-project "Added dark mode toggle to settings"
```

### Working with Multiple Projects

```bash
# Check current status
cct status

# Pull context for specific project
cct pull project-a

# Work on it...

# Save progress
cct push project-a "Refactored authentication module"

# Switch to another project
cct switch project-b

# Continue working...
```

### Daily Workflow

```bash
# Morning: Check project status
cct status

# Pull latest context
cct pull my-project

# Start Claude Code session with CLAUDE.md

# Evening: Save session summary
cct push my-project "Summary of today's work"
```

## Tips

1. **Use descriptive summaries**: They help track progress and review history
2. **Pull before starting**: Always pull latest context before a session
3. **Push after major changes**: Save summaries after completing features
4. **Check status regularly**: Keep track of active projects and token usage

## Development

```bash
# Run without building
go run main.go status

# Build for current platform
go build -o cct

# Build for all platforms
GOOS=linux GOARCH=amd64 go build -o cct-linux
GOOS=darwin GOARCH=amd64 go build -o cct-macos
GOOS=windows GOARCH=amd64 go build -o cct.exe
```

## Troubleshooting

### "Project not found"

- Verify project slug is correct
- Check PocketBase is running
- Ensure project exists in database

### "Failed to fetch"

- Check PocketBase URL is correct
- Verify network connectivity
- Ensure PocketBase API is accessible

### Permission errors

- Check file permissions on output directory
- Verify write access to CLAUDE.md location
- Use sudo for global installation
