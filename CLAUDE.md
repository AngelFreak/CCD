# Claude Context Tracker

## Project Overview
A project context tracker with intelligent Claude Code integration. Manages context across multiple development projects and prevents context loss during conversation compacting.

## Tech Stack
- **Frontend**: React + Vite + TypeScript + Tailwind CSS
- **Backend**: PocketBase (single Go binary)
- **Daemon**: Go binary for Claude Code monitoring
- **Mobile**: Capacitor (optional, later phase)

## Current State
Status: **In Development**

## Next Steps
1. Complete frontend implementation
2. Implement daemon for Claude Code monitoring
3. Build CLI tool
4. Add smart context features

## Gotchas
- PocketBase realtime requires proper CORS config for local dev
- Claude Code logs location varies by OS (check ~/.claude/ or similar)
- Token counting needs to match Claude's tokenizer for accuracy

## Decisions Log
- Using PocketBase over Supabase for single-binary simplicity
- Go for daemon/CLI because it compiles to single binary, easy distribution
- Tailwind for rapid UI development
- Structured sections over freeform markdown for better extraction
