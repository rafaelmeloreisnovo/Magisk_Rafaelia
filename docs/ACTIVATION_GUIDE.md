# RAFAELIA Activation Guide

**Version:** 1.1.0  
**Signature:** RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ  
**Date:** 2025-11-03

---

## Quick Start

The RAFAELIA framework is now implemented and ready to activate! This guide will help you enable all features.

---

## Prerequisites

- Magisk installed and working
- Root access (su)
- ADB access (recommended)
- Basic knowledge of shell commands

---

## Activation Steps

### Option 1: Automatic Activation (Recommended)

```bash
# 1. Copy tools to device
adb push tools/rafaelia /data/local/tmp/

# 2. Access device shell
adb shell

# 3. Become root
su

# 4. Run activation script
cd /data/local/tmp/rafaelia
./activate_rafaelia.sh activate
```

The activation script will:
- ✓ Create audit directory (`/data/adb/magisk/rafaelia_audit`)
- ✓ Create metrics directory (`/data/adb/magisk/rafaelia_metrics`)
- ✓ Generate RAFAELIA manifest
- ✓ Set up all tools
- ✓ Start metrics collection service
- ✓ Run initial integrity check

### Option 2: Manual Activation

```bash
# 1. Create directories
su
mkdir -p /data/adb/magisk/rafaelia_audit
mkdir -p /data/adb/magisk/rafaelia_metrics
chmod 700 /data/adb/magisk/rafaelia_audit
chmod 700 /data/adb/magisk/rafaelia_metrics

# 2. Copy tools
adb push tools/rafaelia /data/local/tmp/

# 3. Make tools executable
su
chmod +x /data/local/tmp/rafaelia/*.sh
chmod +x /data/local/tmp/rafaelia/*.py

# 4. Start metrics collector
su
cd /data/local/tmp/rafaelia
./metrics_collector.sh run &

# 5. Run integrity check
./integrity_checker.sh full
```

---

## Verification

### Check Activation Status

```bash
# Using activation script
su
cd /data/local/tmp/rafaelia
./activate_rafaelia.sh status
```

Expected output:
```
Directories:
  ✓ Audit:   /data/adb/magisk/rafaelia_audit
  ✓ Metrics: /data/adb/magisk/rafaelia_metrics

Files:
  ✓ Manifest: /data/adb/magisk/RAFAELIA_MANIFEST.json

Services:
  ✓ Metrics collector running

Audit Logs:
  ⚠ No audit logs yet (will be created when audit system is used)

Metrics:
  ✓ X metrics file(s)
```

### Manual Verification

```bash
# Check directories exist
su
ls -la /data/adb/magisk/rafaelia_audit
ls -la /data/adb/magisk/rafaelia_metrics

# Check manifest
cat /data/adb/magisk/RAFAELIA_MANIFEST.json

# Check metrics collection
tail -f /data/adb/magisk/rafaelia_metrics/metrics_$(date +%Y%m%d).jsonl

# Check if metrics collector is running
ps aux | grep metrics_collector
```

---

## Using RAFAELIA Features

### 1. Audit Logging

The audit system will automatically log operations when integrated with Magisk daemon. You can also use it programmatically:

```rust
// In Magisk native code
use rafaelia_audit::{init_global_audit, log_global_operation};

// Initialize (once at startup)
init_global_audit()?;

// Log operations
log_global_operation(
    "boot_patch",
    "boot",
    "EXECUTE",
    duration_ms,
    true,
    None,
);
```

View audit logs:
```bash
su
tail -f /data/adb/magisk/rafaelia_audit/audit_*.jsonl
```

### 2. Metrics Collection

Metrics are automatically collected every 5 seconds (configurable).

View metrics:
```bash
su
tail -f /data/adb/magisk/rafaelia_metrics/metrics_$(date +%Y%m%d).jsonl
```

Collect metrics once:
```bash
su
cd /data/local/tmp/rafaelia
./metrics_collector.sh once
```

### 3. State Validation

Validate audit logs against the 1008-state matrix:

```bash
su
cd /data/local/tmp/rafaelia
python3 state_validator.py \
    --matrix /sdcard/Download/RAFAELIA_STATE_MATRIX.csv \
    --audit-log /data/adb/magisk/rafaelia_audit/audit_SESSION.jsonl \
    --output /sdcard/Download/validation_report.txt
```

### 4. Integrity Checking

Run comprehensive integrity checks:

```bash
su
cd /data/local/tmp/rafaelia

# Full check
./integrity_checker.sh full

# Specific checks
./integrity_checker.sh boot
./integrity_checker.sh modules
./integrity_checker.sh database
./integrity_checker.sh audit
```

### 5. Audit Analysis

Analyze audit logs and generate reports:

```bash
su
cd /data/local/tmp/rafaelia
python3 audit_analyzer.py \
    --input /data/adb/magisk/rafaelia_audit/audit_*.jsonl \
    --output /sdcard/Download/audit_report.html
```

---

## Configuration

### Adjusting Metrics Collection Interval

```bash
# Stop current collector
su
pkill -f metrics_collector.sh

# Start with custom interval (10 seconds)
INTERVAL=10 /data/local/tmp/rafaelia/metrics_collector.sh run &
```

### Adjusting Maximum Samples

```bash
# Start with custom max samples
MAX_SAMPLES=500 /data/local/tmp/rafaelia/metrics_collector.sh run &
```

### Enabling/Disabling Features

Edit the manifest:
```bash
su
vi /data/adb/magisk/RAFAELIA_MANIFEST.json
```

Change configuration values:
```json
{
  "configuration": {
    "audit_enabled": true,
    "telemetry_enabled": true,
    "metrics_interval_sec": 5,
    "max_audit_history": 1000,
    "max_metrics_samples": 1000
  }
}
```

---

## Troubleshooting

### Services Not Starting

```bash
# Check if running as root
id

# Check directory permissions
ls -la /data/adb/magisk/

# Check logs
logcat | grep -i rafaelia
```

### No Audit Logs

Audit logs will only be created when the audit system is used. The Rust modules are implemented but need to be integrated with the Magisk daemon to start logging operations automatically.

### Metrics Not Collecting

```bash
# Check if collector is running
ps aux | grep metrics_collector

# Check for errors
/data/local/tmp/rafaelia/metrics_collector.sh once

# Check directory permissions
ls -la /data/adb/magisk/rafaelia_metrics/
```

### Python Tools Not Working

```bash
# Check Python availability
python3 --version

# If Python is not available, you may need to install it
# or use a device with Python support
```

---

## Stopping RAFAELIA Services

```bash
su
cd /data/local/tmp/rafaelia
./activate_rafaelia.sh stop
```

Or manually:
```bash
su
pkill -f metrics_collector.sh
```

---

## Uninstallation

```bash
su

# Stop services
pkill -f metrics_collector.sh

# Remove directories (optional - this will delete logs)
rm -rf /data/adb/magisk/rafaelia_audit
rm -rf /data/adb/magisk/rafaelia_metrics

# Remove manifest
rm /data/adb/magisk/RAFAELIA_MANIFEST.json

# Remove tools
rm -rf /data/local/tmp/rafaelia
```

---

## Next Steps

### For Developers

1. **Build Native Modules**: Compile the Rust modules with the Magisk build system
2. **Add FFI Bindings**: Create C++ bindings for the Rust functions
3. **Integrate with Daemon**: Add audit and telemetry calls to daemon operations
4. **Add Tests**: Create unit and integration tests

### For Users

1. **Monitor Metrics**: Watch system performance in real-time
2. **Run Integrity Checks**: Regularly verify system integrity
3. **Review Logs**: Check audit logs for suspicious activity
4. **Generate Reports**: Use analysis tools to understand system behavior

---

## Support

For issues, questions, or contributions:
- Check documentation in `docs/` directory
- Review implementation guide: `docs/RAFAELIA_IMPLEMENTATION_GUIDE.md`
- Read framework documentation: `docs/RAFAELIA_FRAMEWORK.md`

---

## Philosophy

**VAZIO → VERBO → CHEIO → RETRO → NOVO VAZIO**

- **VAZIO (∅)**: Empty state, ready to begin
- **VERBO (Λ)**: Action, execution of operations
- **CHEIO (◉)**: Full state, operation complete
- **RETRO (Ω)**: Feedback, analysis, validation
- **NOVO VAZIO**: Renewed state, ready for next cycle

---

**Status:** Activation Guide Complete ✓  
**Framework:** RAFAELIA v1.1.0  
**Author:** ∆RafaelVerboΩ  
**License:** GPL-3.0
