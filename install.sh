#!/bin/bash

# Define variables
RELEASE_URL="https://github.com/DeForestt/my_notes/releases/latest"
DOWNLOAD_URL=$(curl -sI $RELEASE_URL | grep -i "location" | awk -F' ' '{print $2}' | tr -d '\r')
FILENAME=$(basename $DOWNLOAD_URL)
INSTALL_DIR="/usr/local/bin"

# Download the release
curl -LJO $DOWNLOAD_URL

# Extract the binary (assuming it's a single file)
tar -zxvf $FILENAME

# Move the binary to the installation directory
sudo mv my-notes $INSTALL_DIR

# Clean up downloaded files
rm $FILENAME

# Provide executable permissions
sudo chmod +x $INSTALL_DIR/my-notes

# Confirm installation
my-notes --version

echo "My Notes has been installed successfully!"