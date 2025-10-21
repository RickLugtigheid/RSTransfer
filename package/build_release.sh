#!/bin/bash

# -----------------------------------------------------------------------------
# build_release.sh
#
# This script builds release binaries for the `rst-cli` Rust project.
# It supports building for multiple targets including Debian (.deb package),
# and Windows (cross-compilation).
#
# Usage:
#   ./build_release.sh --target debian
#   ./build_release.sh --target windows
#   ./build_release.sh --all
#
# Requirements:
#   - Rust toolchain (`rustc`, `cargo`)
#   - `dpkg-deb` for Debian packaging
#
# Notes:
#   - The script auto-detects the version from Cargo.toml
#   - Uses `rustup` to add compilation targets
#   - Only escalates privileges for the parts that require root
# -----------------------------------------------------------------------------

set -euo pipefail

# ---- Utility Functions ----

function command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Run with sudo or just run if already root
function run_root_or_sudo() {
    if [[ $EUID -ne 0 ]]; then
        if command_exists sudo; then
            sudo "$@"
        else
            echo "Error: sudo not found; please run with sudo or as root"
            exit 1
        fi
    else
        "$@"
    fi
}

function require_command() {
    if ! command_exists "$1"; then
        echo "Error: command '$1' not found"

        read -p "Would you like to try to install '$1'? [y/N] " yn
        case "$yn" in
            [Yy]* ) run_root_or_sudo apt-get install -y "$1" ;;
            * ) echo "Aborting. Please install '$1' manually."; exit 1 ;;
        esac
    fi
}

function require_root() {
    if [[ $EUID -ne 0 ]]; then
        echo "This step requires root."
        exit 1
    fi
}

function print_usage() {
    echo "Usage: $0 [--target <target>] [--all]"
    echo ""
    echo "Options:"
    echo "  -t, --target <target>  Build for the specified target"
    echo "  --all                Build for all targets"
    echo ""
    echo "Targets:"
    echo "  debian            Build Debian package"
    echo "  windows           Build Windows binaries"
}

# ---- Initial Setup ----

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
SRC_DIR="$SCRIPT_DIR/../rst-cli"

if [[ ! -f "$SRC_DIR/Cargo.toml" ]]; then
    echo "Error: Cannot find Cargo.toml in $SRC_DIR"
    exit 1
fi

VERSION="$(grep -m 1 '^version\s*=' "$SRC_DIR/Cargo.toml" | cut -d '"' -f 2)"
echo "Building for version $VERSION"

TARGETS=()

# ---- Parse Arguments ----

while [[ $# -gt 0 ]]; do
    case "$1" in
        -t|--target)
            TARGETS+=("$2")
            shift
            ;;
        --all)
            TARGETS=(debian windows)
            ;;
        *)
            echo "Error: Unknown argument '$1'"
            print_usage
            exit 1
            ;;
    esac
    shift
done

# No targets? Show usage
[[ ${#TARGETS[@]} -eq 0 ]] && print_usage && exit 1

# Check for required commands
require_command "rustc"
require_command "cargo"

# ---- Build Functions ----

function build_debian() {
    BUILD_TARGET="x86_64-unknown-linux-gnu"
    echo "Building Debian package..."

    # Create a temporary directory for the debian package
    TEMP_PACKAGE_DIR=$(mktemp -d)
    cp -r "$SCRIPT_DIR" "$TEMP_PACKAGE_DIR"

    # Update the version in the control file dynamically
    CONTROL_FILE="$TEMP_PACKAGE_DIR/package/DEBIAN/control"
    if [[ -f "$CONTROL_FILE" ]]; then
        # Update version in the control file
        sed -i "s/^Version: .*/Version: $VERSION/" "$CONTROL_FILE"
    else
        echo "Error: Control file not found in $TEMP_PACKAGE_DIR/package/DEBIAN/"
        exit 1
    fi

    # Clean release build directory
    echo "Cleaning release build directory..."
    rm -rf "$SRC_DIR/../target"

    # Build the Rust release binary
    echo "Building the Rust binary for Debian..."
    CURRENT_DIR=$(pwd)
    cd "$SRC_DIR"
    cargo build --release --target $BUILD_TARGET
    cd "$CURRENT_DIR"

    # Create the package binary directory
    BIN_DIR="$TEMP_PACKAGE_DIR/package/usr/local/bin"
    mkdir -p "$BIN_DIR"
    cp "$SRC_DIR/../target/$BUILD_TARGET/release/rst" "$BIN_DIR/"

    # Build the Debian package
    dpkg-deb --build "$TEMP_PACKAGE_DIR/package"

    # Move the .deb file to the current directory
    mv "$TEMP_PACKAGE_DIR/package.deb" "./rst-$VERSION-amd64.deb"

    # Clean up
    rm -rf "$TEMP_PACKAGE_DIR"

    echo "Debian package created: rst-cli-$VERSION-amd64.deb (in $PWD)"
}

function build_windows() {
    BUILD_TARGET="x86_64-pc-windows-gnu"
    echo "Building for Windows..."

    require_command "rustup"

    # Install the Windows target if not already installed
    if ! rustup target list | grep -q "$BUILD_TARGET"; then
        echo "Installing Windows target..."
        rustup target add $BUILD_TARGET
    fi

    # Build the Windows binary in release mode
    cargo build --release --target $BUILD_TARGET

    # Copy the binary to a Windows-compatible location
    BIN_DIR="target/$BUILD_TARGET/release"
    cp "$BIN_DIR/rst-cli.exe" "./rst-$VERSION-x86_64-windows.exe"

    echo "Windows binary created: rst-$VERSION-x86_64-windows.exe (in $PWD)"

    # TODO: We might want to create a ZIP file with the Windows binary and / or an MSI installer
}

# ---- Run ----

for TARGET in "${TARGETS[@]}"; do
    echo "---- Building for $TARGET ----"
    case "$TARGET" in
        debian) build_debian ;;
        windows) build_windows ;;
        *) echo "Error: Unknown target '$TARGET'"; exit 1 ;;
    esac
done

echo "---- All builds completed successfully ----"