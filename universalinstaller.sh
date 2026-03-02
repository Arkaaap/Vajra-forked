#!/usr/bin/env bash

# ----------------------------------
# Vajra Cross-Platform Installer
# ----------------------------------
#DATE 3/2/2026
set -e

BINARY_NAME="vajra"
TARGET_DIR="target/release"
OS="$(uname -s)"

echo "Building Vajra in release mode..."
echo "This may take some time. Please be patient."
cargo build --release

# Detect OS and set install path
case "$OS" in
    Linux*)
        INSTALL_DIR="/usr/local/bin"
        ;;
    Darwin*)
        INSTALL_DIR="/usr/local/bin"
        ;;
    CYGWIN*|MINGW*|MSYS*)
        INSTALL_DIR="/c/Program Files/Vajra"
        BINARY_NAME="vajra.exe"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Verify binary exists
if [ ! -f "$TARGET_DIR/$BINARY_NAME" ]; then
    echo "Error: Compiled binary not found!"
    echo "Check that BINARY_NAME matches Cargo.toml output."
    exit 1
fi

echo "Installing to $INSTALL_DIR..."

# Windows install
if [[ "$OS" == CYGWIN* || "$OS" == MINGW* || "$OS" == MSYS* ]]; then
    mkdir -p "$INSTALL_DIR"
    cp "$TARGET_DIR/$BINARY_NAME" "$INSTALL_DIR/"
    echo ""
    echo "IMPORTANT:"
    echo "Add this to your Windows PATH if not already added:"
    echo "$INSTALL_DIR"
else
    # macOS / Linux
    sudo cp "$TARGET_DIR/$BINARY_NAME" "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
fi

echo ""
echo "************0************"
echo "Installation complete!"
echo ""
echo "You can now run:"
echo "$BINARY_NAME scan -t domain.com"
echo ""
echo "For advanced usage:"
echo "https://git.vulntech.com/mayur/Vajra/src/branch/master/COMMANDS.md"