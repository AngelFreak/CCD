# Multi-stage build for Go daemon
FROM golang:1.21-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git

WORKDIR /build

# Copy go mod files
COPY go.mod go.sum* ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the binary
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags="-w -s" -o cct-daemon .

# Production stage
FROM alpine:latest

# Install ca-certificates for HTTPS requests
RUN apk --no-cache add ca-certificates

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/cct-daemon .

# Create directory for logs (if daemon needs to write logs)
RUN mkdir -p /app/logs

# Run as non-root user
RUN addgroup -g 1000 cct && \
    adduser -D -u 1000 -G cct cct && \
    chown -R cct:cct /app

USER cct

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD pgrep cct-daemon || exit 1

ENTRYPOINT ["./cct-daemon"]
