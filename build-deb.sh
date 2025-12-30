#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Building CCT .deb Package${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════════${NC}"
echo ""

# Configuration
VERSION="1.0.0"
ARCH="amd64"
PACKAGE_NAME="cct_${VERSION}_${ARCH}"
BUILD_DIR="build/${PACKAGE_NAME}"

# Clean previous builds
echo -e "${YELLOW}→${NC} Cleaning previous builds..."
rm -rf build/
mkdir -p "${BUILD_DIR}"

# Create directory structure
echo -e "${YELLOW}→${NC} Creating directory structure..."
mkdir -p "${BUILD_DIR}/DEBIAN"
mkdir -p "${BUILD_DIR}/usr/bin"
mkdir -p "${BUILD_DIR}/usr/share/cct"
mkdir -p "${BUILD_DIR}/usr/share/doc/cct"
mkdir -p "${BUILD_DIR}/etc/systemd/system"
mkdir -p "${BUILD_DIR}/var/lib/cct/pocketbase"

# Build CLI
echo -e "${YELLOW}→${NC} Building CLI..."
cd cli
go build -ldflags "-s -w" -o "../${BUILD_DIR}/usr/bin/cct"
cd ..

# Build Daemon
echo -e "${YELLOW}→${NC} Building Daemon..."
cd daemon
go build -ldflags "-s -w" -o "../${BUILD_DIR}/usr/bin/cct-daemon"
cd ..

# Download PocketBase
echo -e "${YELLOW}→${NC} Downloading PocketBase..."
wget -q https://github.com/pocketbase/pocketbase/releases/latest/download/pocketbase_linux_amd64.zip -O /tmp/pocketbase.zip
unzip -q /tmp/pocketbase.zip -d /tmp/
mv /tmp/pocketbase "${BUILD_DIR}/usr/bin/cct-pocketbase"
chmod +x "${BUILD_DIR}/usr/bin/cct-pocketbase"
rm /tmp/pocketbase.zip

# Copy PocketBase migrations
echo -e "${YELLOW}→${NC} Copying PocketBase migrations..."
if [ -d "pocketbase/pb_migrations" ]; then
    cp -r pocketbase/pb_migrations "${BUILD_DIR}/var/lib/cct/pocketbase/"
fi

# Copy frontend source
echo -e "${YELLOW}→${NC} Copying frontend..."
cp -r frontend "${BUILD_DIR}/usr/share/cct/"
# Remove node_modules if exists (will be installed by user)
rm -rf "${BUILD_DIR}/usr/share/cct/frontend/node_modules"

# Copy documentation
echo -e "${YELLOW}→${NC} Copying documentation..."
cp README.md "${BUILD_DIR}/usr/share/doc/cct/"
cp CLAUDE.md "${BUILD_DIR}/usr/share/doc/cct/" 2>/dev/null || true

# Create DEBIAN control file
echo -e "${YELLOW}→${NC} Creating control file..."
cp debian/control "${BUILD_DIR}/DEBIAN/control"

# Copy systemd service files
echo -e "${YELLOW}→${NC} Installing systemd service files..."
cp debian/cct-pocketbase.service "${BUILD_DIR}/etc/systemd/system/"
cp debian/cct-daemon.service.template "${BUILD_DIR}/usr/share/cct/"

# Copy maintainer scripts
echo -e "${YELLOW}→${NC} Installing maintainer scripts..."
cp debian/postinst "${BUILD_DIR}/DEBIAN/"
cp debian/prerm "${BUILD_DIR}/DEBIAN/"
cp debian/postrm "${BUILD_DIR}/DEBIAN/"
chmod 755 "${BUILD_DIR}/DEBIAN/postinst"
chmod 755 "${BUILD_DIR}/DEBIAN/prerm"
chmod 755 "${BUILD_DIR}/DEBIAN/postrm"

# Calculate installed size
echo -e "${YELLOW}→${NC} Calculating package size..."
INSTALLED_SIZE=$(du -sk "${BUILD_DIR}" | cut -f1)
echo "Installed-Size: ${INSTALLED_SIZE}" >> "${BUILD_DIR}/DEBIAN/control"

# Build the package
echo -e "${YELLOW}→${NC} Building .deb package..."
dpkg-deb --build "${BUILD_DIR}" "build/${PACKAGE_NAME}.deb"

# Show results
echo ""
echo -e "${GREEN}════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Build Complete!${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "Package: ${GREEN}build/${PACKAGE_NAME}.deb${NC}"
echo -e "Size: ${GREEN}$(du -h build/${PACKAGE_NAME}.deb | cut -f1)${NC}"
echo ""
echo "To install:"
echo -e "  ${YELLOW}sudo apt install ./build/${PACKAGE_NAME}.deb${NC}"
echo ""
echo "To remove:"
echo -e "  ${YELLOW}sudo apt remove cct${NC}"
echo ""
echo "To purge (including data):"
echo -e "  ${YELLOW}sudo apt purge cct${NC}"
echo ""
