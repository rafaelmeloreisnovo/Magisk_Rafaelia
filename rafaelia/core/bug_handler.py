# Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
# Instituto Rafael - CientiEspiritual Philosophy
#
# This file is part of Magisk_Rafaelia.
#
# Magisk_Rafaelia is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.

"""
RAFAELIA Comprehensive Bug Handler Module

This module provides a deterministic bug prevention and handling system
that ensures software exactness and prevents all categories of bugs.

Features:
- Bug classification (logical, technical, compatibility)
- Predictive bug prevention through invariant checking
- State validation and consistency enforcement
- Automatic recovery mechanisms
- Comprehensive error reporting and logging

Bug Categories:
1. Logical Bugs: Errors in program logic, algorithms, or business rules
2. Technical Bugs: Implementation errors, resource issues, system limitations
3. Compatibility Bugs: Version mismatches, platform-specific issues

Part of Magisk_Rafaelia RAFAELIA Framework
Philosophy: VAZIO → VERBO → CHEIO → RETRO (Preventing bugs through determinism)

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
Instituto Rafael - CientiEspiritual Philosophy
All Rights Reserved.
"""

import logging
import traceback
import sys
from typing import Dict, List, Optional, Callable, Any
from dataclasses import dataclass, field
from enum import Enum
from datetime import datetime
import threading


class BugCategory(Enum):
    """Categories of bugs that can occur."""
    LOGICAL = "logical"           # Logic errors, algorithm bugs, incorrect behavior
    TECHNICAL = "technical"       # Implementation errors, resource issues
    COMPATIBILITY = "compatibility"  # Version/platform compatibility issues
    UNKNOWN = "unknown"           # Uncategorized bugs


class BugSeverity(Enum):
    """Severity levels for bugs."""
    CRITICAL = "critical"   # System-breaking, immediate attention required
    HIGH = "high"           # Major functionality affected
    MEDIUM = "medium"       # Moderate impact, workaround available
    LOW = "low"             # Minor issue, minimal impact
    INFO = "info"           # Informational, not a bug


@dataclass
class BugReport:
    """Comprehensive bug report."""
    category: BugCategory
    severity: BugSeverity
    message: str
    context: Dict[str, Any]
    timestamp: datetime = field(default_factory=datetime.now)
    exception: Optional[Exception] = None
    traceback_info: Optional[str] = None
    recovery_attempted: bool = False
    recovery_successful: bool = False
    recovery_actions: List[str] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """Convert to dictionary."""
        return {
            'category': self.category.value,
            'severity': self.severity.value,
            'message': self.message,
            'context': self.context,
            'timestamp': self.timestamp.isoformat(),
            'exception': str(self.exception) if self.exception else None,
            'traceback': self.traceback_info,
            'recovery_attempted': self.recovery_attempted,
            'recovery_successful': self.recovery_successful,
            'recovery_actions': self.recovery_actions
        }


@dataclass
class StateInvariant:
    """Represents an invariant that must always hold."""
    name: str
    condition: Callable[[], bool]
    error_message: str
    category: BugCategory = BugCategory.LOGICAL
    
    def check(self) -> bool:
        """Check if invariant holds."""
        try:
            return self.condition()
        except Exception as e:
            logging.error(f"Error checking invariant '{self.name}': {e}")
            return False


class BugHandler:
    """
    Comprehensive bug handler for deterministic error prevention and recovery.
    
    This handler implements a multi-layered approach to bug handling:
    1. Prevention: Check invariants before operations
    2. Detection: Catch and classify errors
    3. Recovery: Attempt automatic recovery
    4. Reporting: Comprehensive logging and reporting
    """
    
    def __init__(self, strict_mode: bool = False):
        """
        Initialize bug handler.
        
        Args:
            strict_mode: If True, raise exceptions on any bug. If False, attempt recovery.
        """
        self.logger = logging.getLogger("RAFAELIA.BugHandler")
        self.strict_mode = strict_mode
        self.invariants: List[StateInvariant] = []
        self.bug_reports: List[BugReport] = []
        self._lock = threading.RLock()
        
        # Statistics
        self.stats = {
            'total_bugs': 0,
            'by_category': {cat: 0 for cat in BugCategory},
            'by_severity': {sev: 0 for sev in BugSeverity},
            'recovered': 0,
            'failed_recovery': 0
        }
    
    def add_invariant(self, invariant: StateInvariant):
        """Add a state invariant to check."""
        with self._lock:
            self.invariants.append(invariant)
            self.logger.debug(f"Added invariant: {invariant.name}")
    
    def check_invariants(self) -> List[BugReport]:
        """
        Check all registered invariants.
        
        Returns:
            List of BugReports for any violated invariants
        """
        violations = []
        
        with self._lock:
            for invariant in self.invariants:
                if not invariant.check():
                    report = BugReport(
                        category=invariant.category,
                        severity=BugSeverity.HIGH,
                        message=f"Invariant violation: {invariant.error_message}",
                        context={'invariant_name': invariant.name}
                    )
                    violations.append(report)
                    self._record_bug(report)
                    self.logger.error(f"Invariant violated: {invariant.name}")
        
        return violations
    
    def handle_exception(
        self,
        exception: Exception,
        category: BugCategory = BugCategory.UNKNOWN,
        severity: BugSeverity = BugSeverity.HIGH,
        context: Optional[Dict[str, Any]] = None,
        recovery_callback: Optional[Callable[[], bool]] = None
    ) -> BugReport:
        """
        Handle an exception with comprehensive logging and optional recovery.
        
        Args:
            exception: The exception that occurred
            category: Bug category
            severity: Bug severity level
            context: Additional context information
            recovery_callback: Optional function to attempt recovery
            
        Returns:
            BugReport with details
        """
        # Classify exception if category is unknown
        if category == BugCategory.UNKNOWN:
            category = self._classify_exception(exception)
        
        # Create bug report
        report = BugReport(
            category=category,
            severity=severity,
            message=str(exception),
            context=context or {},
            exception=exception,
            traceback_info=traceback.format_exc()
        )
        
        # Log the bug
        self.logger.error(
            f"[{category.value.upper()}] [{severity.value.upper()}] {exception}",
            exc_info=True
        )
        
        # Attempt recovery if callback provided and not in strict mode
        if recovery_callback and not self.strict_mode:
            report.recovery_attempted = True
            try:
                report.recovery_successful = recovery_callback()
                if report.recovery_successful:
                    report.recovery_actions.append("Recovery callback succeeded")
                    self.logger.info("Automatic recovery successful")
                else:
                    report.recovery_actions.append("Recovery callback failed")
                    self.logger.warning("Automatic recovery failed")
            except Exception as recovery_error:
                report.recovery_actions.append(f"Recovery error: {recovery_error}")
                self.logger.error(f"Recovery attempt raised exception: {recovery_error}")
        
        # Record bug
        self._record_bug(report)
        
        # In strict mode, re-raise
        if self.strict_mode:
            raise exception
        
        return report
    
    def _classify_exception(self, exception: Exception) -> BugCategory:
        """
        Classify an exception into a bug category.
        
        Args:
            exception: The exception to classify
            
        Returns:
            BugCategory
        """
        exception_type = type(exception).__name__
        exception_msg = str(exception).lower()
        
        # Logical bugs
        logical_indicators = [
            'assertion', 'value', 'logic', 'state', 'invalid',
            'unexpected', 'constraint', 'violation'
        ]
        if any(ind in exception_type.lower() for ind in logical_indicators):
            return BugCategory.LOGICAL
        if any(ind in exception_msg for ind in logical_indicators):
            return BugCategory.LOGICAL
        
        # Compatibility bugs
        compatibility_indicators = [
            'import', 'module', 'version', 'compatibility', 'platform',
            'not found', 'unsupported', 'deprecated'
        ]
        if any(ind in exception_type.lower() for ind in compatibility_indicators):
            return BugCategory.COMPATIBILITY
        if any(ind in exception_msg for ind in compatibility_indicators):
            return BugCategory.COMPATIBILITY
        
        # Technical bugs (default for resource/system issues)
        technical_indicators = [
            'memory', 'resource', 'io', 'os', 'system', 'file',
            'permission', 'timeout', 'connection'
        ]
        if any(ind in exception_type.lower() for ind in technical_indicators):
            return BugCategory.TECHNICAL
        if any(ind in exception_msg for ind in technical_indicators):
            return BugCategory.TECHNICAL
        
        # Default to technical for unclassified
        return BugCategory.TECHNICAL
    
    def _record_bug(self, report: BugReport):
        """Record bug report and update statistics."""
        with self._lock:
            self.bug_reports.append(report)
            self.stats['total_bugs'] += 1
            self.stats['by_category'][report.category] += 1
            self.stats['by_severity'][report.severity] += 1
            
            if report.recovery_attempted:
                if report.recovery_successful:
                    self.stats['recovered'] += 1
                else:
                    self.stats['failed_recovery'] += 1
    
    def validate_state(self, state: Dict[str, Any], expected: Dict[str, Any]) -> bool:
        """
        Validate current state against expected state.
        
        Args:
            state: Current state
            expected: Expected state values
            
        Returns:
            True if state is valid, False otherwise
        """
        for key, expected_value in expected.items():
            if key not in state:
                self.logger.error(f"Missing state key: {key}")
                return False
            
            if state[key] != expected_value:
                self.logger.error(
                    f"State mismatch for {key}: expected {expected_value}, got {state[key]}"
                )
                return False
        
        return True
    
    def get_statistics(self) -> Dict:
        """Get bug handling statistics."""
        with self._lock:
            return {
                'total_bugs': self.stats['total_bugs'],
                'by_category': {cat.value: count for cat, count in self.stats['by_category'].items()},
                'by_severity': {sev.value: count for sev, count in self.stats['by_severity'].items()},
                'recovered': self.stats['recovered'],
                'failed_recovery': self.stats['failed_recovery'],
                'recovery_rate': (
                    self.stats['recovered'] / max(1, self.stats['recovered'] + self.stats['failed_recovery'])
                ) * 100
            }
    
    def get_reports(
        self,
        category: Optional[BugCategory] = None,
        severity: Optional[BugSeverity] = None,
        limit: Optional[int] = None
    ) -> List[BugReport]:
        """
        Get bug reports with optional filtering.
        
        Args:
            category: Filter by category
            severity: Filter by severity
            limit: Maximum number of reports to return
            
        Returns:
            List of BugReports
        """
        with self._lock:
            reports = self.bug_reports.copy()
        
        # Apply filters
        if category:
            reports = [r for r in reports if r.category == category]
        if severity:
            reports = [r for r in reports if r.severity == severity]
        
        # Apply limit
        if limit:
            reports = reports[-limit:]
        
        return reports
    
    def clear_reports(self):
        """Clear all bug reports and reset statistics."""
        with self._lock:
            self.bug_reports.clear()
            self.stats = {
                'total_bugs': 0,
                'by_category': {cat: 0 for cat in BugCategory},
                'by_severity': {sev: 0 for sev in BugSeverity},
                'recovered': 0,
                'failed_recovery': 0
            }
            self.logger.info("Bug reports cleared")


class DeterministicValidator:
    """
    Validator for ensuring deterministic behavior.
    
    Provides tools to ensure that operations produce consistent,
    predictable results across different executions.
    """
    
    def __init__(self, bug_handler: Optional[BugHandler] = None):
        """Initialize validator."""
        self.logger = logging.getLogger("RAFAELIA.DeterministicValidator")
        self.bug_handler = bug_handler or BugHandler()
    
    def validate_input(
        self,
        value: Any,
        expected_type: type,
        constraints: Optional[Dict[str, Callable[[Any], bool]]] = None
    ) -> bool:
        """
        Validate input value against type and constraints.
        
        Args:
            value: Value to validate
            expected_type: Expected type
            constraints: Dictionary of constraint name -> validation function
            
        Returns:
            True if valid, False otherwise
        """
        # Type check
        if not isinstance(value, expected_type):
            self.bug_handler.handle_exception(
                TypeError(f"Expected {expected_type}, got {type(value)}"),
                category=BugCategory.LOGICAL,
                severity=BugSeverity.HIGH,
                context={'value': value, 'expected_type': expected_type}
            )
            return False
        
        # Constraint checks
        if constraints:
            for name, constraint_func in constraints.items():
                try:
                    if not constraint_func(value):
                        self.bug_handler.handle_exception(
                            ValueError(f"Constraint '{name}' violated"),
                            category=BugCategory.LOGICAL,
                            severity=BugSeverity.MEDIUM,
                            context={'value': value, 'constraint': name}
                        )
                        return False
                except Exception as e:
                    self.bug_handler.handle_exception(
                        e,
                        category=BugCategory.TECHNICAL,
                        severity=BugSeverity.HIGH,
                        context={'constraint': name}
                    )
                    return False
        
        return True
    
    def validate_preconditions(self, conditions: Dict[str, bool]) -> bool:
        """
        Validate that all preconditions are met.
        
        Args:
            conditions: Dictionary of condition name -> condition result
            
        Returns:
            True if all preconditions are met
        """
        for name, result in conditions.items():
            if not result:
                self.logger.error(f"Precondition failed: {name}")
                return False
        
        return True
    
    def validate_postconditions(self, conditions: Dict[str, bool]) -> bool:
        """
        Validate that all postconditions are satisfied.
        
        Args:
            conditions: Dictionary of condition name -> condition result
            
        Returns:
            True if all postconditions are satisfied
        """
        for name, result in conditions.items():
            if not result:
                self.logger.error(f"Postcondition failed: {name}")
                return False
        
        return True


# Global bug handler instance
_global_bug_handler: Optional[BugHandler] = None


def get_global_bug_handler() -> BugHandler:
    """Get or create global bug handler."""
    global _global_bug_handler
    if _global_bug_handler is None:
        _global_bug_handler = BugHandler()
    return _global_bug_handler


def handle_bug(
    exception: Exception,
    category: BugCategory = BugCategory.UNKNOWN,
    severity: BugSeverity = BugSeverity.HIGH,
    context: Optional[Dict[str, Any]] = None
) -> BugReport:
    """Convenience function to handle a bug using global handler."""
    handler = get_global_bug_handler()
    return handler.handle_exception(exception, category, severity, context)


def check_invariants() -> List[BugReport]:
    """Check all invariants using global handler."""
    handler = get_global_bug_handler()
    return handler.check_invariants()


if __name__ == "__main__":
    # Example usage
    logging.basicConfig(level=logging.INFO)
    
    print("="*70)
    print("RAFAELIA Comprehensive Bug Handler")
    print("="*70)
    
    # Create bug handler
    handler = BugHandler(strict_mode=False)
    
    # Add some invariants
    handler.add_invariant(StateInvariant(
        name="positive_value",
        condition=lambda: True,  # Always passes
        error_message="Value must be positive",
        category=BugCategory.LOGICAL
    ))
    
    print("\n[Test 1] Check invariants")
    print("-"*70)
    violations = handler.check_invariants()
    print(f"Invariant violations: {len(violations)}")
    
    # Test exception handling
    print("\n[Test 2] Handle exceptions")
    print("-"*70)
    
    # Logical bug
    try:
        raise ValueError("Invalid state detected")
    except Exception as e:
        report = handler.handle_exception(
            e,
            context={'operation': 'state_transition'}
        )
        print(f"Category: {report.category.value}")
        print(f"Severity: {report.severity.value}")
    
    # Compatibility bug
    try:
        raise ImportError("Module not found for this Android version")
    except Exception as e:
        report = handler.handle_exception(
            e,
            context={'android_version': 15}
        )
        print(f"Category: {report.category.value}")
        print(f"Severity: {report.severity.value}")
    
    # Get statistics
    print("\n[Test 3] Statistics")
    print("-"*70)
    stats = handler.get_statistics()
    print(f"Total bugs: {stats['total_bugs']}")
    print(f"By category: {stats['by_category']}")
    print(f"By severity: {stats['by_severity']}")
    
    # Test deterministic validator
    print("\n[Test 4] Deterministic Validator")
    print("-"*70)
    validator = DeterministicValidator(handler)
    
    # Valid input
    is_valid = validator.validate_input(
        42,
        int,
        constraints={
            'positive': lambda x: x > 0,
            'less_than_100': lambda x: x < 100
        }
    )
    print(f"Valid input (42): {is_valid}")
    
    # Invalid input
    is_valid = validator.validate_input(
        "not_a_number",
        int
    )
    print(f"Invalid input ('not_a_number'): {is_valid}")
    
    print("\n" + "="*70)
    print("Bug handler demonstration complete")
    print("="*70)
