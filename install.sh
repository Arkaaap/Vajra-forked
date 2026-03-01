#!/bin/bash

# -----------------------------
# Rust Project Installer Script
# -----------------------------
# This script:
# 1. Builds the project in release mode
# 2. Copies the compiled binary to /usr/local/bin
# 3. Makes it executable
# 4. Allows you to run it from anywhere

# Change this to match your binary name
# (It must match the name produced in target/release/)
BINARY_NAME="vajra"

# Step 1: Build the Rust project in release mode
# --release creates an optimized binary

echo "Building project in release mode..."
echo -e "\e[32mIt will take time please be pateint\e[0m "
cargo build --release

# Step 2: Check if the binary was created successfully
if [ ! -f "target/release/$BINARY_NAME" ]; then
    echo -e "\e[31mError: Binary not found!\e[0m"
    echo -e "\e[31mMake sure BINARY_NAME is correct.\e[0m"
    exit 1
fi

# Step 3: Copy the binary to /usr/local/bin
# This makes it accessible system-wide
echo -e "\e[32mInstalling binary to /usr/local/bin...\e[0m"
sudo cp target/release/$BINARY_NAME /usr/local/bin/

# Step 4: Ensure it has execute permission
sudo chmod +x /usr/local/bin/$BINARY_NAME

# Done
echo -e "\e[32m************0************\e[0m"
echo -e "\e[32mInstallation complete!\e[0m"
echo -e "You can now run the program from anywhere using:"
echo "$BINARY_NAME"
echo -e "\e[33mBasic syntax $BINARY_NAME scan -t domain.com\e[0m"



echo -e "\e[31mFor more extensive scan please visit out https://git.vulntech.com/mayur/Vajra/src/branch/master/COMMANDS.md\e[0m "

