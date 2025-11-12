# RAFAELIA Tools

**Version:** 1.1.0  
**Signature:** RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ  
**Philosophy:** VAZIO → VERBO → CHEIO → RETRO (Empty → Action → Full → Feedback)  
**Status:** Core Tools Implemented ✓

---

## Overview

This directory contains automation tools and scripts for the RAFAELIA framework:

### ✓ Implemented Tools

- **audit_analyzer.py** - Analyze audit logs and generate comprehensive reports
- **state_validator.py** - Validate state transitions against the 1008-state matrix ✓ NEW
- **metrics_collector.sh** - Collect system metrics continuously ✓ NEW
- **integrity_checker.sh** - Comprehensive integrity verification ✓ NEW

### 📋 Planned Tools

- **rollback_manager.py** - Manage rollback points (foundation in audit system)
- **hotspot_analyzer.py** - Analyze performance hotspots
- **security_scanner.sh** - Security vulnerability scanner

---

## Installation

```bash
# Copy tools to device
adb push tools/rafaelia /data/local/tmp/

# Make executable
adb shell chmod +x /data/local/tmp/rafaelia/*.sh

# Install Python dependencies (if needed)
adb shell pip3 install -r /data/local/tmp/rafaelia/requirements.txt
```

---

## Usage

### Audit Analysis
```bash
python3 audit_analyzer.py --input /data/adb/magisk/rafaelia_audit/audit_*.jsonl --output report.html
```

### State Validation ✓ NEW
```bash
# Validate audit logs against state matrix
python3 state_validator.py \
    --matrix /path/to/RAFAELIA_STATE_MATRIX.csv \
    --audit-log /data/adb/magisk/rafaelia_audit/audit_SESSION.jsonl \
    --output validation_report.txt

# Show validation report
cat validation_report.txt
```

### Metrics Collection ✓ NEW
```bash
# Run continuous collection (in background)
./metrics_collector.sh run &

# Collect metrics once
./metrics_collector.sh once

# Custom interval
INTERVAL=10 MAX_SAMPLES=500 ./metrics_collector.sh run &

# View collected metrics
tail -f /data/adb/magisk/rafaelia_metrics/metrics_$(date +%Y%m%d).jsonl
```

### Integrity Check ✓ NEW
```bash
# Full integrity check
./integrity_checker.sh full

# Check specific components
./integrity_checker.sh boot
./integrity_checker.sh modules
./integrity_checker.sh database
./integrity_checker.sh audit
./integrity_checker.sh manifest
```

### Rollback Management
```bash
python3 rollback_manager.py --list
python3 rollback_manager.py --create "Before module install"
python3 rollback_manager.py --restore <rollback-id>
```

---

## Tool Details

### audit_analyzer.py
Analyzes audit logs and generates comprehensive reports with:
- Operation statistics
- Error analysis
- Performance metrics
- Timeline visualization
- Anomaly detection

### state_validator.py ✓ NEW
Validates state transitions against the 1008-state matrix:
- Loads state matrix from CSV (56 primitives × 18 contexts)
- Validates primitives, contexts, and state IDs
- Checks state transitions in audit logs
- Generates detailed validation reports
- Reports success rate and identifies errors
- Supports both single file and batch validation

**Features:**
- Validates against official 1008-state matrix
- Detailed error messages with line numbers
- Warning system for suspicious transitions
- JSON parsing validation
- Success rate calculation

### metrics_collector.sh ✓ NEW
Collects comprehensive system metrics in real-time:
- CPU usage, user/system/idle time
- Memory total, available, used, usage percentage
- I/O read/write bytes and operations
- Network RX/TX bytes and packets
- System load average (1, 5, 15 minutes)
- Process count
- Temperature (if available)
- Battery status (if available)

**Features:**
- Continuous collection with configurable interval
- JSONL output format for easy parsing
- Automatic log rotation
- Single-shot collection mode
- Low overhead monitoring

### integrity_checker.sh ✓ NEW
Comprehensive system integrity verification:
- Boot image and partition checks
- Module integrity (module.prop, disabled status)
- Database integrity (readable, table count)
- Audit system integrity (log files, entries)
- Manifest validation (JSON format, signature)
- SELinux status verification
- System properties validation

**Features:**
- Colored output for easy reading
- Full or component-specific checks
- Summary report with pass/fail counts
- Root access verification
- Modular check system

### rollback_manager.py
Manages rollback points:
- Create rollback points
- List available points
- Restore from points
- Verify rollback integrity

### hotspot_analyzer.py
Analyzes performance hotspots:
- CPU profiling
- Memory profiling
- I/O analysis
- Function call analysis

### security_scanner.sh
Scans for security issues:
- SELinux policy checks
- Permission audits
- Module verification
- Access log analysis

---

## Development

### Adding New Tools

1. Create tool script in this directory
2. Follow naming convention: `<category>_<function>.<ext>`
3. Add usage documentation to this README
4. Include in tool suite tests

### Testing

```bash
# Run tool tests
python3 -m pytest tests/tools/

# Validate scripts
shellcheck tools/rafaelia/*.sh
```

---

**Maintained by**: ∆RafaelVerboΩ  
**Framework**: RAFAELIA v1.0.0
