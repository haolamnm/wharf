#!/bin/bash
#
# wharf - uninstaller

set -euo pipefail

SCRIPT_NAME="wharf"
TARGET_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/wharf"

echo "Uninstalling $SCRIPT_NAME..."

# Remove symlink or binary
if [ -L "$TARGET_DIR/$SCRIPT_NAME" ]; then
    rm "$TARGET_DIR/$SCRIPT_NAME"
    echo "Removed symlink: $TARGET_DIR/$SCRIPT_NAME"
elif [ -f "$TARGET_DIR/$SCRIPT_NAME" ]; then
    rm "$TARGET_DIR/$SCRIPT_NAME"
    echo "Removed binary: $TARGET_DIR/$SCRIPT_NAME"
else
    echo "$SCRIPT_NAME not found in $TARGET_DIR"
fi

# Ask about removing config
read -p "Remove config files and descriptions? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -d "$CONFIG_DIR" ]; then
        rm -rf "$CONFIG_DIR"
        echo "Removed config: $CONFIG_DIR"
    else
        echo "Config directory not found: $CONFIG_DIR"
    fi
else
    echo "Config files preserved at: $CONFIG_DIR"
fi

echo "Uninstall completed!"
