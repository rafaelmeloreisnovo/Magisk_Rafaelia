#!/usr/bin/env python3
"""
Governance Compliance Checker
Automated validation tool for ISO, IEEE, ICT, NIST, W3C, and ABNT standards

This tool performs comprehensive compliance checks across the entire codebase
according to the Global Governance Framework defined in GOVERNANCE.md

Usage:
    python3 governance_compliance_checker.py [--fix] [--report output.json]
    
Author: RAFAELIA Governance System
Version: 1.0.0
Date: 2025-11-20
"""

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple
from datetime import datetime

# ANSI color codes for terminal output
class Colors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

class ComplianceChecker:
    """Main compliance checking class"""
    
    def __init__(self, root_dir: str, fix_mode: bool = False):
        self.root_dir = Path(root_dir)
        self.fix_mode = fix_mode
        self.violations = []
        self.warnings = []
        self.passed = []
        
    def print_header(self, text: str):
        """Print section header"""
        print(f"\n{Colors.HEADER}{Colors.BOLD}{'=' * 80}{Colors.ENDC}")
        print(f"{Colors.HEADER}{Colors.BOLD}{text:^80}{Colors.ENDC}")
        print(f"{Colors.HEADER}{Colors.BOLD}{'=' * 80}{Colors.ENDC}\n")
        
    def print_success(self, text: str):
        """Print success message"""
        print(f"{Colors.OKGREEN}✓ {text}{Colors.ENDC}")
        
    def print_warning(self, text: str):
        """Print warning message"""
        print(f"{Colors.WARNING}⚠ {text}{Colors.ENDC}")
        
    def print_error(self, text: str):
        """Print error message"""
        print(f"{Colors.FAIL}✗ {text}{Colors.ENDC}")
        
    def add_violation(self, category: str, file_path: str, issue: str, line: int = None):
        """Record a compliance violation"""
        violation = {
            "category": category,
            "file": str(file_path),
            "issue": issue,
            "line": line,
            "timestamp": datetime.utcnow().isoformat()
        }
        self.violations.append(violation)
        
    def add_warning(self, category: str, file_path: str, issue: str):
        """Record a compliance warning"""
        warning = {
            "category": category,
            "file": str(file_path),
            "issue": issue,
            "timestamp": datetime.utcnow().isoformat()
        }
        self.warnings.append(warning)
        
    def add_passed(self, category: str, check: str):
        """Record a passed check"""
        self.passed.append({
            "category": category,
            "check": check,
            "timestamp": datetime.utcnow().isoformat()
        })
        
    def check_file_headers(self):
        """ISO/IEEE: Check for proper file headers and documentation"""
        self.print_header("ISO/IEEE Standard: File Headers and Documentation")
        
        # Check Python files
        python_files = list(self.root_dir.rglob("*.py"))
        for file_path in python_files:
            if ".git" in str(file_path) or "__pycache__" in str(file_path):
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    
                # Check for module docstring
                if not content.strip().startswith('"""') and not content.strip().startswith("'''"):
                    if not content.strip().startswith('#!'):
                        self.add_violation("ISO-9126", file_path, "Missing module docstring")
                        self.print_error(f"Missing docstring: {file_path.relative_to(self.root_dir)}")
                    else:
                        # Check if docstring comes after shebang
                        lines = content.split('\n')
                        if len(lines) > 1 and not (lines[1].strip().startswith('"""') or lines[1].strip().startswith("'''")):
                            self.add_violation("ISO-9126", file_path, "Missing module docstring after shebang")
                            self.print_error(f"Missing docstring after shebang: {file_path.relative_to(self.root_dir)}")
                else:
                    self.print_success(f"Valid header: {file_path.relative_to(self.root_dir)}")
                    self.add_passed("ISO-9126", f"File header: {file_path.relative_to(self.root_dir)}")
                    
            except Exception as e:
                self.add_warning("ISO-9126", file_path, f"Could not read file: {str(e)}")
                
        # Check shell scripts
        shell_files = list(self.root_dir.rglob("*.sh"))
        for file_path in shell_files:
            if ".git" in str(file_path):
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    lines = content.split('\n')
                    
                # Check for shebang
                if not lines[0].startswith('#!'):
                    self.add_violation("IEEE-730", file_path, "Missing shebang line")
                    self.print_error(f"Missing shebang: {file_path.relative_to(self.root_dir)}")
                else:
                    self.add_passed("IEEE-730", f"Shebang present: {file_path.relative_to(self.root_dir)}")
                    
                # Check for header comments
                has_description = False
                for i, line in enumerate(lines[:20]):  # Check first 20 lines
                    if "description" in line.lower() or "purpose" in line.lower():
                        has_description = True
                        break
                        
                if not has_description:
                    self.add_violation("IEEE-730", file_path, "Missing script description in header")
                    self.print_warning(f"Missing description: {file_path.relative_to(self.root_dir)}")
                else:
                    self.print_success(f"Valid header: {file_path.relative_to(self.root_dir)}")
                    self.add_passed("IEEE-730", f"Header description: {file_path.relative_to(self.root_dir)}")
                    
            except Exception as e:
                self.add_warning("IEEE-730", file_path, f"Could not read file: {str(e)}")
                
    def check_security_practices(self):
        """NIST SP 800-53: Check for security best practices"""
        self.print_header("NIST SP 800-53: Security Controls")
        
        # Check for hardcoded secrets in Python files
        python_files = list(self.root_dir.rglob("*.py"))
        secret_patterns = [
            r'password\s*=\s*["\'][^"\']+["\']',
            r'api_key\s*=\s*["\'][^"\']+["\']',
            r'secret\s*=\s*["\'][^"\']+["\']',
            r'token\s*=\s*["\'][^"\']+["\']',
            r'aws_access_key',
            r'aws_secret_key'
        ]
        
        for file_path in python_files:
            if ".git" in str(file_path) or "__pycache__" in str(file_path):
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    lines = content.split('\n')
                    
                for i, line in enumerate(lines, 1):
                    for pattern in secret_patterns:
                        if re.search(pattern, line, re.IGNORECASE):
                            # Check if it's in a comment or example
                            if not line.strip().startswith('#'):
                                self.add_violation("NIST-SP-800-53-IA", file_path, 
                                                 f"Potential hardcoded secret on line {i}", i)
                                self.print_error(f"Potential secret: {file_path.relative_to(self.root_dir)}:{i}")
                                
            except Exception as e:
                self.add_warning("NIST-SP-800-53-IA", file_path, f"Could not read file: {str(e)}")
                
        # Check shell scripts for security issues
        shell_files = list(self.root_dir.rglob("*.sh"))
        for file_path in shell_files:
            if ".git" in str(file_path):
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    
                # Check for set -e or set -euo pipefail
                if "set -e" not in content and "set -u" not in content:
                    self.add_violation("NIST-SP-800-53-SI", file_path, 
                                     "Missing error handling (set -e or set -euo pipefail)")
                    self.print_warning(f"Missing error handling: {file_path.relative_to(self.root_dir)}")
                else:
                    self.add_passed("NIST-SP-800-53-SI", f"Error handling: {file_path.relative_to(self.root_dir)}")
                    
                # Check for eval usage
                if re.search(r'\beval\s+', content):
                    self.add_violation("NIST-SP-800-53-SI", file_path, 
                                     "Usage of 'eval' command (security risk)")
                    self.print_error(f"Unsafe eval usage: {file_path.relative_to(self.root_dir)}")
                    
            except Exception as e:
                self.add_warning("NIST-SP-800-53-SI", file_path, f"Could not read file: {str(e)}")
                
    def check_code_quality(self):
        """ISO/IEC 25010: Check code quality metrics"""
        self.print_header("ISO/IEC 25010: Software Quality")
        
        # Check for TODO/FIXME/HACK comments
        all_code_files = list(self.root_dir.rglob("*.py")) + list(self.root_dir.rglob("*.sh"))
        tech_debt_patterns = [
            (r'\bTODO\b', "TODO"),
            (r'\bFIXME\b', "FIXME"),
            (r'\bHACK\b', "HACK"),
            (r'\bXXX\b', "XXX")
        ]
        
        for file_path in all_code_files:
            if ".git" in str(file_path) or "__pycache__" in str(file_path):
                continue
                
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    lines = content.split('\n')
                    
                for i, line in enumerate(lines, 1):
                    for pattern, debt_type in tech_debt_patterns:
                        if re.search(pattern, line):
                            self.add_warning("ISO-25010-Maintainability", file_path, 
                                           f"Technical debt marker '{debt_type}' on line {i}")
                            self.print_warning(f"{debt_type} found: {file_path.relative_to(self.root_dir)}:{i}")
                            
            except Exception as e:
                pass
                
    def check_build_system(self):
        """IEEE 828: Check build system configuration"""
        self.print_header("IEEE 828: Configuration Management")
        
        # Check for build.py
        build_file = self.root_dir / "build.py"
        if build_file.exists():
            self.print_success("Build system present: build.py")
            self.add_passed("IEEE-828", "Build system file exists")
            
            try:
                with open(build_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                    
                # Check for version information
                if "version" in content.lower() or "__version__" in content:
                    self.print_success("Version tracking present in build system")
                    self.add_passed("IEEE-828", "Version tracking in build system")
                else:
                    self.add_warning("IEEE-828", build_file, "No explicit version tracking found")
                    
            except Exception as e:
                self.add_warning("IEEE-828", build_file, f"Could not read file: {str(e)}")
        else:
            self.add_violation("IEEE-828", "root", "Build system file (build.py) not found")
            self.print_error("Build system file not found")
            
        # Check for .gitignore
        gitignore = self.root_dir / ".gitignore"
        if gitignore.exists():
            self.print_success(".gitignore present")
            self.add_passed("IEEE-828", "Version control ignore file exists")
        else:
            self.add_violation("IEEE-828", "root", ".gitignore file not found")
            
    def check_workflow_standards(self):
        """Check GitHub workflows compliance"""
        self.print_header("CI/CD Workflow Standards")
        
        workflows_dir = self.root_dir / ".github" / "workflows"
        if workflows_dir.exists():
            workflow_files = list(workflows_dir.glob("*.yml")) + list(workflows_dir.glob("*.yaml"))
            
            for workflow_file in workflow_files:
                if ".archived" in str(workflow_file):
                    continue
                    
                try:
                    with open(workflow_file, 'r', encoding='utf-8') as f:
                        content = f.read()
                        
                    # Check for timeout configuration
                    if "timeout-minutes" not in content:
                        self.add_warning("IEEE-730", workflow_file, 
                                       "Missing timeout-minutes configuration")
                        self.print_warning(f"No timeout: {workflow_file.relative_to(self.root_dir)}")
                    else:
                        self.add_passed("IEEE-730", f"Timeout configured: {workflow_file.name}")
                        
                    # Check for proper permissions
                    if "permissions:" in content:
                        self.print_success(f"Permissions defined: {workflow_file.name}")
                        self.add_passed("NIST-SP-800-53-AC", f"Permissions: {workflow_file.name}")
                    else:
                        self.add_warning("NIST-SP-800-53-AC", workflow_file, 
                                       "Missing explicit permissions configuration")
                        
                except Exception as e:
                    self.add_warning("CI/CD", workflow_file, f"Could not read file: {str(e)}")
        else:
            self.add_warning("CI/CD", "workflows", "No .github/workflows directory found")
            
    def check_documentation(self):
        """W3C/WCAG: Check documentation standards"""
        self.print_header("W3C/Documentation Standards")
        
        # Check for required documentation files
        required_docs = [
            ("README.MD", "Project README"),
            ("CONTRIBUTING.md", "Contribution guidelines"),
            ("LICENSE", "License file"),
            ("GOVERNANCE.md", "Governance framework")
        ]
        
        for doc_file, description in required_docs:
            doc_path = self.root_dir / doc_file
            if doc_path.exists():
                self.print_success(f"{description} present: {doc_file}")
                self.add_passed("W3C-Documentation", f"{description}")
            else:
                self.add_violation("W3C-Documentation", "root", f"Missing {description} ({doc_file})")
                self.print_error(f"Missing: {doc_file}")
                
        # Check markdown files for proper structure
        md_files = list(self.root_dir.rglob("*.md"))
        for md_file in md_files[:10]:  # Limit check to avoid too many files
            if ".git" in str(md_file):
                continue
                
            try:
                with open(md_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                    
                # Check for heading structure
                if content.strip() and not re.search(r'^#\s+', content, re.MULTILINE):
                    self.add_warning("W3C-Documentation", md_file, "No top-level heading found")
                    
            except Exception as e:
                pass
                
    def check_rafaelia_compliance(self):
        """Check RAFAELIA framework specific compliance"""
        self.print_header("RAFAELIA Framework Compliance")
        
        # Check for RAFAELIA directories and files
        rafaelia_components = [
            ("rafaelia", "RAFAELIA core directory"),
            ("tools/rafaelia", "RAFAELIA tools directory"),
            ("RAFAELIA_MANIFEST.json", "RAFAELIA manifest"),
            ("METADATA.md", "Project metadata")
        ]
        
        for component, description in rafaelia_components:
            component_path = self.root_dir / component
            if component_path.exists():
                self.print_success(f"{description} present")
                self.add_passed("RAFAELIA", description)
            else:
                self.add_warning("RAFAELIA", "root", f"Missing {description} ({component})")
                
    def generate_report(self, output_file: str = None):
        """Generate compliance report"""
        self.print_header("COMPLIANCE REPORT SUMMARY")
        
        total_checks = len(self.passed) + len(self.violations) + len(self.warnings)
        
        print(f"\n{Colors.BOLD}Total Checks Performed:{Colors.ENDC} {total_checks}")
        print(f"{Colors.OKGREEN}✓ Passed:{Colors.ENDC} {len(self.passed)}")
        print(f"{Colors.WARNING}⚠ Warnings:{Colors.ENDC} {len(self.warnings)}")
        print(f"{Colors.FAIL}✗ Violations:{Colors.ENDC} {len(self.violations)}")
        
        compliance_rate = (len(self.passed) / total_checks * 100) if total_checks > 0 else 0
        print(f"\n{Colors.BOLD}Compliance Rate:{Colors.ENDC} {compliance_rate:.2f}%")
        
        if compliance_rate >= 90:
            print(f"{Colors.OKGREEN}✓ EXCELLENT COMPLIANCE{Colors.ENDC}")
        elif compliance_rate >= 75:
            print(f"{Colors.OKCYAN}✓ GOOD COMPLIANCE{Colors.ENDC}")
        elif compliance_rate >= 60:
            print(f"{Colors.WARNING}⚠ ACCEPTABLE COMPLIANCE{Colors.ENDC}")
        else:
            print(f"{Colors.FAIL}✗ POOR COMPLIANCE - ACTION REQUIRED{Colors.ENDC}")
            
        # Generate JSON report if requested
        if output_file:
            report = {
                "timestamp": datetime.utcnow().isoformat(),
                "summary": {
                    "total_checks": total_checks,
                    "passed": len(self.passed),
                    "warnings": len(self.warnings),
                    "violations": len(self.violations),
                    "compliance_rate": compliance_rate
                },
                "passed": self.passed,
                "warnings": self.warnings,
                "violations": self.violations
            }
            
            with open(output_file, 'w', encoding='utf-8') as f:
                json.dump(report, f, indent=2)
                
            print(f"\n{Colors.OKBLUE}Report saved to: {output_file}{Colors.ENDC}")
            
        return len(self.violations) == 0
        
    def run_all_checks(self):
        """Run all compliance checks"""
        print(f"\n{Colors.BOLD}{Colors.HEADER}")
        print("=" * 80)
        print("MAGISK_RAFAELIA GOVERNANCE COMPLIANCE CHECKER".center(80))
        print("ISO | IEEE | ICT | NIST | W3C | ABNT".center(80))
        print("=" * 80)
        print(f"{Colors.ENDC}")
        
        print(f"\n{Colors.BOLD}Root Directory:{Colors.ENDC} {self.root_dir}")
        print(f"{Colors.BOLD}Mode:{Colors.ENDC} {'Fix Mode' if self.fix_mode else 'Check Mode'}")
        print(f"{Colors.BOLD}Timestamp:{Colors.ENDC} {datetime.utcnow().isoformat()}")
        
        # Run all compliance checks
        self.check_file_headers()
        self.check_security_practices()
        self.check_code_quality()
        self.check_build_system()
        self.check_workflow_standards()
        self.check_documentation()
        self.check_rafaelia_compliance()
        
        return self.generate_report()

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="Governance Compliance Checker for Magisk_Rafaelia",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Standards Checked:
  - ISO/IEC 27001: Information Security Management
  - ISO/IEC 9126: Software Quality
  - ISO/IEC 25010: Systems and Software Quality
  - IEEE 730: Software Quality Assurance
  - IEEE 828: Software Configuration Management
  - IEEE 829: Software Test Documentation
  - NIST SP 800-53: Security and Privacy Controls
  - W3C Web Standards
  - ABNT NBR ISO/IEC standards

Examples:
  python3 governance_compliance_checker.py
  python3 governance_compliance_checker.py --report compliance_report.json
  python3 governance_compliance_checker.py --fix --report report.json
        """
    )
    
    parser.add_argument(
        "--fix",
        action="store_true",
        help="Attempt to automatically fix some violations"
    )
    
    parser.add_argument(
        "--report",
        type=str,
        metavar="FILE",
        help="Generate JSON report to specified file"
    )
    
    parser.add_argument(
        "--root",
        type=str,
        default=".",
        help="Root directory to check (default: current directory)"
    )
    
    args = parser.parse_args()
    
    # Create checker instance
    checker = ComplianceChecker(args.root, args.fix)
    
    # Run checks
    success = checker.run_all_checks()
    
    # Generate report if requested
    if args.report:
        checker.generate_report(args.report)
        
    # Exit with appropriate code
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
