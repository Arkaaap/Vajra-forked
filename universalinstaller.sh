#!/usr/bin/env bash

# ==============================================================
# Vajra Cross-Platform Installer
# --------------------------------------------------------------
# Builds Vajra in release mode and installs it system-wide.
# Additionally, it securely stores the VAJRA_API_KEY as an
# environment variable inside .bashrc or .zshrc (Linux/macOS)
# or Windows user environment variables.
#
# DATE: 3/2/2026
# ==============================================================

# Exit immediately if a command fails
set -e

# --------------------------------------------------------------
# Configuration Variables
# --------------------------------------------------------------
BINARY_NAME="vajra"
TARGET_DIR="target/release"
OS="$(uname -s)"
ENV_VAR_NAME="VAJRA_API_KEY"

# --------------------------------------------------------------
# Build the project in release mode
# --------------------------------------------------------------
echo "Building Vajra in release mode..."
echo "This may take some time. Please be patient."
cargo build --release

# --------------------------------------------------------------
# Detect Operating System & Define Install Directory
# --------------------------------------------------------------
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

# --------------------------------------------------------------
# Verify the compiled binary exists
# --------------------------------------------------------------
if [ ! -f "$TARGET_DIR/$BINARY_NAME" ]; then
    echo "Error: Compiled binary not found!"
    echo "Ensure BINARY_NAME matches your Cargo.toml output."
    exit 1
fi

echo "Installing to $INSTALL_DIR..."

# ==============================================================
# Windows Installation Section
# ==============================================================
if [[ "$OS" == CYGWIN* || "$OS" == MINGW* || "$OS" == MSYS* ]]; then

    # Create installation directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Copy compiled binary
    cp "$TARGET_DIR/$BINARY_NAME" "$INSTALL_DIR/"

    echo ""

    # Securely prompt for API key (input hidden)
    read -s -p "Enter your Vajra API Key (or press Enter to skip): " USER_API_KEY
    echo ""

    # If user provided API key, save it as Windows user environment variable
    if [ -n "$USER_API_KEY" ]; then
        setx $ENV_VAR_NAME "$USER_API_KEY"
        echo "✅ API key saved to Windows user environment."
        echo "⚠ Please restart your terminal to apply changes."
    fi

    echo ""
    echo "IMPORTANT:"
    echo "Add this directory to your Windows PATH if not already added:"
    echo "$INSTALL_DIR"

# ==============================================================
# macOS / Linux Installation Section
# ==============================================================
else

    # Copy binary with elevated privileges
    sudo cp "$TARGET_DIR/$BINARY_NAME" "$INSTALL_DIR/"

    # Ensure binary is executable
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

    echo ""

    # Securely prompt for API key (hidden input)
    read -s -p "Enter your Vajra API Key (or press Enter to skip): " USER_API_KEY
    echo ""

    # Only proceed if user entered a key
    if [ -n "$USER_API_KEY" ]; then

        # Detect user's default shell (bash or zsh)
        SHELL_NAME="$(basename "$SHELL")"

        if [[ "$SHELL_NAME" == "zsh" ]]; then
            SHELL_RC="$HOME/.zshrc"
        else
            SHELL_RC="$HOME/.bashrc"
        fi

        echo "Saving API key to $SHELL_RC ..."

        # Create rc file if it does not exist
        touch "$SHELL_RC"

        # Remove any previous VAJRA_API_KEY entry
        # A backup file (.bak) will be created automatically
        sed -i.bak "/export $ENV_VAR_NAME=/d" "$SHELL_RC"

        # Append new API key export line
        echo "export $ENV_VAR_NAME=\"$USER_API_KEY\"" >> "$SHELL_RC"

        # Export variable immediately for current session
        export $ENV_VAR_NAME="$USER_API_KEY"

        echo "✅ API key saved successfully."
        echo "⚠ Run: source $SHELL_RC"
        echo "   or restart your terminal to apply changes."
    fi
fi

# --------------------------------------------------------------
# Final Success Message
# --------------------------------------------------------------
echo ""
echo "************0************"
echo "Installation complete!"
echo ""
echo "You can now run:"
echo "$BINARY_NAME scan -t domain.com"
echo ""
echo "For advanced usage:"
echo "https://git.vulntech.com/mayur/Vajra/src/branch/master/COMMANDS.md"