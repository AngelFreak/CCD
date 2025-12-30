# Docker Deployment Guide

Complete guide for deploying Claude Context Tracker using Docker and Docker Compose.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Development Setup](#development-setup)
- [Production Setup](#production-setup)
- [Configuration](#configuration)
- [Docker Commands](#docker-commands)
- [Troubleshooting](#troubleshooting)
- [Advanced Topics](#advanced-topics)

## Overview

Claude Context Tracker consists of three main services:

1. **PocketBase** - Backend database and API (Port 8090)
2. **Frontend** - React web interface (Port 3000 prod, 5173 dev)
3. **Daemon** - Claude Code monitoring service (optional)

All services are containerized and orchestrated using Docker Compose.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                Docker Network                   │
│                                                 │
│  ┌──────────────┐  ┌──────────────┐            │
│  │   Frontend   │  │  PocketBase  │            │
│  │  (nginx/node)│──│   (Alpine)   │            │
│  │   Port 3000  │  │   Port 8090  │            │
│  └──────────────┘  └──────┬───────┘            │
│                            │                     │
│                     ┌──────▼───────┐            │
│                     │   pb_data    │            │
│                     │   (Volume)   │            │
│                     └──────────────┘            │
│                                                 │
│  ┌──────────────┐                              │
│  │   Daemon     │ (Optional)                   │
│  │  (Go binary) │                              │
│  └──────────────┘                              │
│         │                                       │
│         └─── Mounts: ~/.claude/logs (read-only)│
│         └─── Mounts: repo path                 │
└─────────────────────────────────────────────────┘
```

## Quick Start

### Development Mode (Hot Reload)

```bash
# Clone repository
git clone https://github.com/your-username/claude-context-tracker.git
cd claude-context-tracker

# Start services
docker-compose -f docker-compose.dev.yml up

# Services will be available at:
# - Frontend: http://localhost:5173 (with hot reload)
# - PocketBase: http://localhost:8090
```

### Production Mode

```bash
# Build and start
docker-compose up -d

# Services will be available at:
# - Frontend: http://localhost:3000
# - PocketBase: http://localhost:8090
```

## Development Setup

### Starting Development Environment

```bash
# Start all services with logs
docker-compose -f docker-compose.dev.yml up

# Start in background
docker-compose -f docker-compose.dev.yml up -d

# View logs
docker-compose -f docker-compose.dev.yml logs -f

# View specific service logs
docker-compose -f docker-compose.dev.yml logs -f frontend
```

### Development Features

- **Hot Reload**: Frontend automatically reloads on code changes
- **Volume Mounting**: Source code is mounted, no rebuild needed
- **Local Database**: PocketBase data persists in `./pocketbase/pb_data`
- **Fast Iteration**: Changes reflect immediately

### Making Changes

1. Edit files in `frontend/src/`
2. Browser automatically refreshes
3. For backend changes, restart PocketBase:
   ```bash
   docker-compose -f docker-compose.dev.yml restart pocketbase
   ```

## Production Setup

### Building for Production

```bash
# Build all images
docker-compose build

# Build specific service
docker-compose build frontend

# Build with no cache (clean build)
docker-compose build --no-cache
```

### Starting Production Services

```bash
# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f
```

### Production Optimizations

The production setup includes:

- **Multi-stage builds** for minimal image sizes
- **Non-root users** for security
- **Health checks** for all services
- **Optimized nginx** configuration with gzip and caching
- **Named volumes** for data persistence

## Configuration

### Environment Variables

Create a `.env` file from the example:

```bash
cp .env.example .env
```

Edit `.env`:

```env
# Required for daemon
PROJECT_ID=your-project-id-from-pocketbase
REPO_PATH=/path/to/your/repository

# Optional overrides
PB_URL=http://pocketbase:8090
TZ=America/New_York
```

### Enabling the Daemon

1. Edit `.env` and set `PROJECT_ID`
2. Uncomment daemon service in `docker-compose.yml`:
   ```yaml
   daemon:
     build:
       context: ./daemon
     # ... rest of configuration
   ```
3. Restart:
   ```bash
   docker-compose up -d
   ```

### Port Configuration

To change default ports, edit `docker-compose.yml`:

```yaml
services:
  frontend:
    ports:
      - "8080:80"  # Change from 3000:80

  pocketbase:
    ports:
      - "9000:8090"  # Change from 8090:8090
```

## Docker Commands

### Service Management

```bash
# Start services
docker-compose up -d

# Stop services (preserves data)
docker-compose down

# Stop and remove volumes (deletes data!)
docker-compose down -v

# Restart all services
docker-compose restart

# Restart specific service
docker-compose restart frontend
```

### Viewing Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f pocketbase

# Last 100 lines
docker-compose logs --tail=100 frontend

# Since timestamp
docker-compose logs --since 2024-01-01T00:00:00
```

### Executing Commands

```bash
# Shell into a container
docker-compose exec pocketbase /bin/sh

# Run command in container
docker-compose exec frontend ls /usr/share/nginx/html

# Run as root
docker-compose exec -u root pocketbase /bin/sh
```

### Building and Updating

```bash
# Rebuild after code changes
docker-compose build

# Pull latest base images
docker-compose pull

# Build and start
docker-compose up -d --build

# Remove dangling images
docker image prune
```

## Troubleshooting

### Common Issues

#### Containers Exit Immediately

```bash
# Check logs for errors
docker-compose logs

# Check specific service
docker-compose logs pocketbase
```

#### Port Already in Use

```bash
# Find what's using the port
sudo lsof -i :8090

# Option 1: Kill the process
kill -9 <PID>

# Option 2: Change port in docker-compose.yml
```

#### Permission Denied

```bash
# Fix ownership
sudo chown -R $(id -u):$(id -g) ./pocketbase/pb_data

# Or run as root (not recommended)
sudo docker-compose up -d
```

#### Build Cache Issues

```bash
# Clean rebuild
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

#### Network Issues

```bash
# Recreate network
docker-compose down
docker network prune
docker-compose up -d
```

### Debugging

#### Check Container Health

```bash
# View status
docker-compose ps

# Inspect health
docker inspect cct-pocketbase | grep -A 10 Health

# Manual health check
docker-compose exec pocketbase wget -q -O- http://localhost:8090/api/health
```

#### Access Container Shell

```bash
# PocketBase
docker-compose exec pocketbase /bin/sh

# Frontend (nginx)
docker-compose exec frontend /bin/sh

# Daemon
docker-compose exec daemon /bin/sh
```

#### View Container Stats

```bash
# Real-time stats
docker stats

# Specific container
docker stats cct-pocketbase
```

## Advanced Topics

### Custom Nginx Configuration

Edit `frontend/nginx.conf` and rebuild:

```bash
docker-compose build frontend
docker-compose up -d frontend
```

### Using a Reverse Proxy

#### With Traefik

```yaml
# docker-compose.yml
services:
  frontend:
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.cct.rule=Host(`cct.example.com`)"
      - "traefik.http.routers.cct.entrypoints=websecure"
      - "traefik.http.routers.cct.tls.certresolver=letsencrypt"
```

#### With Nginx Proxy Manager

1. Don't publish ports in docker-compose.yml
2. Add services to proxy network
3. Configure in NPM dashboard

### Data Backup and Restore

#### Backup Volume

```bash
# Stop containers
docker-compose down

# Backup
docker run --rm \
  -v cct_pocketbase_data:/data \
  -v $(pwd):/backup \
  alpine tar czf /backup/pb-backup-$(date +%Y%m%d).tar.gz /data

# Restart
docker-compose up -d
```

#### Restore Volume

```bash
docker-compose down
docker run --rm \
  -v cct_pocketbase_data:/data \
  -v $(pwd):/backup \
  alpine tar xzf /backup/pb-backup-YYYYMMDD.tar.gz -C /
docker-compose up -d
```

#### Database Migration

```bash
# Export from old instance
docker-compose exec pocketbase ./pocketbase export backup.zip

# Copy to new instance
docker cp cct-pocketbase:/app/backup.zip ./

# Import on new instance
docker cp backup.zip cct-pocketbase:/app/
docker-compose exec pocketbase ./pocketbase import backup.zip
```

### Multi-Platform Builds

```bash
# Enable buildx
docker buildx create --use

# Build for multiple platforms
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t cct-frontend:latest \
  ./frontend
```

### Resource Limits

Add to `docker-compose.yml`:

```yaml
services:
  frontend:
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

### Security Hardening

1. **Run as non-root** (already implemented)
2. **Read-only filesystems**:
   ```yaml
   services:
     frontend:
       read_only: true
       tmpfs:
         - /tmp
         - /var/cache/nginx
   ```
3. **Drop capabilities**:
   ```yaml
   services:
     pocketbase:
       cap_drop:
         - ALL
       cap_add:
         - NET_BIND_SERVICE
   ```
4. **Use secrets**:
   ```yaml
   secrets:
     pb_admin_password:
       file: ./secrets/pb_admin_password.txt
   ```

### Monitoring

#### Prometheus + Grafana

Add monitoring stack:

```yaml
services:
  prometheus:
    image: prom/prometheus
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana
    ports:
      - "3001:3000"
```

#### Health Monitoring

```bash
# Check all services
docker-compose ps

# Continuous health monitoring
watch -n 5 docker-compose ps
```

### CI/CD Integration

#### GitHub Actions Example

```yaml
name: Deploy
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Deploy to server
        run: |
          ssh user@server "cd /app && git pull && docker-compose up -d --build"
```

## Best Practices

1. **Always use tagged images** in production (not `latest`)
2. **Pin versions** in Dockerfiles and compose files
3. **Use health checks** for all services
4. **Enable logging** drivers for centralized logs
5. **Regular backups** of volumes
6. **Monitor resource usage** and set limits
7. **Keep images updated** for security patches
8. **Use multi-stage builds** to minimize image size
9. **Never commit** `.env` files to git
10. **Test deployments** in staging before production

## Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [PocketBase Documentation](https://pocketbase.io/docs/)
- [Nginx Docker Image](https://hub.docker.com/_/nginx)

## Support

For issues or questions:
- Check logs: `docker-compose logs`
- Review [troubleshooting](#troubleshooting) section
- Open an issue on GitHub
- Check Docker daemon status: `systemctl status docker`
