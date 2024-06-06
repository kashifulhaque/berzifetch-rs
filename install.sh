#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

command_exists () {
  command -v "$1" &> /dev/null
}

if ! command_exists rustc; then
  echo "Error: rustc (Rust compiler) is not installed."
  exit 1
fi

if ! command_exists cargo; then
  echo "Error: cargo (Rust package manager) is not installed."
  exit 1
fi

echo "Building the release executable..."
cargo build --release

EXECUTABLE_PATH="./target/release/berzifetch"
if [ ! -f "$EXECUTABLE_PATH" ]; then
  echo "[ERROR] Build failed: Executable NOT found at $EXECUTABLE_PATH"
  exit 1
fi

INSTALL_DIR="/usr/local/bin"
if [ ! -d "$INSTALL_DIR" ]; then
  echo "Creating installation directory $INSTALL_DIR"
  sudo mkdir -p "$INSTALL_DIR"
fi

echo "Installing the executable to $INSTALL_DIR"
sudo cp "$EXECUTABLE_PATH" "$INSTALL_DIR"
sudo chmod +x "$INSTALL_DIR/berzifetch"