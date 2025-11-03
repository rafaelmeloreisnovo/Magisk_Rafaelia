# RAFAELIA Operational Checklist

**Version:** 1.0.0  
**Signature:** RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ  
**Author:** ∆RafaelVerboΩ

---

## Daily Operations

### Morning Check (Every Day)

- [ ] **1. Verify Daemon Status**
  ```bash
  magisk --status
  # Expected: "Magisk is running"
  ```

- [ ] **2. Review Overnight Audit Logs**
  ```bash
  magisk-audit review --since yesterday
  # Check for errors, warnings, critical events
  ```

- [ ] **3. Check System Integrity**
  ```bash
  magisk --verify-boot
  magisk --verify-modules
  magisk --verify-db
  # All should return "OK"
  ```

- [ ] **4. Monitor Resource Usage**
  ```bash
  magisk-metrics snapshot
  # Verify CPU < 80%, Memory < 512MB, Disk space > 1GB
  ```

- [ ] **5. Review Active Modules**
  ```bash
  magisk --list-modules
  # Confirm expected modules are active
  ```

- [ ] **6. Check for Updates**
  ```bash
  magisk --check-updates
  # Note any available updates
  ```

### Weekly Maintenance (Every Sunday)

- [ ] **1. Backup Critical Data**
  ```bash
  magisk-backup create --full
  # Backup boot image, database, modules
  ```

- [ ] **2. Analyze Performance Trends**
  ```bash
  magisk-metrics history --days 7
  # Review performance over the week
  ```

- [ ] **3. Clean Old Audit Logs**
  ```bash
  magisk-audit cleanup --older-than 30d
  # Remove logs older than 30 days
  ```

- [ ] **4. Review Rollback Points**
  ```bash
  magisk-rollback list
  # Keep only 10 most recent, delete old ones
  ```

- [ ] **5. Update Documentation**
  ```bash
  # Document any configuration changes
  # Update module list if changed
  ```

- [ ] **6. Security Audit**
  ```bash
  magisk-audit security-review
  # Check for unauthorized access attempts
  ```

### Monthly Review (First of Month)

- [ ] **1. Comprehensive System Audit**
  ```bash
  magisk-audit full-report --month last
  # Generate and review full audit report
  ```

- [ ] **2. Performance Optimization**
  ```bash
  magisk-metrics analyze-hotspots
  # Identify and address performance bottlenecks
  ```

- [ ] **3. Module Health Check**
  ```bash
  for module in $(magisk --list-modules); do
    magisk --verify-module $module
  done
  ```

- [ ] **4. Update Strategy Review**
  ```bash
  # Review Magisk and module update schedule
  # Plan updates for next month
  ```

- [ ] **5. Backup Verification**
  ```bash
  magisk-backup verify --all
  # Ensure all backups are valid
  ```

- [ ] **6. Documentation Update**
  ```bash
  # Update RAFAELIA framework documentation
  # Document any issues or improvements
  ```

---

## Pre-Operation Checklists

### Before Installing Module

- [ ] **1. Create Rollback Point**
  ```bash
  magisk-rollback create --description "Before installing module X"
  ```

- [ ] **2. Verify Module Integrity**
  ```bash
  magisk --verify-module-package /path/to/module.zip
  # Check signature and hash
  ```

- [ ] **3. Check Compatibility**
  ```bash
  magisk --check-module-compatibility /path/to/module.zip
  # Verify module is compatible with current Magisk version
  ```

- [ ] **4. Review Module Permissions**
  ```bash
  magisk --show-module-permissions /path/to/module.zip
  # Review what the module will modify
  ```

- [ ] **5. Backup Current State**
  ```bash
  magisk-backup create --modules
  # Backup current module configuration
  ```

- [ ] **6. Check Free Space**
  ```bash
  df -h /data
  # Ensure sufficient free space (>1GB recommended)
  ```

### Before Patching Boot Image

- [ ] **1. Backup Boot Image**
  ```bash
  magisk-backup create --boot
  # Critical: backup original boot image
  ```

- [ ] **2. Verify Boot Image**
  ```bash
  magisk --verify-boot-image /path/to/boot.img
  # Verify boot image integrity
  ```

- [ ] **3. Check AVB/vbmeta Status**
  ```bash
  magisk --check-avb
  # Verify AVB status and compatibility
  ```

- [ ] **4. Create Rollback Point**
  ```bash
  magisk-rollback create --description "Before boot patch"
  ```

- [ ] **5. Document Current State**
  ```bash
  magisk --status > /data/local/tmp/pre_patch_status.txt
  magisk --list-modules >> /data/local/tmp/pre_patch_status.txt
  ```

- [ ] **6. Ensure Battery Charged**
  ```bash
  # Verify battery > 50% to prevent interruption
  ```

### Before System Update (OTA)

- [ ] **1. Full Backup**
  ```bash
  magisk-backup create --full --ota
  # Complete system backup
  ```

- [ ] **2. Disable All Modules**
  ```bash
  magisk --disable-all-modules
  # Disable modules before OTA
  ```

- [ ] **3. Export Configuration**
  ```bash
  magisk --export-config > /sdcard/magisk_config_backup.json
  # Export all settings
  ```

- [ ] **4. Document Module List**
  ```bash
  magisk --list-modules > /sdcard/magisk_modules_list.txt
  # Save module list for reinstall
  ```

- [ ] **5. Verify Magisk Uninstall**
  ```bash
  magisk --uninstall
  # Uninstall Magisk before OTA (if required)
  ```

- [ ] **6. Create Recovery Point**
  ```bash
  # Ensure you have backup boot image for recovery
  ```

---

## Post-Operation Checklists

### After Installing Module

- [ ] **1. Verify Module Installation**
  ```bash
  magisk --verify-module <module-id>
  # Confirm module installed correctly
  ```

- [ ] **2. Test Module Functionality**
  ```bash
  # Test that module works as expected
  # Check for any system issues
  ```

- [ ] **3. Monitor Performance**
  ```bash
  magisk-metrics snapshot
  # Check for performance impact
  ```

- [ ] **4. Review Audit Log**
  ```bash
  magisk-audit view --last 10
  # Check for any errors during installation
  ```

- [ ] **5. Verify System Stability**
  ```bash
  # Use device normally for 5-10 minutes
  # Watch for crashes or issues
  ```

- [ ] **6. Document Changes**
  ```bash
  # Update module documentation
  # Note any configuration changes
  ```

### After Patching Boot Image

- [ ] **1. Verify Boot Success**
  ```bash
  magisk --status
  # Confirm Magisk loaded correctly
  ```

- [ ] **2. Check SELinux Status**
  ```bash
  getenforce
  # Should be "Enforcing" or "Permissive" as configured
  ```

- [ ] **3. Verify Modules**
  ```bash
  magisk --list-modules
  # Confirm all modules loaded
  ```

- [ ] **4. Test Root Access**
  ```bash
  su -c "id"
  # Verify root works
  ```

- [ ] **5. Run Integrity Check**
  ```bash
  magisk --verify-boot
  # Verify boot image integrity
  ```

- [ ] **6. Create Rollback Point**
  ```bash
  magisk-rollback create --description "After successful boot patch"
  ```

### After System Update (OTA)

- [ ] **1. Reinstall Magisk**
  ```bash
  # Patch new boot image with Magisk
  magisk --patch-boot /path/to/new_boot.img
  ```

- [ ] **2. Restore Configuration**
  ```bash
  magisk --import-config /sdcard/magisk_config_backup.json
  # Restore settings
  ```

- [ ] **3. Reinstall Modules**
  ```bash
  # Reinstall modules from saved list
  # Test each module individually
  ```

- [ ] **4. Verify System Integrity**
  ```bash
  magisk --verify-boot
  magisk --verify-modules
  ```

- [ ] **5. Test All Functionality**
  ```bash
  # Test root access
  # Test module functionality
  # Test app compatibility
  ```

- [ ] **6. Create New Rollback Point**
  ```bash
  magisk-rollback create --description "After OTA and Magisk reinstall"
  ```

---

## Emergency Procedures

### System Won't Boot

1. **Enter Recovery Mode**
   - Hold Volume Down + Power during boot

2. **Check for Magisk Uninstaller**
   ```bash
   # Flash Magisk uninstaller ZIP if available
   ```

3. **Restore Boot Image**
   ```bash
   # Flash original boot image backup
   fastboot flash boot boot_backup.img
   ```

4. **Clear Cache/Dalvik**
   ```bash
   # In recovery, wipe cache and dalvik cache
   ```

5. **Reboot and Verify**
   ```bash
   # Reboot system
   # If boots, reinstall Magisk carefully
   ```

### Module Causing Issues

1. **Boot to Safe Mode**
   ```bash
   # Hold Volume Down during boot to disable all modules
   ```

2. **Identify Problem Module**
   ```bash
   magisk-audit view --since last-boot
   # Check which module caused issues
   ```

3. **Remove Problem Module**
   ```bash
   magisk --remove-module <module-id>
   ```

4. **Verify System Stability**
   ```bash
   # Reboot and test
   ```

5. **Report Issue**
   ```bash
   # Document issue with module
   # Report to module developer
   ```

### Database Corruption

1. **Stop Magisk Daemon**
   ```bash
   magisk --stop-daemon
   ```

2. **Backup Corrupted Database**
   ```bash
   cp /data/adb/magisk.db /data/adb/magisk.db.corrupted
   ```

3. **Restore from Backup**
   ```bash
   magisk-backup restore --db
   # Or restore from rollback point
   ```

4. **Verify Restoration**
   ```bash
   magisk --verify-db
   ```

5. **Restart Daemon**
   ```bash
   magisk --start-daemon
   ```

### Rollback Required

1. **List Rollback Points**
   ```bash
   magisk-rollback list
   ```

2. **Select Rollback Point**
   ```bash
   # Choose appropriate rollback point
   ```

3. **Perform Rollback**
   ```bash
   magisk-rollback restore <rollback-id>
   ```

4. **Verify Rollback**
   ```bash
   magisk --verify-boot
   magisk --verify-modules
   magisk --verify-db
   ```

5. **Document Reason**
   ```bash
   # Document why rollback was needed
   # Update procedures to prevent recurrence
   ```

---

## Security Checks

### Daily Security Scan

- [ ] **1. Check for Unauthorized Root Grants**
  ```bash
  magisk-audit query --primitive su_grant --since today
  # Review all root grants
  ```

- [ ] **2. Verify No Unknown Modules**
  ```bash
  magisk --list-modules
  # Confirm all modules are known and authorized
  ```

- [ ] **3. Check SELinux Status**
  ```bash
  getenforce
  sestatus
  # Verify SELinux is in expected state
  ```

- [ ] **4. Review Failed Operations**
  ```bash
  magisk-audit query --status ERROR --since today
  # Investigate any failed operations
  ```

- [ ] **5. Check for Anomalies**
  ```bash
  magisk-metrics anomalies --since today
  # Review any detected anomalies
  ```

### Weekly Security Review

- [ ] **1. Audit All Root Access**
  ```bash
  magisk-audit security-report --days 7
  # Full security audit
  ```

- [ ] **2. Review Module Permissions**
  ```bash
  for module in $(magisk --list-modules); do
    magisk --show-module-permissions $module
  done
  ```

- [ ] **3. Check File Integrity**
  ```bash
  magisk --verify-system-integrity
  # Check for unauthorized modifications
  ```

- [ ] **4. Update Security Hashes**
  ```bash
  magisk --update-security-hashes
  # Update baseline hashes for verification
  ```

- [ ] **5. Review Network Activity**
  ```bash
  magisk-audit query --context network --days 7
  # Review all network operations
  ```

### Monthly Security Audit

- [ ] **1. Comprehensive Security Scan**
  ```bash
  magisk-audit security-full-scan
  # Complete security analysis
  ```

- [ ] **2. Update Security Policies**
  ```bash
  # Review and update SELinux policies if needed
  ```

- [ ] **3. Review Access Logs**
  ```bash
  # Analyze all su_grant/su_deny operations
  ```

- [ ] **4. Check for Updates**
  ```bash
  # Ensure Magisk and all modules are up to date
  # Check for security patches
  ```

- [ ] **5. Document Security Status**
  ```bash
  # Create monthly security report
  # Document any incidents or concerns
  ```

---

## Performance Optimization

### Weekly Performance Review

- [ ] **1. Analyze CPU Usage**
  ```bash
  magisk-metrics history --metric cpu --days 7
  # Identify high CPU usage patterns
  ```

- [ ] **2. Review Memory Usage**
  ```bash
  magisk-metrics history --metric memory --days 7
  # Check for memory leaks
  ```

- [ ] **3. Check I/O Performance**
  ```bash
  magisk-metrics history --metric io --days 7
  # Identify I/O bottlenecks
  ```

- [ ] **4. Identify Hotspots**
  ```bash
  magisk-metrics analyze-hotspots
  # Find performance bottlenecks
  ```

- [ ] **5. Optimize Configuration**
  ```bash
  # Adjust settings based on analysis
  # Disable unnecessary modules
  ```

---

## Compliance and Documentation

### Monthly Documentation Update

- [ ] **1. Update RAFAELIA Framework**
  ```bash
  # Review and update framework documentation
  ```

- [ ] **2. Document Configuration Changes**
  ```bash
  # Record all configuration modifications
  ```

- [ ] **3. Update Module List**
  ```bash
  # Document current modules and their purposes
  ```

- [ ] **4. Review Audit Reports**
  ```bash
  # Analyze monthly audit reports
  # Identify trends and issues
  ```

- [ ] **5. Update Runbooks**
  ```bash
  # Update operational procedures
  # Document lessons learned
  ```

- [ ] **6. Backup Documentation**
  ```bash
  # Backup all documentation
  # Store securely
  ```

---

## Quick Reference Commands

### Status and Info
```bash
magisk --status                    # Magisk status
magisk --version                   # Version info
magisk --list-modules              # List active modules
magisk-metrics snapshot            # Current metrics
magisk-audit summary              # Audit summary
```

### Verification
```bash
magisk --verify-boot              # Verify boot image
magisk --verify-modules           # Verify all modules
magisk --verify-db                # Verify database
magisk --verify-integrity         # Full integrity check
```

### Backup and Rollback
```bash
magisk-backup create --full       # Full backup
magisk-backup verify --all        # Verify backups
magisk-rollback list              # List rollback points
magisk-rollback create            # Create rollback point
magisk-rollback restore <id>      # Restore rollback
```

### Monitoring
```bash
magisk-metrics snapshot           # Current snapshot
magisk-metrics history            # Metrics history
magisk-audit view --last 20       # Last 20 audit entries
magisk-audit query --status ERROR # Query errors
```

### Emergency
```bash
magisk --disable-all-modules      # Disable all modules
magisk --safe-mode                # Boot to safe mode
magisk --uninstall                # Uninstall Magisk
magisk-rollback restore <id>      # Emergency rollback
```

---

**Status**: Operational Checklist Complete ✓  
**Usage**: Follow checklists for safe Magisk operations  
**Author**: ∆RafaelVerboΩ  
**Cycle**: VAZIO → VERBO → CHEIO → RETRO
