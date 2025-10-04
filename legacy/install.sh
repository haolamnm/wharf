#!/bin/bash
#
# wharf - installer

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPT_NAME="wharf"
TARGET_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/wharf"

echo "Installing $SCRIPT_NAME..."

# Check if main script exists
if [ ! -f "$SCRIPT_DIR/$SCRIPT_NAME" ]; then
    echo "Error: Main script '$SCRIPT_NAME' not found in $SCRIPT_DIR"
    exit 1
fi

# Create target and config directories
mkdir -p "$TARGET_DIR" "$CONFIG_DIR"

# Make main script executable
chmod +x "$SCRIPT_DIR/$SCRIPT_NAME"

# Create symlink
ln -sf "$SCRIPT_DIR/$SCRIPT_NAME" "$TARGET_DIR/$SCRIPT_NAME"

# Check dependencies
if ! command -v jq &>/dev/null; then
    echo "Warning: jq is not installed. Install with: sudo apt install jq."
fi
if ! command -v realpath &>/dev/null; then
    echo "Warning: realpath not installed. Some features may not work."
fi

# Verify installation
if [ -L "$TARGET_DIR/$SCRIPT_NAME" ] || [ -f "$TARGET_DIR/$SCRIPT_NAME" ]; then
    echo "$SCRIPT_NAME installed successfully!"
    echo "Location: $TARGET_DIR/$SCRIPT_NAME"
    echo "Config: $CONFIG_DIR/descriptions.json"

    # Check if target dir is in PATH
    if [[ ":$PATH:" != *":$TARGET_DIR:"* ]]; then
        echo ""
        echo "Warning: $TARGET_DIR is not in your PATH."
        echo "Add this to your ~/.bashrc or ~/.zshrc:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    else
        echo "Available as '$SCRIPT_NAME' command"
    fi
else
    echo "Installation failed!"
    exit 1
fi
