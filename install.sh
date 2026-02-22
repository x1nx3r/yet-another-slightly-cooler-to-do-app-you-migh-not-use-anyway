#!/bin/bash

# Exit on error
set -e

APP_NAME="yet-another-slighty-cooler-to-do-list-app"
DISPLAY_NAME="Slightly Cooler To-Do List"
BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/128x128/apps"

echo "🚀 Starting installation of $DISPLAY_NAME..."

# Check for prerequisites
command -v bun >/dev/null 2>&1 || { echo >&2 "❌ Error: bun is not installed. Please install it first."; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo >&2 "❌ Error: cargo is not installed. Please install it first."; exit 1; }

# Install dependencies
echo "📦 Installing node dependencies..."
bun install

# Build the application
echo "🏗️ Building the release binary (this may take a minute)..."
bun tauri build

# Create directories if they don't exist
mkdir -p "$BIN_DIR"
mkdir -p "$APP_DIR"
mkdir -p "$ICON_DIR"

# Copy binary
echo "🚚 Installing binary to $BIN_DIR..."
cp "src-tauri/target/release/$APP_NAME" "$BIN_DIR/"

# Copy icon
echo "🖼️ Installing icon..."
cp "src-tauri/icons/128x128.png" "$ICON_DIR/$APP_NAME.png"

# Create .desktop file
echo "🖥️ Creating desktop entry..."
cat > "$APP_DIR/$APP_NAME.desktop" <<EOF
[Desktop Entry]
Name=$DISPLAY_NAME
Comment=A minimalist task widget suite for Linux
Exec=$BIN_DIR/$APP_NAME
Icon=$APP_NAME
Terminal=false
Type=Application
Categories=Utility;
EOF

chmod +x "$APP_DIR/$APP_NAME.desktop"

echo "✅ Installation complete!"
echo "✨ You can now find '$DISPLAY_NAME' in your application launcher."
echo "💡 Note: Make sure $BIN_DIR is in your PATH."
