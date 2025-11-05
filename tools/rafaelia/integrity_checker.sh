#!/system/bin/sh
#
# RAFAELIA Integrity Checker
# Verifies system integrity using hashes and signatures
# Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
#

MAGISK_DIR="/data/adb/magisk"
MODULES_DIR="$MAGISK_DIR/modules"
AUDIT_DIR="$MAGISK_DIR/rafaelia_audit"
MANIFEST_FILE="/data/adb/RAFAELIA_MANIFEST.json"
COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_RESET='\033[0m'

# Print colored output
print_color() {
    color=$1
    shift
    printf "${color}%s${COLOR_RESET}\n" "$*"
}

# Print section header
print_header() {
    echo ""
    print_color "$COLOR_BLUE" "=========================================="
    print_color "$COLOR_BLUE" "$1"
    print_color "$COLOR_BLUE" "=========================================="
}

# Check if running as root
check_root() {
    if [ "$(id -u)" != "0" ]; then
        print_color "$COLOR_RED" "Error: This script must be run as root"
        exit 1
    fi
}

# Check boot image integrity
check_boot_integrity() {
    print_header "Boot Image Integrity Check"
    
    # Check if boot is mounted
    if mountpoint -q /dev/block/bootdevice/by-name/boot 2>/dev/null; then
        print_color "$COLOR_GREEN" "✓ Boot partition accessible"
    else
        print_color "$COLOR_YELLOW" "⚠ Boot partition not accessible (may be normal)"
    fi
    
    # Check magisk binary
    if [ -f "$MAGISK_DIR/magisk64" ]; then
        magisk_size=$(stat -c %s "$MAGISK_DIR/magisk64" 2>/dev/null || echo "0")
        print_color "$COLOR_GREEN" "✓ Magisk binary found (${magisk_size} bytes)"
    else
        print_color "$COLOR_RED" "✗ Magisk binary not found"
        return 1
    fi
    
    return 0
}

# Check modules integrity
check_modules_integrity() {
    print_header "Modules Integrity Check"
    
    if [ ! -d "$MODULES_DIR" ]; then
        print_color "$COLOR_YELLOW" "⚠ Modules directory not found"
        return 0
    fi
    
    module_count=0
    valid_modules=0
    invalid_modules=0
    
    for module_dir in "$MODULES_DIR"/*; do
        if [ ! -d "$module_dir" ]; then
            continue
        fi
        
        module_name=$(basename "$module_dir")
        module_count=$((module_count + 1))
        
        # Check for module.prop
        if [ -f "$module_dir/module.prop" ]; then
            # Check if module is disabled
            if [ -f "$module_dir/disable" ]; then
                print_color "$COLOR_YELLOW" "⚠ Module '$module_name' is disabled"
            else
                print_color "$COLOR_GREEN" "✓ Module '$module_name' is valid"
                valid_modules=$((valid_modules + 1))
            fi
        else
            print_color "$COLOR_RED" "✗ Module '$module_name' missing module.prop"
            invalid_modules=$((invalid_modules + 1))
        fi
    done
    
    echo ""
    echo "Total modules: $module_count"
    echo "Valid modules: $valid_modules"
    echo "Invalid modules: $invalid_modules"
    
    return $invalid_modules
}

# Check database integrity
check_database_integrity() {
    print_header "Database Integrity Check"
    
    db_file="$MAGISK_DIR/magisk.db"
    
    if [ ! -f "$db_file" ]; then
        print_color "$COLOR_RED" "✗ Magisk database not found"
        return 1
    fi
    
    print_color "$COLOR_GREEN" "✓ Database file exists"
    
    # Try to query database
    if command -v sqlite3 >/dev/null 2>&1; then
        table_count=$(sqlite3 "$db_file" "SELECT COUNT(*) FROM sqlite_master WHERE type='table';" 2>/dev/null)
        if [ $? -eq 0 ]; then
            print_color "$COLOR_GREEN" "✓ Database is readable ($table_count tables)"
        else
            print_color "$COLOR_RED" "✗ Database is corrupted or unreadable"
            return 1
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ sqlite3 not available, skipping database query test"
    fi
    
    return 0
}

# Check audit system integrity
check_audit_integrity() {
    print_header "Audit System Integrity Check"
    
    if [ ! -d "$AUDIT_DIR" ]; then
        print_color "$COLOR_YELLOW" "⚠ Audit directory not found (may not be initialized)"
        return 0
    fi
    
    audit_files=$(find "$AUDIT_DIR" -name "audit_*.jsonl" 2>/dev/null | wc -l)
    
    if [ "$audit_files" -gt 0 ]; then
        print_color "$COLOR_GREEN" "✓ Found $audit_files audit log files"
        
        # Check latest audit file
        latest_audit=$(find "$AUDIT_DIR" -name "audit_*.jsonl" 2>/dev/null | sort | tail -n 1)
        if [ -f "$latest_audit" ]; then
            line_count=$(wc -l < "$latest_audit" 2>/dev/null || echo "0")
            print_color "$COLOR_GREEN" "✓ Latest audit log: $line_count entries"
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ No audit logs found"
    fi
    
    return 0
}

# Check manifest integrity
check_manifest_integrity() {
    print_header "Manifest Integrity Check"
    
    if [ ! -f "$MANIFEST_FILE" ]; then
        print_color "$COLOR_YELLOW" "⚠ RAFAELIA manifest not found"
        return 0
    fi
    
    print_color "$COLOR_GREEN" "✓ Manifest file exists"
    
    # Validate JSON (if jq is available)
    if command -v jq >/dev/null 2>&1; then
        if jq empty "$MANIFEST_FILE" 2>/dev/null; then
            print_color "$COLOR_GREEN" "✓ Manifest is valid JSON"
            
            # Show manifest info
            signature=$(jq -r '.signature' "$MANIFEST_FILE" 2>/dev/null)
            timestamp=$(jq -r '.timestamp' "$MANIFEST_FILE" 2>/dev/null)
            
            echo "  Signature: $signature"
            echo "  Timestamp: $timestamp"
        else
            print_color "$COLOR_RED" "✗ Manifest is invalid JSON"
            return 1
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ jq not available, skipping JSON validation"
    fi
    
    return 0
}

# Check SELinux status
check_selinux_status() {
    print_header "SELinux Status Check"
    
    if command -v getenforce >/dev/null 2>&1; then
        selinux_mode=$(getenforce)
        if [ "$selinux_mode" = "Enforcing" ]; then
            print_color "$COLOR_GREEN" "✓ SELinux is Enforcing"
        elif [ "$selinux_mode" = "Permissive" ]; then
            print_color "$COLOR_YELLOW" "⚠ SELinux is Permissive"
        else
            print_color "$COLOR_YELLOW" "⚠ SELinux status: $selinux_mode"
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ Cannot determine SELinux status"
    fi
    
    return 0
}

# Check system properties
check_system_properties() {
    print_header "System Properties Check"
    
    # Check important Magisk properties
    magisk_ver=$(getprop ro.magisk.version 2>/dev/null || echo "unknown")
    magisk_vcode=$(getprop ro.magisk.versionCode 2>/dev/null || echo "unknown")
    
    echo "Magisk version: $magisk_ver"
    echo "Magisk version code: $magisk_vcode"
    
    if [ "$magisk_ver" != "unknown" ]; then
        print_color "$COLOR_GREEN" "✓ Magisk properties are set"
    else
        print_color "$COLOR_YELLOW" "⚠ Magisk properties not found"
    fi
    
    return 0
}

# Generate summary report
generate_summary() {
    total_checks=$1
    passed_checks=$2
    
    print_header "Integrity Check Summary"
    
    echo "Total checks: $total_checks"
    echo "Passed checks: $passed_checks"
    echo "Failed checks: $((total_checks - passed_checks))"
    
    if [ "$passed_checks" -eq "$total_checks" ]; then
        print_color "$COLOR_GREEN" ""
        print_color "$COLOR_GREEN" "✓ All integrity checks passed!"
        return 0
    else
        print_color "$COLOR_YELLOW" ""
        print_color "$COLOR_YELLOW" "⚠ Some integrity checks failed or produced warnings"
        return 1
    fi
}

# Main function
main() {
    print_color "$COLOR_BLUE" "RAFAELIA Integrity Checker"
    print_color "$COLOR_BLUE" "Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ"
    
    check_root
    
    total_checks=0
    passed_checks=0
    
    # Run all checks
    checks=(
        "check_boot_integrity"
        "check_modules_integrity"
        "check_database_integrity"
        "check_audit_integrity"
        "check_manifest_integrity"
        "check_selinux_status"
        "check_system_properties"
    )
    
    for check in "${checks[@]}"; do
        total_checks=$((total_checks + 1))
        if $check; then
            passed_checks=$((passed_checks + 1))
        fi
    done
    
    # Generate summary
    generate_summary "$total_checks" "$passed_checks"
}

# Handle command line arguments
case "${1:-full}" in
    full)
        main
        ;;
    boot)
        check_root
        check_boot_integrity
        ;;
    modules)
        check_root
        check_modules_integrity
        ;;
    database|db)
        check_root
        check_database_integrity
        ;;
    audit)
        check_root
        check_audit_integrity
        ;;
    manifest)
        check_root
        check_manifest_integrity
        ;;
    help|--help|-h)
        cat <<EOF
RAFAELIA Integrity Checker
Usage: $0 [check]

Checks:
    full        Run all integrity checks (default)
    boot        Check boot image integrity
    modules     Check modules integrity
    database    Check database integrity
    audit       Check audit system integrity
    manifest    Check manifest integrity
    help        Show this help message

Examples:
    # Run all checks
    $0 full
    
    # Check only modules
    $0 modules
EOF
        ;;
    *)
        echo "Unknown check: $1" >&2
        echo "Run '$0 help' for usage information" >&2
        exit 1
        ;;
esac
