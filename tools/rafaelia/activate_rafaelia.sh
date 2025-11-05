#!/system/bin/sh
#
# RAFAELIA Activation Script
# Enables and initializes the RAFAELIA framework
# Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
#

# Version configuration
RAFAELIA_VERSION="1.1.0"

MAGISK_DIR="/data/adb/magisk"
AUDIT_DIR="$MAGISK_DIR/rafaelia_audit"
METRICS_DIR="$MAGISK_DIR/rafaelia_metrics"
MANIFEST_FILE="/data/adb/RAFAELIA_MANIFEST.json"
TOOLS_DIR="/data/local/tmp/rafaelia"

COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_CYAN='\033[0;36m'
COLOR_RESET='\033[0m'

print_color() {
    color=$1
    shift
    printf "${color}%s${COLOR_RESET}\n" "$*"
}

print_header() {
    echo ""
    print_color "$COLOR_CYAN" "============================================================"
    print_color "$COLOR_CYAN" "$1"
    print_color "$COLOR_CYAN" "============================================================"
}

print_banner() {
    print_color "$COLOR_BLUE" ""
    print_color "$COLOR_BLUE" "  ██████╗  █████╗ ███████╗ █████╗ ███████╗██╗     ██╗ █████╗ "
    print_color "$COLOR_BLUE" "  ██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔════╝██║     ██║██╔══██╗"
    print_color "$COLOR_BLUE" "  ██████╔╝███████║█████╗  ███████║█████╗  ██║     ██║███████║"
    print_color "$COLOR_BLUE" "  ██╔══██╗██╔══██║██╔══╝  ██╔══██║██╔══╝  ██║     ██║██╔══██║"
    print_color "$COLOR_BLUE" "  ██║  ██║██║  ██║██║     ██║  ██║███████╗███████╗██║██║  ██║"
    print_color "$COLOR_BLUE" "  ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═╝"
    print_color "$COLOR_BLUE" ""
    print_color "$COLOR_CYAN" "  Recursively Auditable Fractal Architecture"
    print_color "$COLOR_CYAN" "  for Ethical and Logical Integrity Assurance"
    print_color "$COLOR_CYAN" ""
    print_color "$COLOR_YELLOW" "  Philosophy: VAZIO → VERBO → CHEIO → RETRO"
    print_color "$COLOR_YELLOW" "  Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ"
    print_color "$COLOR_BLUE" ""
}

check_root() {
    if [ "$(id -u)" != "0" ]; then
        print_color "$COLOR_RED" "Error: This script must be run as root"
        exit 1
    fi
}

create_directories() {
    print_header "Creating RAFAELIA Directories"
    
    # Create audit directory
    if mkdir -p "$AUDIT_DIR" 2>/dev/null; then
        chmod 700 "$AUDIT_DIR"
        print_color "$COLOR_GREEN" "✓ Created audit directory: $AUDIT_DIR"
    else
        print_color "$COLOR_RED" "✗ Failed to create audit directory"
        return 1
    fi
    
    # Create metrics directory
    if mkdir -p "$METRICS_DIR" 2>/dev/null; then
        chmod 700 "$METRICS_DIR"
        print_color "$COLOR_GREEN" "✓ Created metrics directory: $METRICS_DIR"
    else
        print_color "$COLOR_RED" "✗ Failed to create metrics directory"
        return 1
    fi
    
    return 0
}

create_manifest() {
    print_header "Creating RAFAELIA Manifest"
    
    if [ -f "$MANIFEST_FILE" ]; then
        print_color "$COLOR_YELLOW" "⚠ Manifest already exists, skipping"
        return 0
    fi
    
    timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    cat > "$MANIFEST_FILE" <<EOF
{
  "signature": "RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ",
  "timestamp": "$timestamp",
  "version": "$RAFAELIA_VERSION",
  "status": "activated",
  "components": {
    "audit_system": true,
    "telemetry_system": true,
    "state_validator": true,
    "metrics_collector": true,
    "integrity_checker": true
  },
  "configuration": {
    "audit_enabled": true,
    "telemetry_enabled": true,
    "metrics_interval_sec": 5,
    "max_audit_history": 1000,
    "max_metrics_samples": 1000
  },
  "notes": "RAFAELIA framework activated on $timestamp"
}
EOF
    
    if [ $? -eq 0 ]; then
        chmod 600 "$MANIFEST_FILE"
        print_color "$COLOR_GREEN" "✓ Created manifest: $MANIFEST_FILE"
        return 0
    else
        print_color "$COLOR_RED" "✗ Failed to create manifest"
        return 1
    fi
}

setup_tools() {
    print_header "Setting Up RAFAELIA Tools"
    
    # Check if tools directory exists
    if [ ! -d "$TOOLS_DIR" ]; then
        print_color "$COLOR_YELLOW" "⚠ Tools directory not found: $TOOLS_DIR"
        print_color "$COLOR_YELLOW" "  You can copy tools from the repository:"
        print_color "$COLOR_YELLOW" "  adb push tools/rafaelia /data/local/tmp/"
        return 0
    fi
    
    # Make scripts executable
    chmod +x "$TOOLS_DIR"/*.sh 2>/dev/null
    chmod +x "$TOOLS_DIR"/*.py 2>/dev/null
    
    # Check for each tool
    tools_found=0
    for tool in audit_analyzer.py state_validator.py metrics_collector.sh integrity_checker.sh; do
        if [ -f "$TOOLS_DIR/$tool" ]; then
            print_color "$COLOR_GREEN" "✓ Found: $tool"
            tools_found=$((tools_found + 1))
        else
            print_color "$COLOR_YELLOW" "⚠ Missing: $tool"
        fi
    done
    
    if [ $tools_found -gt 0 ]; then
        print_color "$COLOR_GREEN" "✓ Tools setup complete ($tools_found tools available)"
    fi
    
    return 0
}

start_services() {
    print_header "Starting RAFAELIA Services"
    
    # Start metrics collector if available
    if [ -f "$TOOLS_DIR/metrics_collector.sh" ]; then
        if pgrep -f "metrics_collector.sh" > /dev/null; then
            print_color "$COLOR_YELLOW" "⚠ Metrics collector already running"
        else
            nohup "$TOOLS_DIR/metrics_collector.sh" run > /dev/null 2>&1 &
            sleep 1
            if pgrep -f "metrics_collector.sh" > /dev/null; then
                print_color "$COLOR_GREEN" "✓ Started metrics collector (PID: $(pgrep -f 'metrics_collector.sh'))"
            else
                print_color "$COLOR_YELLOW" "⚠ Failed to start metrics collector"
            fi
        fi
    fi
    
    return 0
}

run_integrity_check() {
    print_header "Running Initial Integrity Check"
    
    if [ -f "$TOOLS_DIR/integrity_checker.sh" ]; then
        "$TOOLS_DIR/integrity_checker.sh" full
    else
        print_color "$COLOR_YELLOW" "⚠ Integrity checker not available"
    fi
    
    return 0
}

print_status() {
    print_header "RAFAELIA Status"
    
    echo ""
    print_color "$COLOR_CYAN" "Directories:"
    [ -d "$AUDIT_DIR" ] && print_color "$COLOR_GREEN" "  ✓ Audit:   $AUDIT_DIR" || print_color "$COLOR_RED" "  ✗ Audit directory missing"
    [ -d "$METRICS_DIR" ] && print_color "$COLOR_GREEN" "  ✓ Metrics: $METRICS_DIR" || print_color "$COLOR_RED" "  ✗ Metrics directory missing"
    
    echo ""
    print_color "$COLOR_CYAN" "Files:"
    [ -f "$MANIFEST_FILE" ] && print_color "$COLOR_GREEN" "  ✓ Manifest: $MANIFEST_FILE" || print_color "$COLOR_YELLOW" "  ⚠ Manifest not found"
    
    echo ""
    print_color "$COLOR_CYAN" "Services:"
    if pgrep -f "metrics_collector.sh" > /dev/null; then
        print_color "$COLOR_GREEN" "  ✓ Metrics collector running"
    else
        print_color "$COLOR_YELLOW" "  ⚠ Metrics collector not running"
    fi
    
    echo ""
    print_color "$COLOR_CYAN" "Audit Logs:"
    audit_count=$(find "$AUDIT_DIR" -name "audit_*.jsonl" 2>/dev/null | wc -l)
    [ $audit_count -gt 0 ] && print_color "$COLOR_GREEN" "  ✓ $audit_count audit log file(s)" || print_color "$COLOR_YELLOW" "  ⚠ No audit logs yet"
    
    echo ""
    print_color "$COLOR_CYAN" "Metrics:"
    metrics_count=$(find "$METRICS_DIR" -name "metrics_*.jsonl" 2>/dev/null | wc -l)
    [ $metrics_count -gt 0 ] && print_color "$COLOR_GREEN" "  ✓ $metrics_count metrics file(s)" || print_color "$COLOR_YELLOW" "  ⚠ No metrics collected yet"
    
    echo ""
}

show_usage() {
    cat <<EOF
RAFAELIA Activation Script
Usage: $0 [command]

Commands:
    activate    Full activation (create dirs, manifest, start services)
    status      Show current status
    start       Start RAFAELIA services
    stop        Stop RAFAELIA services
    check       Run integrity check
    help        Show this help message

Examples:
    # Full activation
    $0 activate
    
    # Check status
    $0 status
    
    # Run integrity check
    $0 check
EOF
}

stop_services() {
    print_header "Stopping RAFAELIA Services"
    
    # Stop metrics collector
    if pgrep -f "metrics_collector.sh" > /dev/null; then
        pkill -f "metrics_collector.sh"
        sleep 1
        if pgrep -f "metrics_collector.sh" > /dev/null; then
            print_color "$COLOR_RED" "✗ Failed to stop metrics collector"
        else
            print_color "$COLOR_GREEN" "✓ Stopped metrics collector"
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ Metrics collector not running"
    fi
}

main_activate() {
    print_banner
    check_root
    
    create_directories
    create_manifest
    setup_tools
    start_services
    run_integrity_check
    
    print_header "Activation Complete"
    print_color "$COLOR_GREEN" "✓ RAFAELIA framework has been activated!"
    print_color "$COLOR_CYAN" ""
    print_color "$COLOR_CYAN" "Next steps:"
    print_color "$COLOR_CYAN" "  1. Review status: $0 status"
    print_color "$COLOR_CYAN" "  2. View audit logs: tail -f $AUDIT_DIR/audit_*.jsonl"
    print_color "$COLOR_CYAN" "  3. View metrics: tail -f $METRICS_DIR/metrics_*.jsonl"
    print_color "$COLOR_CYAN" "  4. Run integrity check: $0 check"
    echo ""
}

# Main command handler
case "${1:-activate}" in
    activate)
        main_activate
        ;;
    status)
        print_banner
        check_root
        print_status
        ;;
    start)
        check_root
        start_services
        ;;
    stop)
        check_root
        stop_services
        ;;
    check)
        check_root
        run_integrity_check
        ;;
    help|--help|-h)
        show_usage
        ;;
    *)
        echo "Unknown command: $1" >&2
        echo "Run '$0 help' for usage information" >&2
        exit 1
        ;;
esac
