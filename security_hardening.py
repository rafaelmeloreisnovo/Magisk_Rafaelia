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
SECURITY_HARDENING.PY - Security Hardening and Vulnerability Mitigation

Comprehensive security toolkit implementing OWASP and NIST best practices for
application security, vulnerability scanning, and Zero Trust architecture.

CAPABILITIES:
- Input validation utilities (OWASP-compliant)
- Secure coding patterns library
- Zero Trust architecture validator (NIST SP 800-207)
- Vulnerability scanning (credentials, SQL injection)
- Path traversal protection (URL-decoded)
- Security best practices guide

INPUT VALIDATION:
- String sanitization (control character removal, length limits)
- Path validation (directory traversal prevention with URL decode)
- Email format validation (RFC-compliant)
- Alphanumeric validation (with/without spaces)

SECURE CODING PATTERNS:
- Safe file I/O (size limits, atomic writes with backup)
- Cryptographically secure random generation
- Secure hashing (SHA3-256)
- Atomic file operations

ZERO TRUST VALIDATOR:
- Authorization policy framework
- Context-aware access control
- Continuous authentication and authorization
- Comprehensive audit logging

VULNERABILITY SCANNING:
- Hardcoded credential detection (passwords, API keys, tokens)
- SQL injection pattern detection (string concatenation)
- File size limits (5MB/10MB) to prevent DoS
- Pattern-based security analysis

Part of Magisk_Rafaelia Governance Framework
ZIPRAF_OMEGA v999 Security Module
Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
Philosophy: VAZIO → VERBO → CHEIO → RETRO
Motto: Amor, Luz e Coerência

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved.

DUAL LICENSE - Choose one:

1. SOCIAL INCLUSION LICENSE (Free):
   Free for educational, research, non-profit, and social inclusion purposes.
   Must include attribution. No commercial use.

2. COMMERCIAL SAAS LICENSE (Paid Subscription):
   Required for any commercial use, SaaS, or revenue-generating purposes.
   Contact rafaelmeloreisnovo for commercial licensing.

AUTOMATIC PENALTIES: Unauthorized commercial use subject to automatic penalties
of minimum R$ 50,000 (BRL) or USD $10,000 per violation plus 5% of gross revenue.

See LICENSE and RAFAELIA_LICENSE.md for complete terms.

AUTHOR BIOGRAPHY:
Rafael Melo Reis (rafaelmeloreisnovo)
Creator of RAFAELIA Framework and ZIPRAF_OMEGA Governance System
Instituto Rafael - CientiEspiritual Philosophy
ESTADO FRACTAL HAJA Framework

REFERENCES:
- OWASP Top 10: https://owasp.org/www-project-top-ten/
- NIST SP 800-53: Security and Privacy Controls
- NIST SP 800-207: Zero Trust Architecture
- CWE Top 25: https://cwe.mitre.org/top25/
- CERT Secure Coding Standards
- ISO/IEC 27002: Code of Practice for Information Security Controls

USAGE:
    ./security_hardening.py scan        # Run vulnerability scan
    ./security_hardening.py guide       # Display security best practices
    ./security_hardening.py validate    # Test Zero Trust validator

VERSION: 999
TIMESTAMP: 2025-11-23
STATUS: OPERATIONAL
"""

import os
import re
import sys
import logging
from pathlib import Path
from typing import List, Dict, Any, Optional, Literal, Protocol
from dataclasses import dataclass
from functools import lru_cache


# ============================================================================
# CUSTOM EXCEPTIONS
# ============================================================================

class SecurityError(Exception):
    """Base exception for security-related errors"""
    pass


class ValidationError(SecurityError):
    """Raised when input validation fails"""
    pass


class PathTraversalError(SecurityError):
    """Raised when path traversal is detected"""
    pass


class VulnerabilityDetectedError(SecurityError):
    """Raised when a vulnerability is detected"""
    pass


# ============================================================================
# TYPE ALIASES
# ============================================================================

VulnerabilityType = Literal[
    "password",
    "api_key",
    "secret",
    "token",
    "aws_key",
    "private_key",
    "sql_injection"
]


# ============================================================================
# CONFIGURATION
# ============================================================================

# File size limits (in bytes)
MAX_FILE_SIZE_SCAN = 5 * 1024 * 1024  # 5MB for vulnerability scanning
MAX_FILE_SIZE_READ = 10 * 1024 * 1024  # 10MB for general file reading

logger = logging.getLogger('security_hardening')
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s'
)


# ============================================================================
# INPUT VALIDATION UTILITIES
# ============================================================================

class InputValidator:
    """
    Secure input validation utilities
    Implements OWASP guidelines and NIST 800-53 controls
    """
    
    @staticmethod
    def sanitize_string(input_str: str, max_length: int = 1000) -> str:
        """
        Sanitize string input to prevent injection attacks
        
        Args:
            input_str: Input string to sanitize
            max_length: Maximum allowed length
            
        Returns:
            Sanitized string
        """
        if not isinstance(input_str, str):
            raise ValueError("Input must be a string")
        
        # Truncate to max length
        input_str = input_str[:max_length]
        
        # Remove null bytes
        input_str = input_str.replace('\x00', '')
        
        # Remove control characters (except newline, tab)
        input_str = ''.join(char for char in input_str 
                           if ord(char) >= 32 or char in '\n\t')
        
        return input_str
    
    @staticmethod
    def validate_path(path_str: str, base_dir: Optional[Path] = None) -> Path:
        """
        Validate file path to prevent directory traversal attacks
        
        Args:
            path_str: Path string to validate
            base_dir: Base directory to restrict access
            
        Returns:
            Validated Path object
            
        Raises:
            ValueError: If path is invalid or attempts traversal
        """
        # Normalize and resolve path to handle encoded traversals
        import urllib.parse
        
        # Decode URL encoding to catch encoded traversal attempts
        decoded_path = urllib.parse.unquote(path_str)
        
        # Normalize and resolve to absolute path
        path = Path(os.path.normpath(decoded_path)).resolve()
        
        # If base_dir provided, ensure path is within it
        # This is the primary security check - resolved path must be within base
        if base_dir:
            base_dir = base_dir.resolve()
            try:
                # This will raise ValueError if path is not relative to base_dir
                path.relative_to(base_dir)
            except ValueError:
                raise ValueError(f"Path must be within {base_dir}")
        
        return path
    
    @staticmethod
    def validate_email(email: str) -> bool:
        """
        Validate email format
        
        Args:
            email: Email string to validate
            
        Returns:
            True if valid email format
        """
        pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        return bool(re.match(pattern, email))
    
    @staticmethod
    def validate_alphanumeric(text: str, allow_spaces: bool = False) -> bool:
        """
        Validate that text contains only alphanumeric characters
        
        Args:
            text: Text to validate
            allow_spaces: Whether to allow spaces
            
        Returns:
            True if valid
        """
        if allow_spaces:
            pattern = r'^[a-zA-Z0-9 ]+$'
        else:
            pattern = r'^[a-zA-Z0-9]+$'
        return bool(re.match(pattern, text))


# ============================================================================
# SECURE CODING PATTERNS
# ============================================================================

class SecureCoding:
    """
    Secure coding pattern implementations
    Follows OWASP Top 10 and CERT Secure Coding Standards
    """
    
    @staticmethod
    def safe_file_read(filepath: Path, max_size: int = MAX_FILE_SIZE_READ) -> str:
        """
        Safely read file with size limit
        
        Args:
            filepath: Path to file
            max_size: Maximum file size in bytes (default 10MB)
            
        Returns:
            File contents
            
        Raises:
            ValueError: If file too large or doesn't exist
        """
        if not filepath.exists():
            raise ValueError(f"File does not exist: {filepath}")
        
        file_size = filepath.stat().st_size
        if file_size > max_size:
            raise ValueError(f"File too large: {file_size} > {max_size} bytes")
        
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            return f.read()
    
    @staticmethod
    def safe_file_write(filepath: Path, content: str, create_backup: bool = True) -> None:
        """
        Safely write file with backup
        
        Args:
            filepath: Path to file
            content: Content to write
            create_backup: Whether to create backup of existing file
        """
        # Create backup if file exists
        if create_backup and filepath.exists():
            backup_path = filepath.with_suffix(filepath.suffix + '.bak')
            import shutil
            shutil.copy2(filepath, backup_path)
        
        # Write atomically using temp file
        temp_path = filepath.with_suffix(filepath.suffix + '.tmp')
        
        try:
            with open(temp_path, 'w', encoding='utf-8') as f:
                f.write(content)
                f.flush()
                os.fsync(f.fileno())
            
            # Atomic rename
            temp_path.replace(filepath)
        except Exception as e:
            # Clean up temp file on error
            if temp_path.exists():
                temp_path.unlink()
            raise e
    
    @staticmethod
    def secure_random_string(length: int = 32) -> str:
        """
        Generate cryptographically secure random string
        
        Args:
            length: Length of random string
            
        Returns:
            Random string
        """
        import secrets
        import string
        
        alphabet = string.ascii_letters + string.digits
        return ''.join(secrets.choice(alphabet) for _ in range(length))
    
    @staticmethod
    def hash_sensitive_data(data: str) -> str:
        """
        Hash sensitive data using secure algorithm
        
        Args:
            data: Data to hash
            
        Returns:
            Hex digest of hash
        """
        import hashlib
        return hashlib.sha3_256(data.encode()).hexdigest()


# ============================================================================
# ZERO TRUST ARCHITECTURE
# ============================================================================

@dataclass
class AuthorizationPolicy:
    """Zero Trust authorization policy"""
    resource: str
    action: str
    allowed_principals: List[str]
    conditions: Dict[str, Any]


class ZeroTrustValidator:
    """
    Zero Trust Architecture validation
    Implements NIST SP 800-207 principles
    """
    
    def __init__(self):
        self.policies: List[AuthorizationPolicy] = []
    
    def add_policy(self, policy: AuthorizationPolicy) -> None:
        """Add authorization policy"""
        self.policies.append(policy)
    
    def validate_access(
        self,
        principal: str,
        resource: str,
        action: str,
        context: Dict[str, Any]
    ) -> bool:
        """
        Validate access request (Never Trust, Always Verify)
        
        Args:
            principal: Identity requesting access
            resource: Resource being accessed
            action: Action being performed
            context: Additional context for decision
            
        Returns:
            True if access allowed
        """
        # Find matching policies
        matching_policies = [
            p for p in self.policies
            if p.resource == resource and p.action == action
        ]
        
        if not matching_policies:
            # Default deny
            logger.warning(f"No policy found for {resource}:{action}")
            return False
        
        # Check each policy
        for policy in matching_policies:
            # Check principal
            if principal not in policy.allowed_principals:
                continue
            
            # Check conditions
            conditions_met = all(
                context.get(key) == value
                for key, value in policy.conditions.items()
            )
            
            if conditions_met:
                logger.info(f"Access granted: {principal} -> {resource}:{action}")
                return True
        
        # No policy matched - deny
        logger.warning(f"Access denied: {principal} -> {resource}:{action}")
        return False


# ============================================================================
# VULNERABILITY SCANNING HELPERS
# ============================================================================

class VulnerabilityScanner:
    """Helper utilities for vulnerability scanning"""
    
    def __init__(self, repo_path: Path):
        self.repo_path = repo_path
    
    def scan_for_hardcoded_credentials(self) -> List[Dict[str, Any]]:
        """
        Scan for potential hardcoded credentials
        
        Returns:
            List of findings
        """
        findings = []
        
        # Patterns that might indicate credentials
        patterns = [
            (r'password\s*=\s*["\'][^"\']+["\']', 'password'),
            (r'api[_-]?key\s*=\s*["\'][^"\']+["\']', 'api_key'),
            (r'secret\s*=\s*["\'][^"\']+["\']', 'secret'),
            (r'token\s*=\s*["\'][^"\']+["\']', 'token'),
            (r'aws[_-]?access[_-]?key', 'aws_key'),
            (r'private[_-]?key', 'private_key'),
        ]
        
        for py_file in self.repo_path.rglob("*.py"):
            # Skip virtual environments
            if any(skip in str(py_file) for skip in ['.venv', 'venv', '__pycache__']):
                continue
            
            try:
                # Check file size first to avoid memory exhaustion
                file_size = py_file.stat().st_size
                max_size = MAX_FILE_SIZE_SCAN  # 5MB limit
                
                if file_size > max_size:
                    logger.warning(f"Skipping large file {py_file}: {file_size} bytes")
                    continue
                
                content = py_file.read_text(encoding='utf-8')
                
                for pattern, cred_type in patterns:
                    matches = re.finditer(pattern, content, re.IGNORECASE)
                    for match in matches:
                        # Get line number
                        line_num = content[:match.start()].count('\n') + 1
                        
                        findings.append({
                            'file': str(py_file.relative_to(self.repo_path)),
                            'line': line_num,
                            'type': cred_type,
                            'match': match.group(0)
                        })
            except Exception as e:
                logger.debug(f"Could not scan {py_file}: {e}")
        
        return findings
    
    def scan_for_sql_injection(self) -> List[Dict[str, Any]]:
        """
        Scan for potential SQL injection vulnerabilities
        
        Returns:
            List of findings
        """
        findings = []
        
        # Pattern for string concatenation in SQL queries
        sql_patterns = [
            r'execute\s*\([^)]*\+',
            r'executemany\s*\([^)]*\+',
            r'SELECT.*\+.*FROM',
            r'INSERT.*\+.*VALUES',
            r'UPDATE.*\+.*SET',
            r'DELETE.*\+.*FROM',
        ]
        
        for py_file in self.repo_path.rglob("*.py"):
            if any(skip in str(py_file) for skip in ['.venv', 'venv', '__pycache__']):
                continue
            
            try:
                # Check file size first to avoid memory exhaustion
                file_size = py_file.stat().st_size
                max_size = MAX_FILE_SIZE_SCAN  # 5MB limit
                
                if file_size > max_size:
                    logger.warning(f"Skipping large file {py_file}: {file_size} bytes")
                    continue
                
                content = py_file.read_text(encoding='utf-8')
                
                for pattern in sql_patterns:
                    matches = re.finditer(pattern, content, re.IGNORECASE)
                    for match in matches:
                        line_num = content[:match.start()].count('\n') + 1
                        
                        findings.append({
                            'file': str(py_file.relative_to(self.repo_path)),
                            'line': line_num,
                            'type': 'sql_injection',
                            'match': match.group(0)[:50]  # Truncate
                        })
            except Exception as e:
                logger.debug(f"Could not scan {py_file}: {e}")
        
        return findings
    
    def generate_report(self) -> Dict[str, Any]:
        """
        Generate comprehensive vulnerability report
        
        Returns:
            Report dictionary
        """
        logger.info("Scanning for vulnerabilities...")
        
        credential_findings = self.scan_for_hardcoded_credentials()
        sql_findings = self.scan_for_sql_injection()
        
        report = {
            'timestamp': Path(__file__).stat().st_mtime,
            'repository': str(self.repo_path),
            'findings': {
                'hardcoded_credentials': {
                    'count': len(credential_findings),
                    'items': credential_findings[:10]  # Limit to 10
                },
                'sql_injection': {
                    'count': len(sql_findings),
                    'items': sql_findings[:10]
                }
            },
            'summary': {
                'total_findings': len(credential_findings) + len(sql_findings),
                'critical': len(credential_findings),
                'high': len(sql_findings),
            }
        }
        
        return report


# ============================================================================
# SECURITY BEST PRACTICES
# ============================================================================

def print_security_best_practices():
    """Print security best practices guide"""
    print("""
╔══════════════════════════════════════════════════════════════════════════════╗
║                    SECURITY BEST PRACTICES GUIDE                             ║
║                    NIST SP 800-53 & OWASP Top 10                            ║
╚══════════════════════════════════════════════════════════════════════════════╝

1. INPUT VALIDATION
   ✓ Validate all input from untrusted sources
   ✓ Use whitelisting over blacklisting
   ✓ Sanitize data before processing
   ✓ Enforce length limits

2. AUTHENTICATION & ACCESS CONTROL
   ✓ Implement strong authentication
   ✓ Use principle of least privilege
   ✓ Never trust, always verify (Zero Trust)
   ✓ Log all access attempts

3. CRYPTOGRAPHY
   ✓ Use strong algorithms (AES-256, SHA-3)
   ✓ Never implement custom crypto
   ✓ Protect keys and secrets
   ✓ Use secure random number generation

4. DATA PROTECTION
   ✓ Encrypt sensitive data at rest
   ✓ Encrypt data in transit (TLS 1.3)
   ✓ Minimize data retention
   ✓ Secure data disposal

5. ERROR HANDLING
   ✓ Don't expose sensitive information in errors
   ✓ Log errors securely
   ✓ Fail securely (default deny)
   ✓ Validate error messages

6. SECURE CODING
   ✓ Avoid SQL injection (use parameterized queries)
   ✓ Prevent XSS (escape output)
   ✓ Prevent CSRF (use tokens)
   ✓ Validate file uploads

7. MONITORING & LOGGING
   ✓ Log security events
   ✓ Monitor for anomalies
   ✓ Protect log integrity
   ✓ Regular security audits

8. DEPENDENCY MANAGEMENT
   ✓ Keep dependencies updated
   ✓ Scan for known vulnerabilities
   ✓ Use trusted sources only
   ✓ Verify integrity (checksums)

9. CONFIGURATION
   ✓ Secure defaults
   ✓ Disable unnecessary features
   ✓ Protect configuration files
   ✓ Regular security reviews

10. INCIDENT RESPONSE
    ✓ Have an incident response plan
    ✓ Regular backups
    ✓ Test recovery procedures
    ✓ Document lessons learned

For more information:
- OWASP: https://owasp.org/
- NIST: https://www.nist.gov/cybersecurity
- CERT: https://www.cert.org/secure-coding/

""")


# ============================================================================
# CLI INTERFACE
# ============================================================================

def main():
    """Main entry point"""
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Security Hardening and Vulnerability Mitigation Tool"
    )
    
    parser.add_argument(
        'command',
        choices=['scan', 'guide', 'validate'],
        help='Command to execute'
    )
    
    parser.add_argument(
        '--repo',
        type=Path,
        default=Path.cwd(),
        help='Repository path (default: current directory)'
    )
    
    args = parser.parse_args()
    
    if args.command == 'scan':
        scanner = VulnerabilityScanner(args.repo)
        report = scanner.generate_report()
        
        print("\n" + "=" * 80)
        print("VULNERABILITY SCAN REPORT")
        print("=" * 80)
        print(f"Repository: {report['repository']}")
        print(f"Total Findings: {report['summary']['total_findings']}")
        print(f"  Critical: {report['summary']['critical']}")
        print(f"  High: {report['summary']['high']}")
        print("")
        
        if report['findings']['hardcoded_credentials']['count'] > 0:
            print("⚠️  Hardcoded Credentials:")
            for finding in report['findings']['hardcoded_credentials']['items']:
                print(f"  {finding['file']}:{finding['line']} - {finding['type']}")
        
        if report['findings']['sql_injection']['count'] > 0:
            print("\n⚠️  Potential SQL Injection:")
            for finding in report['findings']['sql_injection']['items']:
                print(f"  {finding['file']}:{finding['line']}")
        
        print("=" * 80)
        
        return 0 if report['summary']['total_findings'] == 0 else 1
    
    elif args.command == 'guide':
        print_security_best_practices()
        return 0
    
    elif args.command == 'validate':
        # Example Zero Trust validation
        validator = ZeroTrustValidator()
        
        # Add example policy
        policy = AuthorizationPolicy(
            resource="sensitive_data",
            action="read",
            allowed_principals=["admin", "auditor"],
            conditions={"authenticated": True, "mfa_enabled": True}
        )
        validator.add_policy(policy)
        
        # Test access
        context = {"authenticated": True, "mfa_enabled": True}
        result = validator.validate_access("admin", "sensitive_data", "read", context)
        
        print(f"Access validation result: {'✓ ALLOWED' if result else '✗ DENIED'}")
        return 0 if result else 1


if __name__ == "__main__":
    sys.exit(main())
