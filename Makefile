# Claude Context Tracker - Makefile

.PHONY: all install build clean dev test help

# Default target
all: build

# Install dependencies
install:
	@echo "Installing dependencies..."
	cd frontend && npm install
	cd daemon && go mod download
	cd cli && go mod download
	@echo "✓ Dependencies installed"

# Build all components
build: build-frontend build-daemon build-cli
	@echo "✓ All components built"

# Build frontend
build-frontend:
	@echo "Building frontend..."
	cd frontend && npm run build
	@echo "✓ Frontend built"

# Build daemon
build-daemon:
	@echo "Building daemon..."
	cd daemon && go build -o cct-daemon
	@echo "✓ Daemon built"

# Build CLI
build-cli:
	@echo "Building CLI..."
	cd cli && go build -o cct
	@echo "✓ CLI built"

# Install binaries globally
install-binaries:
	@echo "Installing binaries..."
	sudo cp daemon/cct-daemon /usr/local/bin/
	sudo cp cli/cct /usr/local/bin/
	@echo "✓ Binaries installed to /usr/local/bin"

# Development mode (run all services)
dev:
	@echo "Starting development environment..."
	@echo "Start PocketBase in one terminal:"
	@echo "  cd pocketbase && ./pocketbase serve"
	@echo ""
	@echo "Start frontend in another terminal:"
	@echo "  cd frontend && npm run dev"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf frontend/dist
	rm -f daemon/cct-daemon
	rm -f cli/cct
	@echo "✓ Cleaned"

# Run tests
test:
	@echo "Running tests..."
	cd frontend && npm run lint
	cd daemon && go test ./...
	cd cli && go test ./...
	@echo "✓ Tests passed"

# Download PocketBase (Linux)
download-pocketbase-linux:
	@echo "Downloading PocketBase for Linux..."
	cd pocketbase && \
		wget https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_linux_amd64.zip && \
		unzip -o pocketbase_linux_amd64.zip && \
		rm pocketbase_linux_amd64.zip && \
		chmod +x pocketbase
	@echo "✓ PocketBase downloaded"

# Download PocketBase (macOS)
download-pocketbase-macos:
	@echo "Downloading PocketBase for macOS..."
	cd pocketbase && \
		curl -LO https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_darwin_amd64.zip && \
		unzip -o pocketbase_darwin_amd64.zip && \
		rm pocketbase_darwin_amd64.zip && \
		chmod +x pocketbase
	@echo "✓ PocketBase downloaded"

# Help
help:
	@echo "Claude Context Tracker - Make targets:"
	@echo ""
	@echo "  make install                  - Install all dependencies"
	@echo "  make build                    - Build all components"
	@echo "  make build-frontend           - Build frontend only"
	@echo "  make build-daemon             - Build daemon only"
	@echo "  make build-cli                - Build CLI only"
	@echo "  make install-binaries         - Install binaries to /usr/local/bin"
	@echo "  make clean                    - Clean build artifacts"
	@echo "  make test                     - Run tests"
	@echo "  make dev                      - Show dev environment instructions"
	@echo "  make download-pocketbase-linux - Download PocketBase (Linux)"
	@echo "  make download-pocketbase-macos - Download PocketBase (macOS)"
	@echo "  make help                     - Show this help message"
