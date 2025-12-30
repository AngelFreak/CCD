# PocketBase Setup

## Installation

1. Download PocketBase for your platform:
   - Linux: `https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_linux_amd64.zip`
   - macOS: `https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_darwin_amd64.zip`
   - Windows: `https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_windows_amd64.zip`

2. Extract the binary to this directory (`pocketbase/`)

3. Run PocketBase:
   ```bash
   ./pocketbase serve --http=0.0.0.0:8090
   ```

4. Open `http://localhost:8090/_/` to access the admin UI

5. Create an admin account (first time only)

6. The migrations in `pb_migrations/` will be applied automatically

## Collections

- **projects**: Main project tracking
- **context_sections**: Structured context sections for each project
- **session_history**: Claude Code session summaries
- **extracted_facts**: Facts extracted from conversations

## API Access

All collections are accessible via REST API at:
- `http://localhost:8090/api/collections/{collection}/records`

## CORS Configuration

For local development, CORS is enabled by default. For production, configure CORS settings in the admin UI.
