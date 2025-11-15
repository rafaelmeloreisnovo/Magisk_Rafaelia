#!/usr/bin/env bash
#
# sccache.sh - Install sccache for GitHub Actions CI
#
# This script installs Mozilla's sccache (Shared Compilation Cache) which speeds up
# Rust compilation by caching compilation results. This is particularly useful in CI
# environments where clean builds are common.
#
# The script automatically detects the runner OS and installs the appropriate version:
# - macOS: Uses Homebrew for installation
# - Linux: Downloads prebuilt musl binary (static linking for better compatibility)
# - Windows: Downloads prebuilt Windows binary to Cargo's bin directory
#
# sccache works by intercepting compilation calls and caching the results, which can
# significantly reduce build times for incremental builds and repeated CI runs.

# Get latest sccache version from GitHub releases API
# Returns the version tag (e.g., "v0.5.4")
get_sccache_ver() {
  curl -sL 'https://api.github.com/repos/mozilla/sccache/releases/latest' | jq -r .name
}

# Install sccache from GitHub releases
#
# Downloads the latest sccache release for the specified platform variant,
# extracts the binary from the tarball, and installs it to the specified location.
#
# Arguments:
#   $1 - variant: Platform-specific identifier (e.g., "x86_64-unknown-linux-musl")
#   $2 - install_dir: Directory where the binary should be installed
#   $3 - exe: Name of the executable file (e.g., "sccache" or "sccache.exe")
install_from_gh() {
  local ver=$(curl -sL 'https://api.github.com/repos/mozilla/sccache/releases/latest' | jq -r .name)
  local url="https://github.com/mozilla/sccache/releases/download/${ver}/sccache-${ver}-$1.tar.gz"
  local dest="$2/$3"
  
  # Download tarball, extract executable using wildcard pattern, and write to destination
  curl -L "$url" | tar xz -O --wildcards "*/$3" > $dest
  chmod +x $dest
}

# Detect runner OS and install appropriate sccache version
# RUNNER_OS is set by GitHub Actions and can be: Linux, macOS, or Windows
if [ $RUNNER_OS = "macOS" ]; then
  # On macOS, use Homebrew for easy installation and updates
  brew install sccache
elif [ $RUNNER_OS = "Linux" ]; then
  # On Linux, use statically-linked musl binary for maximum compatibility
  install_from_gh x86_64-unknown-linux-musl /usr/local/bin sccache
elif [ $RUNNER_OS = "Windows" ]; then
  # On Windows, install to Cargo's bin directory (already in PATH)
  install_from_gh x86_64-pc-windows-msvc $USERPROFILE/.cargo/bin sccache.exe
fi
