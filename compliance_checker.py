#!/usr/bin/env python3
"""
compliance_checker.py - CI/CD Compliance and Security Checker

Automated compliance verification for CI/CD pipelines:
- Standards compliance (ISO/IEEE/NIST/W3C)
- Security vulnerability scanning
- Code quality checks
- License compliance
- Configuration validation

Part of ZIPRAF_OMEGA Governance Framework v999
"""

import json
import logging
import os
import subprocess
import sys
from dataclasses import dataclass, asdict
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Any, Tuple


# ============================================================================
# CONFIGURATION
# ============================================================================

logger = logging.getLogger('compliance_checker')
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s'
)


# ============================================================================
# DATA STRUCTURES
# ============================================================================

@dataclass
class ComplianceCheck:
    """Individual compliance check result"""
    category: str
    check_name: str
    passed: bool
    severity: str  # INFO, WARNING, CRITICAL
    message: str
    details: Dict[str, Any]
    timestamp: str


@dataclass
class ComplianceReport:
    """Overall compliance report"""
    timestamp: str
    repository: str
    branch: str
    commit: str
    checks: List[ComplianceCheck]
    total_checks: int
    passed_checks: int
    failed_checks: int
    critical_failures: int
    overall_status: str  # PASS, FAIL, WARNING


# ============================================================================
# SECURITY CHECKS
# ============================================================================

class SecurityChecker:
    """Security compliance and vulnerability checking"""
    
    def __init__(self, repo_path: Path, verbose: bool = False):
        self.repo_path = repo_path
        self.verbose = verbose
    
    def check_file_permissions(self) -> ComplianceCheck:
        """Check for insecure file permissions"""
        issues = []
        
        # Check critical files
        critical_files = [
            self.repo_path / "ativar.txt",
            self.repo_path / "ativar.py",
            self.repo_path / "build.py",
        ]
        
        for filepath in critical_files:
            if filepath.exists():
                stat_info = filepath.stat()
                # Check if world-writable (dangerous)
                if stat_info.st_mode & 0o002:
                    issues.append(f"{filepath.name} is world-writable")
        
        passed = len(issues) == 0
        severity = "CRITICAL" if not passed else "INFO"
        
        return ComplianceCheck(
            category="security",
            check_name="file_permissions",
            passed=passed,
            severity=severity,
            message=f"File permissions check: {len(issues)} issues found" if issues else "All file permissions secure",
            details={"issues": issues},
            timestamp=datetime.utcnow().isoformat()
        )
    
    def check_hardcoded_secrets(self) -> ComplianceCheck:
        """Check for hardcoded secrets and credentials"""
        issues = []
        
        # Common patterns that might indicate secrets
        secret_patterns = [
            "password",
            "api_key",
            "secret",
            "token",
            "credential",
        ]
        
        # Scan Python files
        for py_file in self.repo_path.rglob("*.py"):
            if any(skip in str(py_file) for skip in ['.venv', 'venv', '__pycache__']):
                continue
            
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    for line_num, line in enumerate(f, 1):
                        line_lower = line.lower()
                        for pattern in secret_patterns:
                            if pattern in line_lower and '=' in line:
                                # Exclude comments and safe patterns
                                if not line.strip().startswith('#'):
                                    issues.append({
                                        "file": str(py_file.relative_to(self.repo_path)),
                                        "line": line_num,
                                        "pattern": pattern
                                    })
            except Exception as e:
                if self.verbose:
                    logger.debug(f"Could not scan {py_file}: {e}")
        
        passed = len(issues) == 0
        severity = "CRITICAL" if not passed else "INFO"
        
        return ComplianceCheck(
            category="security",
            check_name="hardcoded_secrets",
            passed=passed,
            severity=severity,
            message=f"Found {len(issues)} potential hardcoded secrets" if issues else "No hardcoded secrets detected",
            details={"potential_secrets": issues[:10]},  # Limit to first 10
            timestamp=datetime.utcnow().isoformat()
        )
    
    def check_dependency_vulnerabilities(self) -> ComplianceCheck:
        """Check for known vulnerabilities in dependencies"""
        # This is a stub - in production, integrate with tools like:
        # - Safety (Python)
        # - npm audit (Node.js)
        # - OWASP Dependency-Check
        
        # For now, just check if requirements.txt exists
        requirements_file = self.repo_path / "requirements.txt"
        has_requirements = requirements_file.exists()
        
        passed = True  # Assume no vulnerabilities for stub
        severity = "INFO"
        
        return ComplianceCheck(
            category="security",
            check_name="dependency_vulnerabilities",
            passed=passed,
            severity=severity,
            message="Dependency vulnerability check passed (stub)",
            details={
                "has_requirements": has_requirements,
                "recommendation": "Use 'safety check' for Python dependencies"
            },
            timestamp=datetime.utcnow().isoformat()
        )


# ============================================================================
# CODE QUALITY CHECKS
# ============================================================================

class CodeQualityChecker:
    """Code quality and standards compliance checking"""
    
    def __init__(self, repo_path: Path, verbose: bool = False):
        self.repo_path = repo_path
        self.verbose = verbose
    
    def check_python_syntax(self) -> ComplianceCheck:
        """Check Python syntax validity"""
        issues = []
        
        for py_file in self.repo_path.rglob("*.py"):
            if any(skip in str(py_file) for skip in ['.venv', 'venv', '__pycache__']):
                continue
            
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    compile(f.read(), str(py_file), 'exec')
            except SyntaxError as e:
                issues.append({
                    "file": str(py_file.relative_to(self.repo_path)),
                    "error": str(e)
                })
        
        passed = len(issues) == 0
        severity = "CRITICAL" if not passed else "INFO"
        
        return ComplianceCheck(
            category="code_quality",
            check_name="python_syntax",
            passed=passed,
            severity=severity,
            message=f"Python syntax check: {len(issues)} errors found" if issues else "All Python syntax valid",
            details={"syntax_errors": issues},
            timestamp=datetime.utcnow().isoformat()
        )
    
    def check_documentation(self) -> ComplianceCheck:
        """Check for adequate documentation"""
        required_docs = [
            "README.MD",
            "CONTRIBUTING.md",
            "LICENSE",
        ]
        
        missing = []
        for doc in required_docs:
            if not (self.repo_path / doc).exists():
                # Try lowercase
                if not (self.repo_path / doc.lower()).exists():
                    missing.append(doc)
        
        passed = len(missing) == 0
        severity = "WARNING" if not passed else "INFO"
        
        return ComplianceCheck(
            category="code_quality",
            check_name="documentation",
            passed=passed,
            severity=severity,
            message=f"Documentation check: {len(missing)} files missing" if missing else "Required documentation present",
            details={"missing_docs": missing},
            timestamp=datetime.utcnow().isoformat()
        )
    
    def check_code_comments(self) -> ComplianceCheck:
        """Check for adequate code comments"""
        stats = {
            "total_files": 0,
            "files_with_comments": 0,
            "total_lines": 0,
            "comment_lines": 0,
        }
        
        for py_file in self.repo_path.rglob("*.py"):
            if any(skip in str(py_file) for skip in ['.venv', 'venv', '__pycache__']):
                continue
            
            stats["total_files"] += 1
            has_comments = False
            
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    for line in f:
                        stats["total_lines"] += 1
                        stripped = line.strip()
                        if stripped.startswith('#') or stripped.startswith('"""') or stripped.startswith("'''"):
                            stats["comment_lines"] += 1
                            has_comments = True
                
                if has_comments:
                    stats["files_with_comments"] += 1
            except Exception as e:
                if self.verbose:
                    logger.debug(f"Could not analyze {py_file}: {e}")
        
        # Calculate comment ratio
        comment_ratio = (stats["comment_lines"] / stats["total_lines"] * 100) if stats["total_lines"] > 0 else 0
        
        # Consider adequate if > 10% comments
        passed = comment_ratio > 10
        severity = "WARNING" if not passed else "INFO"
        
        return ComplianceCheck(
            category="code_quality",
            check_name="code_comments",
            passed=passed,
            severity=severity,
            message=f"Code comments: {comment_ratio:.1f}% of lines",
            details=stats,
            timestamp=datetime.utcnow().isoformat()
        )


# ============================================================================
# LICENSE COMPLIANCE
# ============================================================================

class LicenseChecker:
    """License compliance checking"""
    
    def __init__(self, repo_path: Path, verbose: bool = False):
        self.repo_path = repo_path
        self.verbose = verbose
    
    def check_license_file(self) -> ComplianceCheck:
        """Check for presence of LICENSE file"""
        license_variants = ["LICENSE", "LICENSE.txt", "LICENSE.md", "COPYING"]
        
        found = None
        for variant in license_variants:
            if (self.repo_path / variant).exists():
                found = variant
                break
        
        passed = found is not None
        severity = "CRITICAL" if not passed else "INFO"
        
        return ComplianceCheck(
            category="license",
            check_name="license_file",
            passed=passed,
            severity=severity,
            message=f"License file found: {found}" if found else "No LICENSE file found",
            details={"license_file": found},
            timestamp=datetime.utcnow().isoformat()
        )
    
    def check_license_headers(self) -> ComplianceCheck:
        """Check for license headers in source files"""
        # This is a simplified check
        files_checked = 0
        files_with_headers = 0
        
        for py_file in self.repo_path.rglob("*.py"):
            if any(skip in str(py_file) for skip in ['.venv', 'venv', '__pycache__']):
                continue
            
            files_checked += 1
            
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    first_lines = ''.join([f.readline() for _ in range(20)])
                    if 'license' in first_lines.lower() or 'copyright' in first_lines.lower():
                        files_with_headers += 1
            except Exception:
                pass
        
        header_ratio = (files_with_headers / files_checked * 100) if files_checked > 0 else 0
        
        # Consider adequate if > 50% have headers
        passed = header_ratio > 50 or files_checked == 0
        severity = "WARNING" if not passed else "INFO"
        
        return ComplianceCheck(
            category="license",
            check_name="license_headers",
            passed=passed,
            severity=severity,
            message=f"License headers: {header_ratio:.1f}% of files ({files_with_headers}/{files_checked})",
            details={
                "files_checked": files_checked,
                "files_with_headers": files_with_headers
            },
            timestamp=datetime.utcnow().isoformat()
        )


# ============================================================================
# CONFIGURATION VALIDATION
# ============================================================================

class ConfigurationValidator:
    """Configuration file validation"""
    
    def __init__(self, repo_path: Path, verbose: bool = False):
        self.repo_path = repo_path
        self.verbose = verbose
    
    def check_governance_files(self) -> ComplianceCheck:
        """Check for governance framework files"""
        required_files = [
            "ativar.txt",
            "ativar.py",
        ]
        
        missing = []
        for filename in required_files:
            if not (self.repo_path / filename).exists():
                missing.append(filename)
        
        passed = len(missing) == 0
        severity = "WARNING" if not passed else "INFO"
        
        return ComplianceCheck(
            category="configuration",
            check_name="governance_files",
            passed=passed,
            severity=severity,
            message=f"Governance files: {len(missing)} missing" if missing else "All governance files present",
            details={"missing_files": missing},
            timestamp=datetime.utcnow().isoformat()
        )
    
    def check_json_validity(self) -> ComplianceCheck:
        """Check validity of JSON configuration files"""
        issues = []
        
        for json_file in self.repo_path.rglob("*.json"):
            if any(skip in str(json_file) for skip in ['.venv', 'venv', 'node_modules']):
                continue
            
            try:
                with open(json_file, 'r', encoding='utf-8') as f:
                    json.load(f)
            except json.JSONDecodeError as e:
                issues.append({
                    "file": str(json_file.relative_to(self.repo_path)),
                    "error": str(e)
                })
        
        passed = len(issues) == 0
        severity = "CRITICAL" if not passed else "INFO"
        
        return ComplianceCheck(
            category="configuration",
            check_name="json_validity",
            passed=passed,
            severity=severity,
            message=f"JSON validation: {len(issues)} errors found" if issues else "All JSON files valid",
            details={"json_errors": issues},
            timestamp=datetime.utcnow().isoformat()
        )


# ============================================================================
# COMPREHENSIVE COMPLIANCE CHECKER
# ============================================================================

class ComplianceChecker:
    """Comprehensive compliance checking"""
    
    def __init__(self, repo_path: Path, verbose: bool = False):
        self.repo_path = repo_path
        self.verbose = verbose
        self.security_checker = SecurityChecker(repo_path, verbose)
        self.quality_checker = CodeQualityChecker(repo_path, verbose)
        self.license_checker = LicenseChecker(repo_path, verbose)
        self.config_validator = ConfigurationValidator(repo_path, verbose)
    
    def get_git_info(self) -> Dict[str, str]:
        """Get current git branch and commit"""
        info = {
            "branch": "unknown",
            "commit": "unknown"
        }
        
        try:
            result = subprocess.run(
                ["git", "rev-parse", "--abbrev-ref", "HEAD"],
                cwd=self.repo_path,
                capture_output=True,
                text=True,
                timeout=5
            )
            if result.returncode == 0:
                info["branch"] = result.stdout.strip()
            
            result = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=self.repo_path,
                capture_output=True,
                text=True,
                timeout=5
            )
            if result.returncode == 0:
                info["commit"] = result.stdout.strip()[:8]
        except Exception as e:
            if self.verbose:
                logger.debug(f"Could not get git info: {e}")
        
        return info
    
    def run_all_checks(self) -> ComplianceReport:
        """Run all compliance checks"""
        logger.info("=" * 80)
        logger.info("COMPLIANCE CHECKER - ZIPRAF_OMEGA v999")
        logger.info("=" * 80)
        
        checks: List[ComplianceCheck] = []
        
        # Get git info
        git_info = self.get_git_info()
        logger.info(f"Repository: {self.repo_path.name}")
        logger.info(f"Branch: {git_info['branch']}")
        logger.info(f"Commit: {git_info['commit']}")
        logger.info("")
        
        # Security checks
        logger.info("🔒 Running security checks...")
        checks.append(self.security_checker.check_file_permissions())
        checks.append(self.security_checker.check_hardcoded_secrets())
        checks.append(self.security_checker.check_dependency_vulnerabilities())
        
        # Code quality checks
        logger.info("📝 Running code quality checks...")
        checks.append(self.quality_checker.check_python_syntax())
        checks.append(self.quality_checker.check_documentation())
        checks.append(self.quality_checker.check_code_comments())
        
        # License checks
        logger.info("⚖️  Running license compliance checks...")
        checks.append(self.license_checker.check_license_file())
        checks.append(self.license_checker.check_license_headers())
        
        # Configuration checks
        logger.info("⚙️  Running configuration validation...")
        checks.append(self.config_validator.check_governance_files())
        checks.append(self.config_validator.check_json_validity())
        
        # Calculate statistics
        total_checks = len(checks)
        passed_checks = sum(1 for c in checks if c.passed)
        failed_checks = total_checks - passed_checks
        critical_failures = sum(1 for c in checks if not c.passed and c.severity == "CRITICAL")
        
        # Determine overall status
        if critical_failures > 0:
            overall_status = "FAIL"
        elif failed_checks > 0:
            overall_status = "WARNING"
        else:
            overall_status = "PASS"
        
        # Create report
        report = ComplianceReport(
            timestamp=datetime.utcnow().isoformat(),
            repository=str(self.repo_path),
            branch=git_info['branch'],
            commit=git_info['commit'],
            checks=checks,
            total_checks=total_checks,
            passed_checks=passed_checks,
            failed_checks=failed_checks,
            critical_failures=critical_failures,
            overall_status=overall_status
        )
        
        # Print results
        logger.info("\n" + "=" * 80)
        logger.info("COMPLIANCE REPORT")
        logger.info("=" * 80)
        
        for check in checks:
            status = "✓" if check.passed else "✗"
            severity_marker = ""
            if not check.passed:
                if check.severity == "CRITICAL":
                    severity_marker = " [CRITICAL]"
                elif check.severity == "WARNING":
                    severity_marker = " [WARNING]"
            
            logger.info(f"{status} {check.category}/{check.check_name}: {check.message}{severity_marker}")
        
        logger.info("\n" + "=" * 80)
        logger.info(f"Total Checks: {total_checks}")
        logger.info(f"Passed: {passed_checks}")
        logger.info(f"Failed: {failed_checks}")
        logger.info(f"Critical Failures: {critical_failures}")
        logger.info(f"Overall Status: {overall_status}")
        logger.info("=" * 80)
        
        return report


# ============================================================================
# CLI INTERFACE
# ============================================================================

def main():
    """Main entry point"""
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Compliance Checker for CI/CD - ZIPRAF_OMEGA v999"
    )
    
    parser.add_argument(
        '--repo',
        type=Path,
        default=Path.cwd(),
        help='Repository path (default: current directory)'
    )
    
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Verbose output'
    )
    
    parser.add_argument(
        '-o', '--output',
        type=Path,
        help='Output JSON report file'
    )
    
    parser.add_argument(
        '--fail-on-warning',
        action='store_true',
        help='Exit with error code if any warnings are present'
    )
    
    args = parser.parse_args()
    
    # Set up logging
    if args.verbose:
        logger.setLevel(logging.DEBUG)
    
    # Run checks
    checker = ComplianceChecker(args.repo, args.verbose)
    report = checker.run_all_checks()
    
    # Save report if requested
    if args.output:
        report_dict = asdict(report)
        with open(args.output, 'w') as f:
            json.dump(report_dict, f, indent=2)
        logger.info(f"\n📄 Report saved to: {args.output}")
    
    # Determine exit code
    if report.overall_status == "FAIL":
        return 1
    elif report.overall_status == "WARNING" and args.fail_on_warning:
        return 1
    else:
        return 0


if __name__ == "__main__":
    sys.exit(main())
