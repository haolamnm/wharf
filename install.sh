#!/bin/sh

set -euo pipefail

REPO="haolamnm/wharf"
BIN_NAME="wharf"
INSTALL_DIR="$HOME/.local/bin"

# Create install dir if missing
mkdir -p "$INSTALL_DIR"

# Download latest release binary
echo "Downloading $BIN_NAME from https://github.com/$REPO/releases/latest..."
curl -L "https://github.com/$REPO/releases/latest/download/$BIN_NAME" -o "$INSTALL_DIR/$BIN_NAME"

# Make executable
chmod +x "$INSTALL_DIR/$BIN_NAME"

echo "Installed $BIN_NAME to $INSTALL_DIR"
echo "Make sure $INSTALL_DIR is in your \$PATH"
echo "Run 'wharf --help' to get started!"
