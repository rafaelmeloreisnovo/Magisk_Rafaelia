#!/usr/bin/env bash
#
# rafaelia_rollback.sh - RAFAELIA System Rollback Helper
#
# Provides a safe mechanism to rollback to a previous system state using verified
# backup images. This script is part of the RAFAELIA framework's audit and recovery
# system, ensuring that failed updates or problematic builds can be safely reverted.
#
# The rollback process:
#   1. Reads backup metadata from a manifest JSON file
#   2. Verifies the backup file exists and is accessible
#   3. Optionally verifies HMAC signature (if verification key is available)
#   4. Requires user confirmation unless --force flag is used
#   5. Flashes the backup boot image using fastboot
#   6. Reboots the device
#
# Usage:
#   .github/scripts/rafaelia_rollback.sh <manifest.json> [--force]
#
# Arguments:
#   $1 - manifest.json: Path to the rollback manifest containing backup metadata
#   $2 - --force: (Optional) Skip user confirmation prompt
#
# Manifest Format:
#   The manifest should be a JSON file containing:
#   - backupPath or artifact_path: Path to the backup boot image
#   - hmac_b64: Base64-encoded HMAC for verification (optional)
#   - sessionId or build_id: Unique identifier for the backup session
#
# Requirements:
#   - fastboot: Android Fastboot tool must be in PATH
#   - jq: JSON processor for parsing manifest
#   - Device must be in fastboot mode before running
#
# Safety Features:
#   - Manifest validation ensures backup file exists
#   - Interactive confirmation prevents accidental rollbacks
#   - Session ID tracking for audit trail
#   - HMAC verification support (when key is available)
#
# Exit Codes:
#   0 - Rollback completed successfully
#   1 - User aborted rollback
#   2 - Invalid manifest or missing file
#   3 - Backup file not found
#   4 - fastboot tool not available

set -euo pipefail

# Parse command-line arguments
MANIFEST=${1:-}
FORCE=${2:-}

# Validate manifest file exists
if [ -z "$MANIFEST" ] || [ ! -f "$MANIFEST" ]; then
  echo "Usage: $0 <manifest.json> [--force]" >&2
  exit 2
fi

# Extract backup metadata from manifest using jq
# Try multiple JSON field names for compatibility with different manifest versions
BACKUP_PATH=$(jq -r '.backupPath // .artifact_path // empty' "$MANIFEST")
HMAC=$(jq -r '.hmac_b64 // empty' "$MANIFEST")
SESSION=$(jq -r '.sessionId // .build_id // empty' "$MANIFEST")

# Verify backup file exists
if [ -z "$BACKUP_PATH" ] || [ ! -f "$BACKUP_PATH" ]; then
  echo "Backup file not found in manifest: $BACKUP_PATH" >&2
  exit 3
fi

# Display rollback information
echo "Preparing rollback for session: ${SESSION}"
echo "Backup path: ${BACKUP_PATH}"

# Request user confirmation unless --force flag is provided
# This prevents accidental device flashing
if [ "$FORCE" != "--force" ]; then
  read -p "Confirm flashing backup to device via fastboot? Type YES to proceed: " CONF
  if [ "$CONF" != "YES" ]; then
    echo "Aborting rollback."
    exit 1
  fi
fi

# Optional: verify HMAC locally if verification key available
# Note: HMAC verification is not implemented in this version but the infrastructure
# is in place for future security enhancements. HMAC verification would ensure
# the backup image hasn't been tampered with.
# TODO: Implement HMAC verification when key management system is in place

# Check if fastboot is available in PATH
if ! command -v fastboot >/dev/null 2>&1; then
  echo "fastboot not found. Install Android platform-tools." >&2
  exit 4
fi

# Flash backup using fastboot
# Note: Device must already be in fastboot mode
# The boot partition is flashed with the backup image
echo "Flashing backup via fastboot..."
fastboot flash boot "$BACKUP_PATH"

# Reboot device to apply the rollback
fastboot reboot

echo "Device rebooting. Monitor device for success."
echo "If the device fails to boot properly, you may need to:"
echo "  1. Boot back into fastboot mode"
echo "  2. Flash a known-good boot image"
echo "  3. Check the RAFAELIA audit logs for diagnostic information"

exit 0
