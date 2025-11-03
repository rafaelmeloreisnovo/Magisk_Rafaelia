#!/system/bin/sh
#
# RAFAELIA Metrics Collector
# Collects system metrics and stores them for analysis
# Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
#

METRICS_DIR="/data/adb/magisk/rafaelia_metrics"
INTERVAL=5  # seconds
MAX_SAMPLES=1000

# Create metrics directory
mkdir -p "$METRICS_DIR"

# Get timestamp
get_timestamp() {
    date +%s
}

# Collect CPU metrics
collect_cpu() {
    if [ -f /proc/stat ]; then
        grep "^cpu " /proc/stat | awk '{
            user=$2; nice=$3; system=$4; idle=$5; 
            iowait=$6; irq=$7; softirq=$8;
            total=user+nice+system+idle+iowait+irq+softirq;
            printf "{\"user\":%d,\"system\":%d,\"idle\":%d,\"total\":%d}", user, system, idle, total
        }'
    else
        echo "{}"
    fi
}

# Collect memory metrics
collect_memory() {
    if [ -f /proc/meminfo ]; then
        awk '
            /MemTotal/ { total=$2 }
            /MemAvailable/ { available=$2 }
            /MemFree/ { free=$2 }
            END {
                used = total - available;
                usage = (total > 0) ? (used * 100.0 / total) : 0;
                printf "{\"total\":%d,\"available\":%d,\"used\":%d,\"usage\":%.2f}", total, available, used, usage
            }
        ' /proc/meminfo
    else
        echo "{}"
    fi
}

# Collect I/O metrics
collect_io() {
    if [ -f /proc/diskstats ]; then
        awk '{
            read_ops+=$4; read_bytes+=$6*512;
            write_ops+=$8; write_bytes+=$10*512;
        } END {
            printf "{\"read_bytes\":%d,\"write_bytes\":%d,\"read_ops\":%d,\"write_ops\":%d}", 
                read_bytes, write_bytes, read_ops, write_ops
        }' /proc/diskstats
    else
        echo "{}"
    fi
}

# Collect network metrics
collect_network() {
    if [ -f /proc/net/dev ]; then
        awk '
            NR>2 && !/lo:/ {
                rx_bytes+=$2; rx_packets+=$3;
                tx_bytes+=$10; tx_packets+=$11;
            } END {
                printf "{\"rx_bytes\":%d,\"tx_bytes\":%d,\"rx_packets\":%d,\"tx_packets\":%d}",
                    rx_bytes, tx_bytes, rx_packets, tx_packets
            }
        ' /proc/net/dev
    else
        echo "{}"
    fi
}

# Collect load average
collect_load() {
    if [ -f /proc/loadavg ]; then
        read load1 load5 load15 rest < /proc/loadavg
        printf "{\"load1\":%s,\"load5\":%s,\"load15\":%s}" "$load1" "$load5" "$load15"
    else
        echo "{}"
    fi
}

# Collect process count
collect_processes() {
    proc_count=$(ls -d /proc/[0-9]* 2>/dev/null | wc -l)
    printf "{\"count\":%d}" "$proc_count"
}

# Collect temperature (if available)
collect_temperature() {
    temp_file="/sys/class/thermal/thermal_zone0/temp"
    if [ -f "$temp_file" ]; then
        temp=$(cat "$temp_file")
        # Convert from millidegrees to degrees
        temp_c=$((temp / 1000))
        printf "{\"temp_c\":%d}" "$temp_c"
    else
        echo "{}"
    fi
}

# Collect battery status (if available)
collect_battery() {
    battery_cap="/sys/class/power_supply/battery/capacity"
    battery_status="/sys/class/power_supply/battery/status"
    
    if [ -f "$battery_cap" ] && [ -f "$battery_status" ]; then
        capacity=$(cat "$battery_cap")
        status=$(cat "$battery_status")
        printf "{\"capacity\":%d,\"status\":\"%s\"}" "$capacity" "$status"
    else
        echo "{}"
    fi
}

# Collect all metrics
collect_all_metrics() {
    timestamp=$(get_timestamp)
    cpu=$(collect_cpu)
    memory=$(collect_memory)
    io=$(collect_io)
    network=$(collect_network)
    load=$(collect_load)
    processes=$(collect_processes)
    temperature=$(collect_temperature)
    battery=$(collect_battery)
    
    # Create JSON output
    printf "{\"timestamp\":%d,\"cpu\":%s,\"memory\":%s,\"io\":%s,\"network\":%s,\"load\":%s,\"processes\":%s,\"temperature\":%s,\"battery\":%s}\n" \
        "$timestamp" "$cpu" "$memory" "$io" "$network" "$load" "$processes" "$temperature" "$battery"
}

# Rotate log files if too large
rotate_logs() {
    metrics_file="$METRICS_DIR/metrics_$(date +%Y%m%d).jsonl"
    
    if [ -f "$metrics_file" ]; then
        line_count=$(wc -l < "$metrics_file")
        if [ "$line_count" -gt "$MAX_SAMPLES" ]; then
            # Rotate: keep only last MAX_SAMPLES/2 lines
            keep_lines=$((MAX_SAMPLES / 2))
            tail -n "$keep_lines" "$metrics_file" > "$metrics_file.tmp"
            mv "$metrics_file.tmp" "$metrics_file"
            echo "Rotated metrics file, kept $keep_lines samples" >&2
        fi
    fi
}

# Main collection loop
main() {
    echo "RAFAELIA Metrics Collector started"
    echo "Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ"
    echo "Interval: ${INTERVAL}s, Max samples: $MAX_SAMPLES"
    echo "Metrics dir: $METRICS_DIR"
    echo ""
    
    while true; do
        metrics_file="$METRICS_DIR/metrics_$(date +%Y%m%d).jsonl"
        
        # Collect and save metrics
        collect_all_metrics >> "$metrics_file"
        
        # Rotate if needed (every 10 iterations)
        if [ $(($(date +%s) % 50)) -eq 0 ]; then
            rotate_logs
        fi
        
        # Wait for next interval
        sleep "$INTERVAL"
    done
}

# Handle command line arguments
case "${1:-run}" in
    run)
        main
        ;;
    once)
        collect_all_metrics
        ;;
    help|--help|-h)
        cat <<EOF
RAFAELIA Metrics Collector
Usage: $0 [command]

Commands:
    run     Run continuous collection (default)
    once    Collect metrics once and exit
    help    Show this help message

Environment variables:
    INTERVAL    Collection interval in seconds (default: 5)
    MAX_SAMPLES Maximum samples per file before rotation (default: 1000)

Examples:
    # Run in background
    $0 run &
    
    # Collect once
    $0 once
    
    # Custom interval
    INTERVAL=10 $0 run &
EOF
        ;;
    *)
        echo "Unknown command: $1" >&2
        echo "Run '$0 help' for usage information" >&2
        exit 1
        ;;
esac
