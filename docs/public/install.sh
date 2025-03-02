#!/bin/bash
set -e

# Tombi installation script
# Automatically installs tombi from GitHub releases based on detected architecture

# Version
VERSION="0.2.16"
REPO="tombi-toml/tombi"
GITHUB_RELEASE_URL="https://github.com/${REPO}/releases/download"

# Color settings
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Helper functions
print_step() {
    echo "${BLUE}==>${NC} $1"
}

print_error() {
    echo "${RED}Error:${NC} $1"
}

print_success() {
    echo "${GREEN}Success:${NC} $1"
}

# Detect OS and architecture
detect_os_arch() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "${OS}" in
    Linux)
        OS="unknown-linux"
        if [[ "${ARCH}" == "aarch64" ]]; then
            ARCH="aarch64"
            TARGET="${ARCH}-${OS}-gnu"
        elif [[ "${ARCH}" == "armv7l" ]]; then
            ARCH="arm"
            TARGET="${ARCH}-${OS}-gnueabihf"
        else
            ARCH="x86_64"
            TARGET="${ARCH}-${OS}-gnu"
        fi
        ;;
    Darwin)
        OS="apple-darwin"
        if [[ "${ARCH}" == "arm64" ]]; then
            ARCH="aarch64"
        else
            ARCH="x86_64"
        fi
        TARGET="${ARCH}-${OS}"
        ;;
    MINGW* | MSYS* | CYGWIN* | Windows_NT)
        OS="pc-windows-msvc"
        if [[ "${ARCH}" == "aarch64" ]]; then
            ARCH="aarch64"
        else
            ARCH="x86_64"
        fi
        TARGET="${ARCH}-${OS}"
        ;;
    *)
        print_error "Unsupported OS: ${OS}"
        exit 1
        ;;
    esac

    print_step "Detected system: ${TARGET}"
}

# Create installation directories
create_install_dir() {
    INSTALL_DIR="${HOME}/.tombi"
    BIN_DIR="${HOME}/.local/bin"

    mkdir -p "${INSTALL_DIR}"
    mkdir -p "${BIN_DIR}"

    if [[ ! ":$PATH:" == *":${BIN_DIR}:"* ]]; then
        print_step "${BIN_DIR} is not in your PATH. Consider adding it to your shell configuration file."
    fi
}

# Download and install tombi
download_and_install() {
    DOWNLOAD_URL="${GITHUB_RELEASE_URL}/v${VERSION}/tombi-vscode-${VERSION}-${TARGET}"
    TEMP_FILE="${INSTALL_DIR}/tombi-${VERSION}"

    print_step "Download from ${DOWNLOAD_URL}"
    print_step "Downloading tombi v${VERSION} (${TARGET})..."

    if command -v curl >/dev/null 2>&1; then
        if ! curl -L -f -s "${DOWNLOAD_URL}" -o "${TEMP_FILE}"; then
            print_error "Download failed. Please check the URL: ${DOWNLOAD_URL}"
            exit 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget --tries=1 -q "${DOWNLOAD_URL}" -O "${TEMP_FILE}"; then
            print_error "Download failed. Please check the URL: ${DOWNLOAD_URL}"
            exit 1
        fi
    else
        print_error "Neither curl nor wget is installed. Please install one of them."
        exit 1
    fi

    if [ ! -f "${TEMP_FILE}" ] || [ ! -s "${TEMP_FILE}" ]; then
        print_error "Download failed. Please check the URL: ${DOWNLOAD_URL}"
        exit 1
    fi

    chmod +x "${TEMP_FILE}"
    mv "${TEMP_FILE}" "${BIN_DIR}/tombi"

    print_success "tombi v${VERSION} has been installed to ${BIN_DIR}/tombi"
}

# Main process
main() {
    print_step "Starting tombi installer..."
    detect_os_arch
    create_install_dir
    if ! download_and_install; then
        exit 1
    fi

    # Verify installation
    if command -v tombi >/dev/null 2>&1; then
        if tombi --version >/dev/null 2>&1; then
            INSTALLED_VERSION=$(tombi --version 2>&1 | head -n 1 || echo "unknown")
            print_success "tombi ${INSTALLED_VERSION} has been successfully installed!"
            echo "Usage: ${GREEN}tombi --help${NC}"
        else
            print_error "Installation completed, but tombi command cannot be executed. "
            echo "To run manually: ${GREEN}${BIN_DIR}/tombi --help${NC}"
            exit 1
        fi
    else
        print_error "Installation completed, but tombi command not found. Please check your PATH settings."
        echo "To run manually: ${GREEN}${BIN_DIR}/tombi --help${NC}"
        exit 1
    fi
}

# Execute the script
main
