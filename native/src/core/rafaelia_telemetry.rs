// RAFAELIA Telemetry System
// Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
// Philosophy: VAZIO → VERBO → CHEIO → RETRO

use base::{libc, ResultExt};
use std::collections::VecDeque;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const MAX_METRICS_HISTORY: usize = 100;
const DEFAULT_COLLECT_INTERVAL_MS: u64 = 1000;

/// CPU metrics
#[derive(Clone, Debug)]
pub struct CpuMetrics {
    pub usage_percent: f64,
    pub user_time: u64,
    pub system_time: u64,
    pub idle_time: u64,
    pub timestamp: u64,
}

/// Memory metrics
#[derive(Clone, Debug)]
pub struct MemoryMetrics {
    pub total_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
    pub usage_percent: f64,
    pub timestamp: u64,
}

/// I/O metrics
#[derive(Clone, Debug)]
pub struct IoMetrics {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_ops: u64,
    pub write_ops: u64,
    pub timestamp: u64,
}

/// Network metrics
#[derive(Clone, Debug)]
pub struct NetworkMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub timestamp: u64,
}

/// Complete metrics snapshot
#[derive(Clone, Debug)]
pub struct MetricsSnapshot {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub io: IoMetrics,
    pub network: NetworkMetrics,
    pub timestamp: u64,
}

impl MetricsSnapshot {
    /// Convert metrics to JSON string
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"timestamp":{},"cpu":{{"usage":{:.2},"user":{},"system":{},"idle":{}}},"memory":{{"total":{},"available":{},"used":{},"usage":{:.2}}},"io":{{"read_bytes":{},"write_bytes":{},"read_ops":{},"write_ops":{}}},"network":{{"rx_bytes":{},"tx_bytes":{},"rx_packets":{},"tx_packets":{}}}}}"#,
            self.timestamp,
            self.cpu.usage_percent,
            self.cpu.user_time,
            self.cpu.system_time,
            self.cpu.idle_time,
            self.memory.total_kb,
            self.memory.available_kb,
            self.memory.used_kb,
            self.memory.usage_percent,
            self.io.read_bytes,
            self.io.write_bytes,
            self.io.read_ops,
            self.io.write_ops,
            self.network.rx_bytes,
            self.network.tx_bytes,
            self.network.rx_packets,
            self.network.tx_packets,
        )
    }
}

/// Telemetry collector with monitoring daemon
pub struct TelemetryCollector {
    interval_ms: u64,
    metrics_history: Arc<Mutex<VecDeque<MetricsSnapshot>>>,
    running: Arc<Mutex<bool>>,
    prev_cpu_stats: Arc<Mutex<Option<CpuStats>>>,
    prev_io_stats: Arc<Mutex<Option<IoStats>>>,
    prev_net_stats: Arc<Mutex<Option<NetStats>>>,
}

// Internal CPU stats for delta calculation
#[derive(Clone, Debug)]
struct CpuStats {
    user: u64,
    system: u64,
    idle: u64,
    total: u64,
}

// Internal I/O stats
#[derive(Clone, Debug)]
struct IoStats {
    read_bytes: u64,
    write_bytes: u64,
    read_ops: u64,
    write_ops: u64,
}

// Internal network stats
#[derive(Clone, Debug)]
struct NetStats {
    rx_bytes: u64,
    tx_bytes: u64,
    rx_packets: u64,
    tx_packets: u64,
}

impl TelemetryCollector {
    /// Create a new telemetry collector
    pub fn new(interval_ms: u64) -> Result<Self, io::Error> {
        Ok(TelemetryCollector {
            interval_ms,
            metrics_history: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_METRICS_HISTORY))),
            running: Arc::new(Mutex::new(false)),
            prev_cpu_stats: Arc::new(Mutex::new(None)),
            prev_io_stats: Arc::new(Mutex::new(None)),
            prev_net_stats: Arc::new(Mutex::new(None)),
        })
    }
    
    /// Start the telemetry collection daemon
    pub fn start(&mut self) -> Result<(), io::Error> {
        let mut running = self.running.lock().unwrap();
        if *running {
            return Ok(()); // Already running
        }
        *running = true;
        
        let interval = Duration::from_millis(self.interval_ms);
        let metrics_history = Arc::clone(&self.metrics_history);
        let running_flag = Arc::clone(&self.running);
        let prev_cpu = Arc::clone(&self.prev_cpu_stats);
        let prev_io = Arc::clone(&self.prev_io_stats);
        let prev_net = Arc::clone(&self.prev_net_stats);
        
        thread::spawn(move || {
            while *running_flag.lock().unwrap() {
                let snapshot = Self::collect_metrics_internal(&prev_cpu, &prev_io, &prev_net);
                
                if let Some(snapshot) = snapshot {
                    let mut history = metrics_history.lock().unwrap();
                    if history.len() >= MAX_METRICS_HISTORY {
                        history.pop_front();
                    }
                    history.push_back(snapshot);
                }
                
                thread::sleep(interval);
            }
        });
        
        Ok(())
    }
    
    /// Stop the telemetry collection daemon
    pub fn stop(&mut self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }
    
    /// Get the latest metrics snapshot
    pub fn get_snapshot(&self) -> Option<MetricsSnapshot> {
        let history = self.metrics_history.lock().unwrap();
        history.back().cloned()
    }
    
    /// Get all metrics history
    pub fn get_history(&self) -> Vec<MetricsSnapshot> {
        let history = self.metrics_history.lock().unwrap();
        history.iter().cloned().collect()
    }
    
    /// Collect metrics manually (one-time)
    pub fn collect(&self) -> Option<MetricsSnapshot> {
        Self::collect_metrics_internal(
            &self.prev_cpu_stats,
            &self.prev_io_stats,
            &self.prev_net_stats,
        )
    }
    
    // Internal metric collection
    fn collect_metrics_internal(
        prev_cpu: &Arc<Mutex<Option<CpuStats>>>,
        prev_io: &Arc<Mutex<Option<IoStats>>>,
        prev_net: &Arc<Mutex<Option<NetStats>>>,
    ) -> Option<MetricsSnapshot> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs();
        
        let cpu = Self::collect_cpu_metrics(prev_cpu)?;
        let memory = Self::collect_memory_metrics()?;
        let io = Self::collect_io_metrics(prev_io)?;
        let network = Self::collect_network_metrics(prev_net)?;
        
        Some(MetricsSnapshot {
            cpu,
            memory,
            io,
            network,
            timestamp,
        })
    }
    
    /// Collect CPU metrics from /proc/stat
    fn collect_cpu_metrics(prev_stats: &Arc<Mutex<Option<CpuStats>>>) -> Option<CpuMetrics> {
        let stat_data = read_to_string("/proc/stat").ok()?;
        let cpu_line = stat_data.lines().next()?;
        
        let parts: Vec<&str> = cpu_line.split_whitespace().collect();
        if parts.len() < 5 || parts[0] != "cpu" {
            return None;
        }
        
        let user: u64 = parts[1].parse().ok()?;
        let system: u64 = parts[3].parse().ok()?;
        let idle: u64 = parts[4].parse().ok()?;
        let total = user + system + idle;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs();
        
        let current = CpuStats {
            user,
            system,
            idle,
            total,
        };
        
        let usage_percent = if let Some(prev) = prev_stats.lock().unwrap().as_ref() {
            let delta_total = current.total.saturating_sub(prev.total);
            let delta_idle = current.idle.saturating_sub(prev.idle);
            if delta_total > 0 {
                100.0 * (1.0 - (delta_idle as f64 / delta_total as f64))
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        *prev_stats.lock().unwrap() = Some(current.clone());
        
        Some(CpuMetrics {
            usage_percent,
            user_time: user,
            system_time: system,
            idle_time: idle,
            timestamp,
        })
    }
    
    /// Collect memory metrics from /proc/meminfo
    fn collect_memory_metrics() -> Option<MemoryMetrics> {
        let meminfo = read_to_string("/proc/meminfo").ok()?;
        
        let mut total_kb = 0u64;
        let mut available_kb = 0u64;
        
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line.split_whitespace().nth(1)?.parse().ok()?;
            } else if line.starts_with("MemAvailable:") {
                available_kb = line.split_whitespace().nth(1)?.parse().ok()?;
            }
        }
        
        let used_kb = total_kb.saturating_sub(available_kb);
        let usage_percent = if total_kb > 0 {
            (used_kb as f64 / total_kb as f64) * 100.0
        } else {
            0.0
        };
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs();
        
        Some(MemoryMetrics {
            total_kb,
            available_kb,
            used_kb,
            usage_percent,
            timestamp,
        })
    }
    
    /// Collect I/O metrics from /proc/diskstats
    fn collect_io_metrics(prev_stats: &Arc<Mutex<Option<IoStats>>>) -> Option<IoMetrics> {
        let diskstats = read_to_string("/proc/diskstats").ok()?;
        
        let mut read_bytes = 0u64;
        let mut write_bytes = 0u64;
        let mut read_ops = 0u64;
        let mut write_ops = 0u64;
        
        for line in diskstats.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 {
                // Sum all block devices
                read_ops += parts[3].parse::<u64>().unwrap_or(0);
                read_bytes += parts[5].parse::<u64>().unwrap_or(0) * 512; // sectors to bytes
                write_ops += parts[7].parse::<u64>().unwrap_or(0);
                write_bytes += parts[9].parse::<u64>().unwrap_or(0) * 512;
            }
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs();
        
        let current = IoStats {
            read_bytes,
            write_bytes,
            read_ops,
            write_ops,
        };
        
        // Calculate deltas if we have previous stats
        let (final_read, final_write, final_read_ops, final_write_ops) = 
            if let Some(prev) = prev_stats.lock().unwrap().as_ref() {
                (
                    current.read_bytes.saturating_sub(prev.read_bytes),
                    current.write_bytes.saturating_sub(prev.write_bytes),
                    current.read_ops.saturating_sub(prev.read_ops),
                    current.write_ops.saturating_sub(prev.write_ops),
                )
            } else {
                (0, 0, 0, 0)
            };
        
        *prev_stats.lock().unwrap() = Some(current);
        
        Some(IoMetrics {
            read_bytes: final_read,
            write_bytes: final_write,
            read_ops: final_read_ops,
            write_ops: final_write_ops,
            timestamp,
        })
    }
    
    /// Collect network metrics from /proc/net/dev
    fn collect_network_metrics(prev_stats: &Arc<Mutex<Option<NetStats>>>) -> Option<NetworkMetrics> {
        let netdev = read_to_string("/proc/net/dev").ok()?;
        
        let mut rx_bytes = 0u64;
        let mut tx_bytes = 0u64;
        let mut rx_packets = 0u64;
        let mut tx_packets = 0u64;
        
        for line in netdev.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 && !parts[0].starts_with("lo:") {
                // Skip loopback, sum all other interfaces
                rx_bytes += parts[1].parse::<u64>().unwrap_or(0);
                rx_packets += parts[2].parse::<u64>().unwrap_or(0);
                tx_bytes += parts[9].parse::<u64>().unwrap_or(0);
                tx_packets += parts[10].parse::<u64>().unwrap_or(0);
            }
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs();
        
        let current = NetStats {
            rx_bytes,
            tx_bytes,
            rx_packets,
            tx_packets,
        };
        
        // Calculate deltas
        let (final_rx, final_tx, final_rx_pkts, final_tx_pkts) = 
            if let Some(prev) = prev_stats.lock().unwrap().as_ref() {
                (
                    current.rx_bytes.saturating_sub(prev.rx_bytes),
                    current.tx_bytes.saturating_sub(prev.tx_bytes),
                    current.rx_packets.saturating_sub(prev.rx_packets),
                    current.tx_packets.saturating_sub(prev.tx_packets),
                )
            } else {
                (0, 0, 0, 0)
            };
        
        *prev_stats.lock().unwrap() = Some(current);
        
        Some(NetworkMetrics {
            rx_bytes: final_rx,
            tx_bytes: final_tx,
            rx_packets: final_rx_pkts,
            tx_packets: final_tx_pkts,
            timestamp,
        })
    }
}

impl Drop for TelemetryCollector {
    fn drop(&mut self) {
        self.stop();
    }
}

use std::sync::OnceLock;

/// Global telemetry collector instance (optional)
/// Thread-safe initialization using OnceLock
static GLOBAL_TELEMETRY: OnceLock<Arc<Mutex<TelemetryCollector>>> = OnceLock::new();

/// Initialize global telemetry collector
/// Returns Ok if initialized successfully, or if already initialized
pub fn init_global_telemetry(interval_ms: u64) -> Result<(), io::Error> {
    GLOBAL_TELEMETRY.get_or_try_init(|| {
        let collector = TelemetryCollector::new(interval_ms)?;
        Ok(Arc::new(Mutex::new(collector)))
    })?;
    Ok(())
}

/// Get reference to global telemetry collector
pub fn get_global_telemetry() -> Option<Arc<Mutex<TelemetryCollector>>> {
    GLOBAL_TELEMETRY.get().cloned()
}

/// Start global telemetry collection
pub fn start_global_telemetry() -> Result<(), io::Error> {
    if let Some(telemetry_arc) = get_global_telemetry() {
        if let Ok(mut telemetry) = telemetry_arc.lock() {
            telemetry.start()?;
        }
    }
    Ok(())
}

/// Get latest snapshot from global telemetry
pub fn get_global_snapshot() -> Option<MetricsSnapshot> {
    if let Some(telemetry_arc) = get_global_telemetry() {
        if let Ok(telemetry) = telemetry_arc.lock() {
            return telemetry.get_snapshot();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_snapshot_to_json() {
        let snapshot = MetricsSnapshot {
            timestamp: 1234567890,
            cpu: CpuMetrics {
                usage_percent: 45.5,
                user_time: 1000,
                system_time: 500,
                idle_time: 2000,
                timestamp: 1234567890,
            },
            memory: MemoryMetrics {
                total_kb: 4096000,
                available_kb: 2048000,
                used_kb: 2048000,
                usage_percent: 50.0,
                timestamp: 1234567890,
            },
            io: IoMetrics {
                read_bytes: 1024,
                write_bytes: 2048,
                read_ops: 10,
                write_ops: 20,
                timestamp: 1234567890,
            },
            network: NetworkMetrics {
                rx_bytes: 4096,
                tx_bytes: 8192,
                rx_packets: 100,
                tx_packets: 200,
                timestamp: 1234567890,
            },
        };
        
        let json = snapshot.to_json();
        assert!(json.contains("\"timestamp\":1234567890"));
        assert!(json.contains("\"usage\":45.5"));
    }
}
