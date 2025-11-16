#!/usr/bin/env bash
#
# bug_snapshot.sh - Android Device Debugging Snapshot Collector
#
# Collects comprehensive debugging artifacts from an Android emulator or physical device
# to assist with CI triage and bug investigation. This script gathers logs, crash dumps,
# ANR traces, and other diagnostic information into a single ZIP archive.
#
# Usage:
#   .github/scripts/bug_snapshot.sh <output-zip> [optional manifest path]
#
# Arguments:
#   $1 - output-zip: Name of the output ZIP file (default: bug_snapshot.zip)
#   $2 - manifest: Path to RAFAELIA manifest JSON (default: RAFAELIA_MANIFEST.json)
#
# The script collects:
#   - logcat: System logs from the Android device
#   - dumpsys: Activity manager state
#   - getprop: System properties
#   - tombstones: Native crash dumps
#   - ANR traces: Application Not Responding logs
#   - Instrumentation output: Test results if available
#   - RAFAELIA manifest: Build metadata
#
# Requirements:
#   - adb (Android Debug Bridge) must be in PATH
#   - Device must be connected and authorized for debugging
#   - jq is recommended but not required

set -euo pipefail

# Parse command-line arguments with defaults
OUT_ZIP=${1:-bug_snapshot.zip}
MANIFEST=${2:-RAFAELIA_MANIFEST.json}
ARTDIR=${GITHUB_WORKSPACE:-$(pwd)}/.rafaelia_artifacts

# Create artifacts directory if it doesn't exist
mkdir -p "$ARTDIR"

echo "Collecting logcat..."
# Wait for device to be ready before collecting logs
adb wait-for-device
# Dump complete logcat buffer (all log messages since device boot)
adb logcat -d > "$ARTDIR"/emulator-logcat.txt 2>&1 || true

echo "Collecting dumpsys (activities)..."
# Get detailed activity stack and state information
# Useful for understanding app lifecycle issues
adb shell dumpsys activity activities > "$ARTDIR"/dumpsys-activities.txt 2>&1 || true

echo "Collecting getprop..."
# Gather all system properties (Android version, build info, hardware details)
adb shell getprop > "$ARTDIR"/emulator-getprop.txt 2>&1 || true

echo "Collecting tombstones (native crashes)..."
# Check if tombstone directory exists and contains crash dumps
# Tombstones are created when native code (C/C++) crashes
if adb shell ls /data/tombstones 2>/dev/null | grep -q .; then
  mkdir -p "$ARTDIR"/tombstones
  adb pull /data/tombstones "$ARTDIR"/tombstones || true
fi

echo "Collecting ANR traces..."
# Application Not Responding traces show where apps are stuck/hanging
# These are crucial for diagnosing performance and deadlock issues
if adb shell ls /data/anr 2>/dev/null | grep -q .; then
  mkdir -p "$ARTDIR"/anr
  adb pull /data/anr "$ARTDIR"/anr || true
fi

echo "Pulling instrumentation output (if any)..."
# Include test instrumentation results if they were generated
if [ -f "$GITHUB_WORKSPACE"/artifacts/instrumentation-output.txt ]; then
  cp "$GITHUB_WORKSPACE"/artifacts/instrumentation-output.txt "$ARTDIR"/ || true
fi

# Include RAFAELIA manifest for build traceability
# This provides SHA hashes, build metadata, and signatures
if [ -f "$MANIFEST" ]; then
  cp "$MANIFEST" "$ARTDIR"/manifest.json || true
fi

echo "Packing snapshot to $OUT_ZIP"
# Create compressed archive of all collected artifacts
( cd "$ARTDIR" && zip -r -q "$GITHUB_WORKSPACE/$OUT_ZIP" . )
echo "Snapshot created at $GITHUB_WORKSPACE/$OUT_ZIP"
exit 0
