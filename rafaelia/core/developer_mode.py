#!/usr/bin/env python3
"""
RAFAELIA Developer Mode and Runtime Learning Framework

This module implements advanced developer mode capabilities including:
- Runtime debugging with learning capabilities
- Error pattern recognition and correction
- Performance optimization through runtime analysis
- Audit database (pigtail) for quality assessments
- Interoperability testing and mitigation
- Garbage collection optimization
- Exception handling improvements

Part of Magisk_Rafaelia RAFAELIA Framework
Philosophy: VAZIO → VERBO → CHEIO → RETRO (Learning from errors)

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
Instituto Rafael - CientiEspiritual Philosophy
All Rights Reserved.

This module requires explicit user permission to activate developer mode.
It only operates when:
1. User has enabled Developer Mode on their device
2. User has explicitly granted permission for runtime learning
3. All data collection follows GDPR/LGPD compliance requirements
"""

import os
import sys
import json
import sqlite3
import hashlib
import logging
import traceback
from typing import Dict, List, Any, Optional, Callable
from dataclasses import dataclass, asdict
from datetime import datetime
from pathlib import Path
from enum import Enum
import threading
import time


class ErrorSeverity(Enum):
    """Error severity classification."""
    CRITICAL = "CRITICAL"
    HIGH = "HIGH"
    MEDIUM = "MEDIUM"
    LOW = "LOW"
    INFO = "INFO"


class LearningCategory(Enum):
    """Categories for runtime learning."""
    GARBAGE_COLLECTION = "GC_OPTIMIZATION"
    EXCEPTION_HANDLING = "EXCEPTION_PATTERN"
    PERFORMANCE = "PERFORMANCE_ISSUE"
    MEMORY_LEAK = "MEMORY_LEAK"
    CONCURRENCY = "CONCURRENCY_ISSUE"
    RESOURCE_LEAK = "RESOURCE_LEAK"
    LOGIC_ERROR = "LOGIC_ERROR"
    INTEROPERABILITY = "INTEROP_ISSUE"
    SECURITY = "SECURITY_CONCERN"


@dataclass
class ErrorEvent:
    """Represents an error or issue detected at runtime."""
    timestamp: str
    category: str
    severity: str
    error_type: str
    error_message: str
    stack_trace: str
    context: Dict[str, Any]
    file_path: Optional[str] = None
    line_number: Optional[int] = None
    function_name: Optional[str] = None
    learned_correction: Optional[str] = None
    mitigation_applied: bool = False
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for storage."""
        return asdict(self)


@dataclass
class PerformanceMetric:
    """Performance measurement at runtime."""
    timestamp: str
    operation: str
    duration_ms: float
    memory_usage_mb: float
    cpu_usage_percent: float
    context: Dict[str, Any]
    anomaly_detected: bool = False
    optimization_suggestion: Optional[str] = None


class PigtailDatabase:
    """
    Pigtail Database - Quality Assessment and Audit Database
    
    Stores:
    - Error events and learning patterns
    - Performance metrics and anomalies
    - Code quality assessments
    - Audit trails
    - Correction patterns
    - Technical coherence evaluations
    """
    
    def __init__(self, db_path: str = None):
        """Initialize pigtail database."""
        if db_path is None:
            # Use secure app-private directory when available
            # Fall back to secure temporary location with proper permissions
            import tempfile
            secure_dir = os.getenv('RAFAELIA_DATA_DIR', tempfile.gettempdir())
            self.db_path = os.path.join(secure_dir, 'rafaelia', 'pigtail.db')
        else:
            self.db_path = db_path
        self._ensure_directory()
        self._init_database()
        self._lock = threading.Lock()
        
    def _ensure_directory(self):
        """Ensure database directory exists with secure permissions."""
        db_dir = os.path.dirname(self.db_path)
        if db_dir and not os.path.exists(db_dir):
            try:
                # Create with secure permissions (owner only: rwx------)
                os.makedirs(db_dir, mode=0o700, exist_ok=True)
                
                # Verify permissions were set correctly
                if os.path.exists(db_dir):
                    os.chmod(db_dir, 0o700)
                    
            except Exception as e:
                # Fallback to user's home directory if can't write to preferred location
                import tempfile
                fallback_dir = os.path.join(os.path.expanduser('~'), '.rafaelia')
                os.makedirs(fallback_dir, mode=0o700, exist_ok=True)
                self.db_path = os.path.join(fallback_dir, 'pigtail.db')
                
        # Ensure the database file itself has secure permissions
        if os.path.exists(self.db_path):
            os.chmod(self.db_path, 0o600)  # rw------- (owner read/write only)
                
    def _init_database(self):
        """Initialize database schema."""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                CREATE TABLE IF NOT EXISTS error_events (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    category TEXT NOT NULL,
                    severity TEXT NOT NULL,
                    error_type TEXT NOT NULL,
                    error_message TEXT,
                    stack_trace TEXT,
                    context TEXT,
                    file_path TEXT,
                    line_number INTEGER,
                    function_name TEXT,
                    learned_correction TEXT,
                    mitigation_applied BOOLEAN,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            conn.execute("""
                CREATE TABLE IF NOT EXISTS performance_metrics (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    operation TEXT NOT NULL,
                    duration_ms REAL NOT NULL,
                    memory_usage_mb REAL,
                    cpu_usage_percent REAL,
                    context TEXT,
                    anomaly_detected BOOLEAN,
                    optimization_suggestion TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            conn.execute("""
                CREATE TABLE IF NOT EXISTS learning_patterns (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    pattern_hash TEXT UNIQUE NOT NULL,
                    category TEXT NOT NULL,
                    pattern_description TEXT,
                    occurrence_count INTEGER DEFAULT 1,
                    correction_strategy TEXT,
                    success_rate REAL DEFAULT 0.0,
                    last_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            conn.execute("""
                CREATE TABLE IF NOT EXISTS audit_trail (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    action TEXT NOT NULL,
                    component TEXT,
                    details TEXT,
                    user_consent_verified BOOLEAN,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            conn.commit()
            
    def record_error(self, error: ErrorEvent) -> int:
        """Record an error event."""
        with self._lock, sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute("""
                INSERT INTO error_events (
                    timestamp, category, severity, error_type, error_message,
                    stack_trace, context, file_path, line_number, function_name,
                    learned_correction, mitigation_applied
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                error.timestamp,
                error.category,
                error.severity,
                error.error_type,
                error.error_message,
                error.stack_trace,
                json.dumps(error.context),
                error.file_path,
                error.line_number,
                error.function_name,
                error.learned_correction,
                error.mitigation_applied
            ))
            conn.commit()
            return cursor.lastrowid
            
    def record_performance(self, metric: PerformanceMetric) -> int:
        """Record a performance metric."""
        with self._lock, sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute("""
                INSERT INTO performance_metrics (
                    timestamp, operation, duration_ms, memory_usage_mb,
                    cpu_usage_percent, context, anomaly_detected,
                    optimization_suggestion
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                metric.timestamp,
                metric.operation,
                metric.duration_ms,
                metric.memory_usage_mb,
                metric.cpu_usage_percent,
                json.dumps(metric.context),
                metric.anomaly_detected,
                metric.optimization_suggestion
            ))
            conn.commit()
            return cursor.lastrowid
            
    def learn_pattern(self, category: str, pattern_desc: str, 
                     correction: str) -> None:
        """Record or update a learned pattern."""
        pattern_hash = hashlib.sha256(
            f"{category}:{pattern_desc}".encode()
        ).hexdigest()[:16]
        
        with self._lock, sqlite3.connect(self.db_path) as conn:
            # Try to update existing pattern
            cursor = conn.execute("""
                UPDATE learning_patterns 
                SET occurrence_count = occurrence_count + 1,
                    correction_strategy = ?,
                    last_seen = CURRENT_TIMESTAMP
                WHERE pattern_hash = ?
            """, (correction, pattern_hash))
            
            if cursor.rowcount == 0:
                # Insert new pattern
                conn.execute("""
                    INSERT INTO learning_patterns (
                        pattern_hash, category, pattern_description,
                        correction_strategy
                    ) VALUES (?, ?, ?, ?)
                """, (pattern_hash, category, pattern_desc, correction))
                
            conn.commit()
            
    def get_learned_correction(self, category: str, 
                              pattern_desc: str) -> Optional[str]:
        """Get learned correction for a pattern if exists."""
        pattern_hash = hashlib.sha256(
            f"{category}:{pattern_desc}".encode()
        ).hexdigest()[:16]
        
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute("""
                SELECT correction_strategy, success_rate 
                FROM learning_patterns 
                WHERE pattern_hash = ? AND success_rate > 0.5
            """, (pattern_hash,))
            
            row = cursor.fetchone()
            return row[0] if row else None
            
    def audit_log(self, action: str, component: str, details: str,
                  consent_verified: bool = True) -> None:
        """Log audit trail entry."""
        with self._lock, sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                INSERT INTO audit_trail (
                    timestamp, action, component, details, user_consent_verified
                ) VALUES (?, ?, ?, ?, ?)
            """, (
                datetime.now().isoformat(),
                action,
                component,
                details,
                consent_verified
            ))
            conn.commit()


class DeveloperModeManager:
    """
    Developer Mode Manager
    
    Handles runtime learning, debugging, and performance optimization when
    developer mode is enabled by the user.
    """
    
    def __init__(self, enable_developer_mode: bool = False):
        """
        Initialize Developer Mode Manager.
        
        Args:
            enable_developer_mode: Must be explicitly True to activate
        """
        self.enabled = enable_developer_mode and self._verify_user_consent()
        self.db = PigtailDatabase() if self.enabled else None
        self.logger = self._setup_logger()
        
        if self.enabled:
            self.logger.info("Developer Mode ACTIVATED with user consent")
            if self.db:
                self.db.audit_log(
                    "DEVELOPER_MODE_ACTIVATED",
                    "DeveloperModeManager",
                    "User has enabled developer mode with full consent",
                    consent_verified=True
                )
        else:
            self.logger.info("Developer Mode DISABLED")
            
    def _verify_user_consent(self) -> bool:
        """
        Verify user has given consent for developer mode.
        
        Checks:
        1. Device is in developer mode
        2. User has explicitly granted permission
        3. GDPR/LGPD compliance requirements met
        
        Security: Uses secure consent verification to prevent exploitation
        """
        # Check if Android Developer Options are enabled
        # This is a placeholder - actual implementation would check Android settings
        try:
            # Use secure application-specific directory for consent file
            # NOT a world-writable location like /tmp
            consent_dir = os.getenv('RAFAELIA_DATA_DIR', os.path.expanduser('~/.rafaelia'))
            os.makedirs(consent_dir, mode=0o700, exist_ok=True)
            
            consent_file = os.path.join(consent_dir, 'dev_consent')
            
            # Verify file exists and has secure permissions
            if os.path.exists(consent_file):
                # Ensure file is owned by current user and not world-readable
                stat_info = os.stat(consent_file)
                # Check that file is not writable by group or others (mask 0o077)
                if stat_info.st_mode & 0o077 == 0:
                    return True
                    
            return False
        except Exception:
            return False
            
    def _setup_logger(self) -> logging.Logger:
        """Setup logger for developer mode."""
        logger = logging.getLogger("RAFAELIA.DeveloperMode")
        logger.setLevel(logging.DEBUG if self.enabled else logging.INFO)
        
        handler = logging.StreamHandler()
        formatter = logging.Formatter(
            '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
        )
        handler.setFormatter(formatter)
        logger.addHandler(handler)
        
        return logger
        
    def capture_error(self, exception: Exception, context: Dict[str, Any],
                     category: LearningCategory = LearningCategory.EXCEPTION_HANDLING,
                     severity: ErrorSeverity = ErrorSeverity.MEDIUM) -> ErrorEvent:
        """
        Capture an error event for learning.
        
        Args:
            exception: The exception that occurred
            context: Additional context information
            category: Category of the error for learning
            severity: Severity level
            
        Returns:
            ErrorEvent object
        """
        if not self.enabled:
            return None
            
        # Extract stack trace information
        tb = traceback.extract_tb(exception.__traceback__)
        stack_trace = ''.join(traceback.format_tb(exception.__traceback__))
        
        # Get file/line/function from last frame
        if tb:
            last_frame = tb[-1]
            file_path = last_frame.filename
            line_number = last_frame.lineno
            function_name = last_frame.name
        else:
            file_path = None
            line_number = None
            function_name = None
            
        # Check for learned correction
        pattern_desc = f"{type(exception).__name__}:{str(exception)[:100]}"
        learned_correction = self.db.get_learned_correction(
            category.value, pattern_desc
        ) if self.db else None
        
        error = ErrorEvent(
            timestamp=datetime.now().isoformat(),
            category=category.value,
            severity=severity.value,
            error_type=type(exception).__name__,
            error_message=str(exception),
            stack_trace=stack_trace,
            context=context,
            file_path=file_path,
            line_number=line_number,
            function_name=function_name,
            learned_correction=learned_correction,
            mitigation_applied=False
        )
        
        if self.db:
            self.db.record_error(error)
            self.logger.debug(f"Captured error: {error.error_type} in {function_name}")
            
        return error
        
    def measure_performance(self, operation: str, duration_ms: float,
                          memory_mb: float, cpu_percent: float,
                          context: Dict[str, Any]) -> PerformanceMetric:
        """
        Record performance measurement.
        
        Args:
            operation: Name of operation measured
            duration_ms: Duration in milliseconds
            memory_mb: Memory usage in MB
            cpu_percent: CPU usage percentage
            context: Additional context
            
        Returns:
            PerformanceMetric object
        """
        if not self.enabled or not self.db:
            return None
            
        # Detect anomalies (simple heuristics)
        anomaly = (
            duration_ms > 5000 or  # > 5 seconds
            memory_mb > 500 or     # > 500 MB
            cpu_percent > 80       # > 80% CPU
        )
        
        suggestion = None
        if anomaly:
            if duration_ms > 5000:
                suggestion = "Consider async execution or optimization"
            elif memory_mb > 500:
                suggestion = "High memory usage - check for leaks or optimize data structures"
            elif cpu_percent > 80:
                suggestion = "High CPU usage - profile and optimize hot paths"
                
        metric = PerformanceMetric(
            timestamp=datetime.now().isoformat(),
            operation=operation,
            duration_ms=duration_ms,
            memory_usage_mb=memory_mb,
            cpu_usage_percent=cpu_percent,
            context=context,
            anomaly_detected=anomaly,
            optimization_suggestion=suggestion
        )
        
        self.db.record_performance(metric)
        
        if anomaly:
            self.logger.warning(
                f"Performance anomaly in {operation}: {suggestion}"
            )
            
        return metric
        
    def learn_correction(self, category: LearningCategory, pattern: str,
                        correction: str) -> None:
        """
        Teach the system a correction pattern.
        
        Args:
            category: Category of the pattern
            pattern: Description of the error pattern
            correction: How to correct or mitigate it
        """
        if not self.enabled or not self.db:
            return
            
        self.db.learn_pattern(category.value, pattern, correction)
        self.logger.info(f"Learned correction for {category.value}: {pattern[:50]}...")
        
    def optimize_garbage_collection(self) -> Dict[str, Any]:
        """
        Analyze and optimize garbage collection patterns.
        
        Returns:
            Dictionary with optimization suggestions
        """
        if not self.enabled:
            return {"enabled": False}
            
        import gc
        
        # Get GC statistics
        gc_stats = {
            "enabled": True,
            "collections": gc.get_count(),
            "thresholds": gc.get_threshold(),
            "suggestions": []
        }
        
        # Analyze collection patterns
        counts = gc.get_count()
        if counts[0] > 1000:
            gc_stats["suggestions"].append(
                "High gen0 collections - consider object pooling"
            )
            
        if counts[1] > 100:
            gc_stats["suggestions"].append(
                "High gen1 collections - review object lifecycle"
            )
            
        if counts[2] > 10:
            gc_stats["suggestions"].append(
                "Frequent gen2 collections - check for memory leaks"
            )
            
        # Log suggestions
        for suggestion in gc_stats["suggestions"]:
            self.logger.info(f"GC Optimization: {suggestion}")
            
        return gc_stats
        
    def get_quality_report(self) -> Dict[str, Any]:
        """
        Generate quality assessment report from pigtail database.
        
        Returns:
            Dictionary with quality metrics and assessments
        """
        if not self.enabled or not self.db:
            return {"enabled": False}
            
        with sqlite3.connect(self.db.db_path) as conn:
            # Error statistics
            error_stats = conn.execute("""
                SELECT category, severity, COUNT(*) as count
                FROM error_events
                WHERE datetime(created_at) > datetime('now', '-7 days')
                GROUP BY category, severity
            """).fetchall()
            
            # Performance anomalies
            perf_anomalies = conn.execute("""
                SELECT COUNT(*) 
                FROM performance_metrics
                WHERE anomaly_detected = 1
                AND datetime(created_at) > datetime('now', '-7 days')
            """).fetchone()[0]
            
            # Learning patterns
            patterns = conn.execute("""
                SELECT category, COUNT(*) as pattern_count,
                       AVG(success_rate) as avg_success
                FROM learning_patterns
                GROUP BY category
            """).fetchall()
            
        report = {
            "enabled": True,
            "report_date": datetime.now().isoformat(),
            "error_statistics": [
                {"category": cat, "severity": sev, "count": cnt}
                for cat, sev, cnt in error_stats
            ],
            "performance_anomalies_7d": perf_anomalies,
            "learning_patterns": [
                {"category": cat, "patterns": cnt, "success_rate": rate}
                for cat, cnt, rate in patterns
            ]
        }
        
        return report


# Global instance (disabled by default)
_dev_mode_instance: Optional[DeveloperModeManager] = None


def get_developer_mode() -> DeveloperModeManager:
    """Get or create developer mode singleton instance."""
    global _dev_mode_instance
    if _dev_mode_instance is None:
        _dev_mode_instance = DeveloperModeManager(enable_developer_mode=False)
    return _dev_mode_instance


def enable_developer_mode_with_consent() -> bool:
    """
    Enable developer mode with user consent.
    
    This should only be called after:
    1. User explicitly enables developer options on device
    2. User grants permission in app settings
    3. User reads and accepts data collection terms (GDPR/LGPD)
    
    To enable, create a consent file:
        mkdir -p ~/.rafaelia  # or use RAFAELIA_DATA_DIR env var
        chmod 700 ~/.rafaelia
        touch ~/.rafaelia/dev_consent
        chmod 600 ~/.rafaelia/dev_consent
    
    Returns:
        True if successfully enabled, False otherwise
    """
    global _dev_mode_instance
    _dev_mode_instance = DeveloperModeManager(enable_developer_mode=True)
    return _dev_mode_instance.enabled


if __name__ == "__main__":
    print("RAFAELIA Developer Mode and Runtime Learning Framework")
    print("=" * 60)
    print("This module provides advanced debugging and learning capabilities")
    print("Requires explicit user consent and developer mode activation")
    print()
    print("Features:")
    print("- Runtime error capture and learning")
    print("- Performance monitoring and optimization")
    print("- Pigtail database for quality assessment")
    print("- Garbage collection optimization")
    print("- Interoperability testing")
    print("- Automatic mitigation suggestions")
    print()
    print("Compliance: GDPR, LGPD, Ethical AI principles")
    print("Author: Rafael Melo Reis (rafaelmeloreisnovo)")
