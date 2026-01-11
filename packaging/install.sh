#!/usr/bin/env bash
# aiandi installation script
# Downloads and installs the latest aiandi binary from GitHub releases

set -euo pipefail

# Configuration
REPO="pentaxis93/aiandi"
INSTALL_DIR="${AIANDI_INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${AIANDI_VERSION:-}"
VERBOSE="${AIANDI_VERBOSE:-false}"

# Colors
if [[ -t 1 ]]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    BOLD='\033[1m'
    RESET='\033[0m'
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    BOLD=''
    RESET=''
fi

# Utility functions
info() {
    echo -e "${BLUE}==>${RESET} $*"
}

success() {
    echo -e "${GREEN}✓${RESET} $*"
}

warn() {
    echo -e "${YELLOW}⚠${RESET} $*"
}

error() {
    echo -e "${RED}✗ Error:${RESET} $*" >&2
}

verbose() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${BLUE}[verbose]${RESET} $*"
    fi
}

show_help() {
    cat <<EOF
${BOLD}aiandi installer${RESET}

Downloads and installs aiandi from GitHub releases.

${BOLD}USAGE:${RESET}
    curl -fsSL https://raw.githubusercontent.com/$REPO/main/packaging/install.sh | bash
    ./install.sh [OPTIONS]

${BOLD}OPTIONS:${RESET}
    -h, --help          Show this help message
    -v, --verbose       Enable verbose output
    --check             Check current and latest versions
    --uninstall         Remove aiandi from your system
    --version VERSION   Install specific version (e.g., v0.1.0)

${BOLD}ENVIRONMENT VARIABLES:${RESET}
    AIANDI_INSTALL_DIR  Installation directory (default: ~/.local/bin)
    AIANDI_VERSION      Specific version to install (default: latest)
    AIANDI_VERBOSE      Enable verbose output (true/false)

${BOLD}EXAMPLES:${RESET}
    # Install latest version
    curl -fsSL https://raw.githubusercontent.com/$REPO/main/packaging/install.sh | bash

    # Install to custom directory
    AIANDI_INSTALL_DIR=/usr/local/bin ./install.sh

    # Install specific version
    AIANDI_VERSION=v0.1.0 ./install.sh

    # Check for updates
    ./install.sh --check

    # Uninstall
    ./install.sh --uninstall

EOF
}

# Detect platform
detect_platform() {
    local os arch target

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            case "$arch" in
                x86_64)
                    target="x86_64-unknown-linux-gnu"
                    ;;
                *)
                    error "Unsupported Linux architecture: $arch"
                    echo ""
                    echo "Supported platforms:"
                    echo "  - Linux x86_64"
                    echo "  - macOS x86_64"
                    echo "  - macOS ARM64"
                    echo ""
                    echo "For other platforms, build from source:"
                    echo "  cargo install --git https://github.com/$REPO --package aiandi"
                    exit 1
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)
                    target="x86_64-apple-darwin"
                    ;;
                arm64)
                    target="aarch64-apple-darwin"
                    ;;
                *)
                    error "Unsupported macOS architecture: $arch"
                    exit 1
                    ;;
            esac
            ;;
        *)
            error "Unsupported operating system: $os"
            echo ""
            echo "Supported platforms:"
            echo "  - Linux x86_64"
            echo "  - macOS x86_64"
            echo "  - macOS ARM64"
            exit 1
            ;;
    esac

    echo "$target"
}

# Check for required tools
check_dependencies() {
    local downloader=""

    if command -v curl >/dev/null 2>&1; then
        downloader="curl"
    elif command -v wget >/dev/null 2>&1; then
        downloader="wget"
    else
        error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi

    if ! command -v tar >/dev/null 2>&1; then
        error "tar not found. Please install tar."
        exit 1
    fi

    echo "$downloader"
}

# Fetch latest version from GitHub API
fetch_latest_version() {
    local api_url="https://api.github.com/repos/$REPO/releases/latest"
    local downloader="$1"
    local response

    verbose "Querying GitHub API: $api_url"

    if [[ "$downloader" == "curl" ]]; then
        response=$(curl -fsSL "$api_url" 2>/dev/null || echo "")
    else
        response=$(wget -qO- "$api_url" 2>/dev/null || echo "")
    fi

    if [[ -z "$response" ]]; then
        error "Failed to fetch latest version from GitHub"
        exit 1
    fi

    # Extract tag_name using grep and sed (portable)
    local version
    version=$(echo "$response" | grep '"tag_name"' | head -n1 | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')

    if [[ -z "$version" ]]; then
        error "Failed to parse version from GitHub API response"
        exit 1
    fi

    echo "$version"
}

# Download file
download() {
    local url="$1"
    local output="$2"
    local downloader="$3"

    verbose "Downloading $url to $output"

    if [[ "$downloader" == "curl" ]]; then
        if ! curl -fsSL "$url" -o "$output"; then
            error "Failed to download $url"
            return 1
        fi
    else
        if ! wget -q "$url" -O "$output"; then
            error "Failed to download $url"
            return 1
        fi
    fi

    return 0
}

# Get installed version
get_installed_version() {
    local bin_path="$INSTALL_DIR/aiandi"

    if [[ ! -x "$bin_path" ]]; then
        echo ""
        return
    fi

    # Get version from aiandi --version
    local version
    version=$("$bin_path" --version 2>/dev/null | awk '{print $2}' || echo "")

    echo "$version"
}

# Check version
check_version() {
    local downloader
    downloader=$(check_dependencies)

    local installed
    installed=$(get_installed_version)

    local latest
    info "Checking latest version..."
    latest=$(fetch_latest_version "$downloader")

    echo ""
    if [[ -n "$installed" ]]; then
        echo -e "${BOLD}Installed version:${RESET} $installed"
    else
        echo -e "${BOLD}Installed version:${RESET} not installed"
    fi

    echo -e "${BOLD}Latest version:${RESET}    $latest"
    echo ""

    if [[ -n "$installed" && "$installed" != "${latest#v}" ]]; then
        info "Update available: $installed → ${latest#v}"
        echo ""
        echo "To update, run:"
        echo "  curl -fsSL https://raw.githubusercontent.com/$REPO/main/packaging/install.sh | bash"
    elif [[ -n "$installed" ]]; then
        success "You have the latest version installed"
    else
        info "aiandi is not installed"
        echo ""
        echo "To install, run:"
        echo "  curl -fsSL https://raw.githubusercontent.com/$REPO/main/packaging/install.sh | bash"
    fi
}

# Uninstall
uninstall() {
    local bin_path="$INSTALL_DIR/aiandi"
    local config_dir="$HOME/.config/aiandi"

    if [[ ! -f "$bin_path" ]]; then
        warn "aiandi is not installed at $bin_path"
        return 0
    fi

    info "Removing aiandi binary from $bin_path..."
    rm -f "$bin_path"
    success "Binary removed"

    if [[ -d "$config_dir" ]]; then
        echo ""
        read -p "Remove configuration directory $config_dir? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf "$config_dir"
            success "Configuration removed"
        else
            info "Configuration kept at $config_dir"
        fi
    fi

    echo ""
    success "aiandi has been uninstalled"
}

# Main installation function
install() {
    local platform downloader version url tarball temp_dir

    # Detect platform
    info "Detecting platform..."
    platform=$(detect_platform)
    verbose "Platform: $platform"

    # Check dependencies
    downloader=$(check_dependencies)
    verbose "Downloader: $downloader"

    # Get version
    if [[ -z "$VERSION" ]]; then
        info "Fetching latest version..."
        version=$(fetch_latest_version "$downloader")
    else
        version="$VERSION"
        info "Using specified version: $version"
    fi

    verbose "Version: $version"

    # Construct download URL
    tarball="aiandi-${platform}.tar.gz"
    url="https://github.com/$REPO/releases/download/${version}/${tarball}"

    # Create temp directory
    temp_dir=$(mktemp -d)
    trap 'rm -rf "$temp_dir"' EXIT

    # Download tarball
    info "Downloading aiandi ${version}..."
    if ! download "$url" "$temp_dir/$tarball" "$downloader"; then
        error "Download failed"
        echo ""
        echo "URL: $url"
        echo ""
        echo "Please check:"
        echo "  - Version exists: https://github.com/$REPO/releases"
        echo "  - Network connectivity"
        exit 1
    fi

    # Extract tarball
    verbose "Extracting tarball..."
    if ! tar -xzf "$temp_dir/$tarball" -C "$temp_dir"; then
        error "Failed to extract tarball"
        exit 1
    fi

    # Create install directory
    if [[ ! -d "$INSTALL_DIR" ]]; then
        verbose "Creating install directory: $INSTALL_DIR"
        if ! mkdir -p "$INSTALL_DIR"; then
            error "Failed to create install directory: $INSTALL_DIR"
            echo ""
            echo "You may need to:"
            echo "  - Use sudo: sudo AIANDI_INSTALL_DIR=/usr/local/bin ./install.sh"
            echo "  - Choose different directory: AIANDI_INSTALL_DIR=~/bin ./install.sh"
            exit 1
        fi
    fi

    # Install binary
    info "Installing to $INSTALL_DIR/aiandi..."
    if ! cp "$temp_dir/aiandi" "$INSTALL_DIR/aiandi"; then
        error "Failed to copy binary to $INSTALL_DIR"
        exit 1
    fi

    if ! chmod +x "$INSTALL_DIR/aiandi"; then
        error "Failed to make binary executable"
        exit 1
    fi

    echo ""
    success "aiandi ${version#v} installed successfully!"

    # Check if install dir is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        warn "$INSTALL_DIR is not in your PATH."
        echo ""
        echo "Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
    fi

    echo ""
    echo "Run 'aiandi --help' to get started."
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -h|--help)
                show_help
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            --check)
                check_version
                exit 0
                ;;
            --uninstall)
                uninstall
                exit 0
                ;;
            --version)
                if [[ -z "${2:-}" ]]; then
                    error "--version requires a value"
                    exit 1
                fi
                VERSION="$2"
                shift 2
                ;;
            *)
                error "Unknown option: $1"
                echo ""
                echo "Run '$0 --help' for usage information."
                exit 1
                ;;
        esac
    done
}

# Main
main() {
    parse_args "$@"
    install
}

main "$@"
