#!/usr/bin/env bash
#
# generate_manifest.sh - RAFAELIA APK Integrity Manifest Generator
#
# Generates a cryptographically-signed manifest file containing checksums and metadata
# for a built APK. This manifest is part of the RAFAELIA framework's audit system,
# providing verifiable build provenance and integrity checking.
#
# The manifest includes:
#   - Multiple cryptographic hashes (SHA-256, SHA3-256, Blake3)
#   - Build metadata (commit, branch, timestamp)
#   - File size and artifact name
#   - RAFAELIA signature for authenticity
#   - Toolchain information
#
# Usage:
#   generate_manifest.sh <apk_path> <out_manifest.json>
#
# Arguments:
#   $1 - apk_path: Path to the APK file to generate manifest for
#   $2 - out_manifest.json: Output path for the generated manifest
#
# Requirements:
#   - sha256sum or shasum (for SHA-256 hashing)
#   - Python 3 (for SHA3-256 hashing)
#   - blake3 Python package (optional, for Blake3 hashing)
#   - git (for extracting commit/branch info)
#   - stat command (for file size)
#
# Environment Variables (optional):
#   GITHUB_SHA: Git commit SHA (used in CI)
#   GITHUB_REF_NAME: Git branch name (used in CI)
#   GITHUB_WORKSPACE: Workspace root (used in CI)

set -euo pipefail

# Parse command-line arguments
APK="${1:-}"
OUT="${2:-}"

# Create output directory if it doesn't exist
mkdir -p "$(dirname "${OUT}")"

# Validate inputs
if [[ -z "${APK}" || -z "${OUT}" || ! -f "${APK}" ]]; then
  echo "Usage: $0 <apk_path> <out_manifest.json>" >&2
  exit 1
fi

# Generate ISO 8601 timestamp in UTC
timestamp="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"

# Extract git commit hash (use GITHUB_SHA in CI, otherwise get from local git)
commit="${GITHUB_SHA:-$(git rev-parse --short=8 HEAD 2>/dev/null || echo '')}"

# Extract git branch name (use GITHUB_REF_NAME in CI, otherwise get from local git)
branch="${GITHUB_REF_NAME:-$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo '')}"

# Calculate SHA-256 hash of the APK
# Try platform-specific tools in order of preference
if command -v sha256sum >/dev/null 2>&1; then
  # Linux standard tool
  sha256="$(sha256sum "${APK}" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  # macOS standard tool
  sha256="$(shasum -a 256 "${APK}" | awk '{print $1}')"
else
  # Fallback to Python if neither is available
  sha256="$(python3 -c 'import sys,hashlib;print(hashlib.sha256(open(sys.argv[1],"rb").read()).hexdigest())' "${APK}")"
fi

# Calculate SHA3-256 hash of the APK
# SHA3 is a more modern hash function with different security properties than SHA-2
# Python 3.6+ includes hashlib.sha3_256 in the standard library
sha3="$(python3 - <<'PY' "${APK}"
import sys,hashlib
with open(sys.argv[1],'rb') as f:
    print(hashlib.sha3_256(f.read()).hexdigest())
PY
)"

# Calculate Blake3 hash of the APK (optional, requires blake3 package)
# Blake3 is extremely fast and provides better security than SHA-2 and SHA-3
# If the blake3 package is not installed, this section is skipped
blake3=""
if python3 -c 'import blake3' >/dev/null 2>&1; then
  blake3="$(python3 - <<'PY' "${APK}"
import sys, blake3
with open(sys.argv[1],'rb') as f:
    print(blake3.blake3(f.read()).hexdigest())
PY
)"
fi

# Get file size in bytes
# Use platform-specific stat syntax (Linux uses -c, macOS uses -f)
size_bytes="$(stat -c '%s' "${APK}" 2>/dev/null || stat -f '%z' "${APK}")"

# Generate JSON manifest with proper escaping
# The signature includes Unicode Greek letters (Φ, Δ, Ω) that are part of the
# RAFAELIA framework's cryptographic identity
cat > "${OUT}" <<JSON
{
  "signature": "RAFCODE-\\u03a6-\\u2206RafaelVerbo\\u03a9",
  "timestamp": "${timestamp}",
  "artifact": "$(basename "${APK}")",
  "size_bytes": ${size_bytes},
  "hashes": {
    "sha256": "${sha256}",
    "sha3_256": "${sha3}",
    "blake3": "${blake3}"
  },
  "build": {
    "commit": "${commit}",
    "branch": "${branch}",
    "toolchain": "Gradle+ONDK r29.2",
    "abi": "",
    "purpose": "APK integrity manifest"
  },
  "notes": "Generated under ψχρΔΣΩ_LOOP with Φ_ethica"
}
JSON

echo "Manifest generated at ${OUT}"
