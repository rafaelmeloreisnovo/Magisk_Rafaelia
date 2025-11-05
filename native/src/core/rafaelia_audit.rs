// RAFAELIA Audit System
// Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
// Philosophy: VAZIO → VERBO → CHEIO → RETRO

use base::{libc, LogLevel, ResultExt, Utf8CStr, cstr};
use std::collections::VecDeque;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Write, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const AUDIT_LOG_DIR: &str = "/data/adb/magisk/rafaelia_audit";
const MAX_AUDIT_HISTORY: usize = 1000;
const AUDIT_BUFFER_SIZE: usize = 8192;

/// Represents a single audit entry in the RAFAELIA system
#[derive(Clone, Debug)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub session_id: String,
    pub primitive: String,
    pub context: String,
    pub action: String,
    pub state_id: String,
    pub input_hash: Option<String>,
    pub output_hash: Option<String>,
    pub duration_ms: u64,
    pub success: bool,
    pub error_msg: Option<String>,
}

impl AuditEntry {
    /// Convert audit entry to JSON line format
    pub fn to_jsonl(&self) -> String {
        let mut json = String::with_capacity(512);
        json.push('{');
        json.push_str(&format!(r#""timestamp":{},"#, self.timestamp));
        json.push_str(&format!(r#""session_id":"{}","#, self.session_id));
        json.push_str(&format!(r#""primitive":"{}","#, self.primitive));
        json.push_str(&format!(r#""context":"{}","#, self.context));
        json.push_str(&format!(r#""action":"{}","#, self.action));
        json.push_str(&format!(r#""state_id":"{}","#, self.state_id));
        
        if let Some(ref hash) = self.input_hash {
            json.push_str(&format!(r#""input_hash":"{}","#, hash));
        }
        if let Some(ref hash) = self.output_hash {
            json.push_str(&format!(r#""output_hash":"{}","#, hash));
        }
        
        json.push_str(&format!(r#""duration_ms":{},"#, self.duration_ms));
        json.push_str(&format!(r#""success":{},"#, self.success));
        
        if let Some(ref msg) = self.error_msg {
            json.push_str(&format!(r#""error_msg":"{}","#, msg.replace('"', "\\\"")));
        }
        
        // Remove trailing comma if exists
        if json.ends_with(',') {
            json.pop();
        }
        json.push('}');
        json
    }
}

/// Rollback point for recovery
#[derive(Clone, Debug)]
pub struct RollbackPoint {
    pub id: String,
    pub timestamp: u64,
    pub primitive: String,
    pub context: String,
    pub state_snapshot: Vec<u8>,
    pub audit_index: usize,
}

impl RollbackPoint {
    /// Generate a unique rollback ID
    fn generate_id() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        format!("RB_{}", timestamp)
    }
}

/// Main RAFAELIA Audit System
pub struct AuditSystem {
    session_id: String,
    log_file: Arc<Mutex<Option<BufWriter<File>>>>,
    audit_history: Arc<Mutex<VecDeque<AuditEntry>>>,
    rollback_points: Arc<Mutex<Vec<RollbackPoint>>>,
    enabled: bool,
}

impl AuditSystem {
    /// Initialize the audit system with a new session
    pub fn init() -> Result<Self, std::io::Error> {
        let session_id = Self::generate_session_id();
        
        // Create audit directory if it doesn't exist
        create_dir_all(AUDIT_LOG_DIR)?;
        
        // Open log file
        let log_path = Self::get_log_path(&session_id);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;
        
        let log_file = Arc::new(Mutex::new(Some(BufWriter::with_capacity(
            AUDIT_BUFFER_SIZE,
            file,
        ))));
        
        let audit_history = Arc::new(Mutex::new(VecDeque::with_capacity(MAX_AUDIT_HISTORY)));
        let rollback_points = Arc::new(Mutex::new(Vec::new()));
        
        Ok(AuditSystem {
            session_id,
            log_file,
            audit_history,
            rollback_points,
            enabled: true,
        })
    }
    
    /// Generate a unique session ID
    fn generate_session_id() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let pid = unsafe { libc::getpid() };
        format!("RAFAELIA_{}_{}", timestamp, pid)
    }
    
    /// Get log file path for session
    fn get_log_path(session_id: &str) -> PathBuf {
        PathBuf::from(AUDIT_LOG_DIR).join(format!("audit_{}.jsonl", session_id))
    }
    
    /// Log an operation to the audit system
    pub fn log_operation(
        &mut self,
        primitive: &str,
        context: &str,
        action: &str,
        duration_ms: u64,
        success: bool,
        error_msg: Option<String>,
    ) -> Result<(), std::io::Error> {
        if !self.enabled {
            return Ok(());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let state_id = format!("PRIM_{}_CTX_{}", primitive, context);
        
        let entry = AuditEntry {
            timestamp,
            session_id: self.session_id.clone(),
            primitive: primitive.to_string(),
            context: context.to_string(),
            action: action.to_string(),
            state_id,
            input_hash: None,
            output_hash: None,
            duration_ms,
            success,
            error_msg,
        };
        
        // Write to log file
        if let Ok(mut log) = self.log_file.lock() {
            if let Some(ref mut writer) = *log {
                let jsonl = entry.to_jsonl();
                writeln!(writer, "{}", jsonl)?;
                writer.flush()?;
            }
        }
        
        // Add to history
        if let Ok(mut history) = self.audit_history.lock() {
            if history.len() >= MAX_AUDIT_HISTORY {
                history.pop_front();
            }
            history.push_back(entry);
        }
        
        Ok(())
    }
    
    /// Create a rollback point
    pub fn create_rollback_point(
        &mut self,
        primitive: &str,
        context: &str,
    ) -> Result<RollbackPoint, std::io::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let audit_index = if let Ok(history) = self.audit_history.lock() {
            history.len()
        } else {
            0
        };
        
        let rollback_point = RollbackPoint {
            id: RollbackPoint::generate_id(),
            timestamp,
            primitive: primitive.to_string(),
            context: context.to_string(),
            state_snapshot: Vec::new(), // Placeholder for actual state
            audit_index,
        };
        
        // Store rollback point
        if let Ok(mut points) = self.rollback_points.lock() {
            points.push(rollback_point.clone());
        }
        
        // Log rollback point creation
        self.log_operation(
            primitive,
            context,
            "CREATE_ROLLBACK",
            0,
            true,
            None,
        )?;
        
        Ok(rollback_point)
    }
    
    /// Perform rollback to a specific point
    pub fn perform_rollback(&mut self, point_id: &str) -> Result<(), std::io::Error> {
        let rollback_point = if let Ok(points) = self.rollback_points.lock() {
            points.iter().find(|p| p.id == point_id).cloned()
        } else {
            None
        };
        
        if let Some(point) = rollback_point {
            // Log rollback operation
            self.log_operation(
                &point.primitive,
                &point.context,
                "PERFORM_ROLLBACK",
                0,
                true,
                Some(format!("Rollback to point: {}", point_id)),
            )?;
            
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Rollback point not found: {}", point_id),
            ))
        }
    }
    
    /// Get audit statistics
    pub fn get_statistics(&self) -> AuditStatistics {
        let history = self.audit_history.lock().unwrap();
        
        let total_operations = history.len();
        let successful_operations = history.iter().filter(|e| e.success).count();
        let failed_operations = total_operations - successful_operations;
        
        let avg_duration = if total_operations > 0 {
            history.iter().map(|e| e.duration_ms).sum::<u64>() / total_operations as u64
        } else {
            0
        };
        
        AuditStatistics {
            total_operations,
            successful_operations,
            failed_operations,
            avg_duration_ms: avg_duration,
            session_id: self.session_id.clone(),
        }
    }
    
    /// Enable or disable audit logging
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Flush all pending writes
    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        if let Ok(mut log) = self.log_file.lock() {
            if let Some(ref mut writer) = *log {
                writer.flush()?;
            }
        }
        Ok(())
    }
}

impl Drop for AuditSystem {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

/// Statistics about audit operations
#[derive(Debug, Clone)]
pub struct AuditStatistics {
    pub total_operations: usize,
    pub successful_operations: usize,
    pub failed_operations: usize,
    pub avg_duration_ms: u64,
    pub session_id: String,
}

use std::sync::OnceLock;

/// Global audit system instance (optional, for convenience)
/// Thread-safe initialization using OnceLock
static GLOBAL_AUDIT: OnceLock<Arc<Mutex<AuditSystem>>> = OnceLock::new();

/// Initialize global audit system
/// Returns Ok if initialized successfully, or if already initialized
pub fn init_global_audit() -> Result<(), std::io::Error> {
    GLOBAL_AUDIT.get_or_try_init(|| -> Result<Arc<Mutex<AuditSystem>>, std::io::Error> {
        let audit = AuditSystem::init()?;
        Ok(Arc::new(Mutex::new(audit)))
    })?;
    Ok(())
}

/// Get reference to global audit system
pub fn get_global_audit() -> Option<Arc<Mutex<AuditSystem>>> {
    GLOBAL_AUDIT.get().cloned()
}

/// Log operation to global audit system (convenience function)
pub fn log_global_operation(
    primitive: &str,
    context: &str,
    action: &str,
    duration_ms: u64,
    success: bool,
    error_msg: Option<String>,
) {
    if let Some(audit_arc) = get_global_audit() {
        match audit_arc.lock() {
            Ok(mut audit) => {
                if let Err(e) = audit.log_operation(primitive, context, action, duration_ms, success, error_msg) {
                    eprintln!("RAFAELIA Audit log_operation error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("RAFAELIA Audit lock error: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_entry_to_jsonl() {
        let entry = AuditEntry {
            timestamp: 1234567890,
            session_id: "test_session".to_string(),
            primitive: "boot_patch".to_string(),
            context: "boot".to_string(),
            action: "EXECUTE".to_string(),
            state_id: "PRIM_boot_patch_CTX_boot".to_string(),
            input_hash: Some("abc123".to_string()),
            output_hash: Some("def456".to_string()),
            duration_ms: 100,
            success: true,
            error_msg: None,
        };
        
        let json = entry.to_jsonl();
        assert!(json.contains("\"timestamp\":1234567890"));
        assert!(json.contains("\"primitive\":\"boot_patch\""));
        assert!(json.contains("\"success\":true"));
    }
    
    #[test]
    fn test_rollback_point_id_generation() {
        let id1 = RollbackPoint::generate_id();
        let id2 = RollbackPoint::generate_id();
        assert_ne!(id1, id2);
        assert!(id1.starts_with("RB_"));
    }
}
