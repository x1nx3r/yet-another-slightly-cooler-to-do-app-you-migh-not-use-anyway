#!/bin/bash

# Exit on error
set -e

APP_NAME="yet-another-slighty-cooler-to-do-list-app"
DISPLAY_NAME="Slightly Cooler To-Do List"
BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/128x128/apps"

echo "🗑️ Starting uninstallation of $DISPLAY_NAME..."

# Remove binary
if [ -f "$BIN_DIR/$APP_NAME" ]; then
    echo "🚚 Removing binary from $BIN_DIR..."
    rm "$BIN_DIR/$APP_NAME"
fi

# Remove icon
if [ -f "$ICON_DIR/$APP_NAME.png" ]; then
    echo "🖼️ Removing icon..."
    rm "$ICON_DIR/$APP_NAME.png"
fi

# Remove .desktop file
if [ -f "$APP_DIR/$APP_NAME.desktop" ]; then
    echo "🖥️ Removing desktop entry..."
    rm "$APP_DIR/$APP_NAME.desktop"
fi

echo "✅ Uninstallation complete!"
echo "👋 $DISPLAY_NAME has been removed from your system."
