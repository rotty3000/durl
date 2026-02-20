#!/bin/bash

# Copyright 2026 Raymond Auge <rayauge@doublebite.com>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -e

REPO="rotty3000/durl"
BINARY_NAME="durl"
GITHUB_API="https://api.github.com/repos/$REPO/releases/latest"

# download source to destination
# turbo
download() {
    if command -v curl >/dev/null 2>&1; then
        curl -sSL "$1" -o "$2"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$2" "$1"
    else
        echo "Error: curl or wget is required."
        exit 1
    fi
}

# fetch text from url
# turbo
fetch_text() {
    if command -v curl >/dev/null 2>&1; then
        curl -sSL "$1"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$1"
    else
        echo "Error: curl or wget is required."
        exit 1
    fi
}

# Detect OS and Architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64)  ARCH="amd64" ;;
    aarch64) ARCH="arm64" ;;
    armv7l)  ARCH="armhf" ;;
    *)       echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

if [ "$OS" != "linux" ]; then
    echo "This script is currently optimized for Linux."
    exit 1
fi

echo "Checking for latest release of $BINARY_NAME..."
RELEASE_DATA=$(fetch_text "$GITHUB_API")
LATEST_TAG=$(echo "$RELEASE_DATA" | grep '"tag_name":' | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "Could not fetch latest release tag from GitHub."
    exit 1
fi

LATEST_VERSION=${LATEST_TAG#v}

# Check if durl is already installed
CURRENT_VERSION=""
if command -v $BINARY_NAME >/dev/null 2>&1; then
    CURRENT_VERSION=$($BINARY_NAME --version | awk '{print $NF}')
fi

if [ "$CURRENT_VERSION" == "$LATEST_VERSION" ]; then
    echo "$BINARY_NAME is already up to date ($LATEST_VERSION)."
    exit 0
fi

if [ -n "$CURRENT_VERSION" ]; then
    echo "Updating $BINARY_NAME from $CURRENT_VERSION to $LATEST_VERSION..."
else
    echo "Installing $BINARY_NAME $LATEST_VERSION..."
fi

ASSET_NAME="${BINARY_NAME}-${OS}-${ARCH}"
DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url.*$ASSET_NAME" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Could not find a release asset for $ASSET_NAME in $LATEST_TAG"
    exit 1
fi

# Temp directory for download
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

echo "Downloading $DOWNLOAD_URL..."
download "$DOWNLOAD_URL" "$TMP_DIR/$BINARY_NAME"
chmod +x "$TMP_DIR/$BINARY_NAME"

# Determine installation directory
INSTALL_DIR="/usr/local/bin"
SUDO=""

if [ ! -w "$INSTALL_DIR" ]; then
    if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
        mkdir -p "$HOME/.local/bin"
        INSTALL_DIR="$HOME/.local/bin"
    else
        if command -v sudo >/dev/null 2>&1; then
            SUDO="sudo"
        else
            echo "Error: $INSTALL_DIR is not writable and 'sudo' is not available."
            echo "Please add $HOME/.local/bin to your PATH or run as root."
            exit 1
        fi
    fi
fi

echo "Installing to $INSTALL_DIR/$BINARY_NAME..."
$SUDO cp "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

echo "Done! $BINARY_NAME $LATEST_VERSION has been installed."
