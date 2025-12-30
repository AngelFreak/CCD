# PocketBase Dockerfile
FROM alpine:latest

# Install required packages
RUN apk add --no-cache \
    ca-certificates \
    unzip \
    wget

WORKDIR /app

# Download and install PocketBase
# Using ARG to allow overriding the version
ARG PB_VERSION=0.20.0
ARG TARGETARCH=amd64

RUN wget -q https://github.com/pocketbase/pocketbase/releases/download/v${PB_VERSION}/pocketbase_${PB_VERSION}_linux_${TARGETARCH}.zip \
    && unzip pocketbase_${PB_VERSION}_linux_${TARGETARCH}.zip \
    && rm pocketbase_${PB_VERSION}_linux_${TARGETARCH}.zip \
    && chmod +x pocketbase

# Copy migrations
COPY pb_migrations /app/pb_migrations

# Create directory for data
RUN mkdir -p /app/pb_data

# Run as non-root user
RUN addgroup -g 1000 pocketbase && \
    adduser -D -u 1000 -G pocketbase pocketbase && \
    chown -R pocketbase:pocketbase /app

USER pocketbase

# Expose port
EXPOSE 8090

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8090/api/health || exit 1

# Start PocketBase
CMD ["./pocketbase", "serve", "--http=0.0.0.0:8090"]
