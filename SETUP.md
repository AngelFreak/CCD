# Setup Guide

## Step-by-Step Installation

### 1. Clone the Repository

```bash
git clone https://github.com/angelfreak/CCD.git
cd CCD
```

### 2. Install PocketBase

#### Linux

```bash
cd pocketbase
wget https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_linux_amd64.zip
unzip pocketbase_linux_amd64.zip
rm pocketbase_linux_amd64.zip
chmod +x pocketbase
```

#### macOS

```bash
cd pocketbase
curl -LO https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_darwin_amd64.zip
unzip pocketbase_darwin_amd64.zip
rm pocketbase_darwin_amd64.zip
chmod +x pocketbase
```

#### Windows

```powershell
cd pocketbase
Invoke-WebRequest -Uri https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_windows_amd64.zip -OutFile pocketbase.zip
Expand-Archive pocketbase.zip -DestinationPath .
Remove-Item pocketbase.zip
```

### 3. Start PocketBase

```bash
cd pocketbase
./pocketbase serve --http=0.0.0.0:8090
```

**First time setup:**
1. Open http://localhost:8090/_/
2. Create an admin account
3. Migrations will be applied automatically

### 4. Setup Frontend

```bash
cd frontend
npm install
npm run dev
```

The frontend will be available at http://localhost:5173

### 5. Build CLI Tool

```bash
cd cli
go mod download
go build -o cct

# Optional: Install globally
sudo mv cct /usr/local/bin/cct
```

### 6. Build Daemon

```bash
cd daemon
go mod download
go build -o cct-daemon

# Optional: Install globally
sudo mv cct-daemon /usr/local/bin/cct-daemon
```

## Configuration

### Frontend Environment

Create `frontend/.env`:

```env
VITE_POCKETBASE_URL=http://localhost:8090
```

For production:

```env
VITE_POCKETBASE_URL=https://your-domain.com
```

### PocketBase Configuration

PocketBase settings can be configured via the admin UI at http://localhost:8090/_/

Important settings:
- **CORS**: Add your frontend URL to allowed origins
- **API Rules**: Configure collection access rules
- **Email**: Setup email provider for notifications (optional)

## Creating Your First Project

### Via Web UI

1. Open http://localhost:5173
2. Click "New Project"
3. Fill in:
   - Name: Your project name
   - Slug: URL-friendly identifier (e.g., "my-awesome-app")
   - Repo Path: Full path to your repository
   - Status: active
   - Priority: 1-5
   - Tech Stack: React, TypeScript, etc.
4. Click "Create"

### Via PocketBase Admin

1. Open http://localhost:8090/_/
2. Navigate to "projects" collection
3. Click "New record"
4. Fill in the required fields
5. Save

## Starting the Daemon

```bash
# Get your project ID from the web UI or PocketBase admin
cct-daemon -project YOUR_PROJECT_ID -v
```

The daemon will:
- Monitor Claude Code logs
- Extract facts automatically
- Update PocketBase in real-time

## Testing the CLI

```bash
# Check if PocketBase is reachable
cct status

# Pull context for a project
cct pull my-project

# Save a session summary
cct push my-project "Initial setup complete"

# Switch to a project
cct switch my-project
```

## Verification Checklist

- [ ] PocketBase is running on port 8090
- [ ] Admin account created in PocketBase
- [ ] Frontend is accessible at http://localhost:5173
- [ ] CLI tool installed and working (`cct version`)
- [ ] Daemon built successfully
- [ ] First project created via web UI
- [ ] Context sections can be added/edited
- [ ] Markdown export works
- [ ] CLI commands work (`cct status`)

## Troubleshooting

### Port 8090 already in use

```bash
# Find process using port 8090
lsof -i :8090
# or
netstat -nlp | grep 8090

# Kill the process or use a different port
./pocketbase serve --http=0.0.0.0:8091
```

### Frontend can't connect

1. Check `.env` file exists in `frontend/`
2. Verify PocketBase is running
3. Check CORS settings in PocketBase admin
4. Clear browser cache

### Go build errors

```bash
# Clean module cache
go clean -modcache

# Re-download dependencies
go mod download
go mod tidy

# Try building again
go build
```

### Permission errors

```bash
# Make binaries executable
chmod +x pocketbase/pocketbase
chmod +x cli/cct
chmod +x daemon/cct-daemon
```

## Next Steps

After successful setup:

1. Create your first project
2. Add context sections
3. Export context to CLAUDE.md
4. Start the daemon for automatic monitoring
5. Use CLI for quick context management

Happy tracking! 🚀
