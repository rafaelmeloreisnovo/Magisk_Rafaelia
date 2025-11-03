# RAFAELIA Tools

**Version:** 1.0.0  
**Signature:** RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ

---

## Overview

This directory contains automation tools and scripts for the RAFAELIA framework:

- **audit_analyzer.py** - Analyze audit logs and generate reports
- **state_validator.py** - Validate state transitions in the fractal matrix
- **metrics_collector.sh** - Collect and export telemetry metrics
- **integrity_checker.sh** - Verify system integrity
- **rollback_manager.py** - Manage rollback points
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

### State Validation
```bash
python3 state_validator.py --matrix RAFAELIA_STATE_MATRIX.csv --audit /data/adb/magisk/rafaelia_audit/
```

### Metrics Collection
```bash
./metrics_collector.sh --interval 60 --output /data/adb/magisk/rafaelia_audit/metrics.json
```

### Integrity Check
```bash
./integrity_checker.sh --full
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

### state_validator.py
Validates state transitions against the 1008-state matrix:
- Checks for invalid transitions
- Verifies integrity requirements
- Validates audit compliance
- Generates state flow diagrams

### metrics_collector.sh
Collects system metrics:
- CPU usage
- Memory usage
- I/O statistics
- Network activity
- Process information

### integrity_checker.sh
Verifies system integrity:
- Boot image verification
- Module verification
- Database verification
- File system checks

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
