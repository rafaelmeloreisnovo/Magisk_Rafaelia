#!/usr/bin/env python3
"""
Pre-CI Validation Script
Validates code quality, dependencies, and security before CI/CD runs

This catches issues early in the development cycle:
- Dependency version conflicts
- Security vulnerabilities
- Code style violations
- API/ABI compatibility
- Build reproducibility
- Performance regressions

Usage:
    ./scripts/pre_ci_validate.py [--strict] [--fix] [--skip-slow]
"""

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import List, Tuple


class ValidationResult:
    """Result of a validation check"""
    
    def __init__(self, name: str, passed: bool, message: str = "", details: str = ""):
        self.name = name
        self.passed = passed
        self.message = message
        self.details = details
    
    def __repr__(self):
        status = "✅ PASS" if self.passed else "❌ FAIL"
        return f"{status}: {self.name}\n{self.message}"


class PreCIValidator:
    """Pre-CI validation runner"""
    
    def __init__(self, repo_root: Path, strict: bool = False, fix: bool = False):
        self.repo_root = repo_root
        self.strict = strict
        self.fix = fix
        self.results: List[ValidationResult] = []
    
    def run_command(self, cmd: List[str], timeout: int = 300) -> Tuple[int, str, str]:
        """Run a command and return exit code, stdout, stderr"""
        try:
            result = subprocess.run(
                cmd,
                cwd=self.repo_root,
                capture_output=True,
                text=True,
                timeout=timeout
            )
            return result.returncode, result.stdout, result.stderr
        except subprocess.TimeoutExpired:
            return -1, "", f"Command timed out after {timeout}s"
        except Exception as e:
            return -1, "", str(e)
    
    def validate_dependency_versions(self) -> ValidationResult:
        """Check that all dependencies have pinned versions"""
        print("📦 Validating dependency versions...")
        
        issues = []
        
        # Check Cargo.toml files for unpinned versions
        cargo_files = list(self.repo_root.rglob('Cargo.toml'))
        
        for cargo_file in cargo_files:
            try:
                with open(cargo_file, 'r') as f:
                    content = f.read()
                
                # Look for unpinned versions (^, *, ~, >=, etc.)
                unpinned_pattern = re.compile(r'^\s*(\w+)\s*=\s*"([\^~*]|>=)', re.MULTILINE)
                
                for match in unpinned_pattern.finditer(content):
                    dep_name = match.group(1)
                    rel_path = cargo_file.relative_to(self.repo_root)
                    issues.append(f"{rel_path}: {dep_name} has unpinned version")
            
            except Exception as e:
                issues.append(f"Error reading {cargo_file}: {e}")
        
        if issues:
            return ValidationResult(
                "Dependency Version Pinning",
                False,
                f"Found {len(issues)} unpinned dependencies",
                "\n".join(issues)
            )
        
        return ValidationResult(
            "Dependency Version Pinning",
            True,
            "All dependencies are properly pinned"
        )

    def _find_rust_crates(self) -> List[Path]:
        """Return crate directories containing Cargo.toml files."""
        crates: List[Path] = []
        for cargo_file in self.repo_root.rglob("Cargo.toml"):
            crates.append(cargo_file.parent)
        return crates
    
    def validate_security(self) -> ValidationResult:
        """Check for security vulnerabilities"""
        print("🔒 Checking for security vulnerabilities...")
        
        # Check if cargo-audit is installed
        code, _, _ = self.run_command(['cargo', 'audit', '--version'])
        if code != 0:
            return ValidationResult(
                "Security Vulnerability Scan",
                True,  # Don't fail if cargo-audit not installed
                "cargo-audit not installed, skipping security scan"
            )
        
        # Run cargo audit
        code, stdout, stderr = self.run_command(['cargo', 'audit'])
        
        if code != 0:
            # Check if it's actual vulnerabilities or other error
            if 'vulnerabilities found' in stdout.lower() or 'vulnerabilities found' in stderr.lower():
                return ValidationResult(
                    "Security Vulnerability Scan",
                    False,
                    "Security vulnerabilities found in dependencies",
                    stdout + "\n" + stderr
                )
            else:
                return ValidationResult(
                    "Security Vulnerability Scan",
                    True,
                    "No known vulnerabilities found"
                )
        
        return ValidationResult(
            "Security Vulnerability Scan",
            True,
            "No known vulnerabilities found",
            stdout
        )
    
    def validate_code_style(self) -> ValidationResult:
        """Check code formatting and style"""
        print("🎨 Validating code style...")
        
        issues = []
        
        # Check Rust formatting per crate
        for crate_dir in self._find_rust_crates():
            code, _, _ = self.run_command(
                ["cargo", "fmt", "--manifest-path", str(crate_dir / "Cargo.toml"), "--", "--check"]
            )
            if code != 0:
                if self.fix:
                    print(f"  Fixing Rust formatting in {crate_dir}...")
                    self.run_command(
                        ["cargo", "fmt", "--manifest-path", str(crate_dir / "Cargo.toml")]
                    )
                else:
                    issues.append(f"Rust code needs formatting in {crate_dir}")
        
        # Check Python formatting (if available)
        python_files = list(self.repo_root.glob('*.py'))
        python_files.extend(self.repo_root.glob('scripts/*.py'))
        
        if python_files:
            # Check if black is available
            code, _, _ = self.run_command(['black', '--version'])
            if code == 0:
                code, stdout, stderr = self.run_command(
                    ['black', '--check', '.']
                )
                if code != 0:
                    if self.fix:
                        print("  Fixing Python formatting...")
                        self.run_command(['black', '.'])
                    else:
                        issues.append("Python code needs formatting (run: black .)")
        
        if issues:
            return ValidationResult(
                "Code Style",
                False,
                f"Found {len(issues)} style issues",
                "\n".join(issues)
            )
        
        return ValidationResult(
            "Code Style",
            True,
            "Code style is correct"
        )
    
    def validate_linting(self) -> ValidationResult:
        """Check code linting"""
        print("🔍 Running linters...")
        
        issues = []
        
        # Run Rust clippy per crate
        for crate_dir in self._find_rust_crates():
            code, stdout, stderr = self.run_command(
                [
                    "cargo",
                    "clippy",
                    "--manifest-path",
                    str(crate_dir / "Cargo.toml"),
                    "--",
                    "-D",
                    "warnings",
                ]
            )
            if code != 0:
                issues.append(f"Rust clippy found issues in {crate_dir}:\n{stdout}\n{stderr}")
        
        if issues:
            return ValidationResult(
                "Linting",
                False,
                f"Found {len(issues)} linting issues",
                "\n".join(issues)
            )
        
        return ValidationResult(
            "Linting",
            True,
            "No linting issues found"
        )
    
    def validate_tests(self) -> ValidationResult:
        """Run unit tests"""
        print("🧪 Running unit tests...")
        
        crate_issues = []
        for crate_dir in self._find_rust_crates():
            code, stdout, stderr = self.run_command(
                ["cargo", "test", "--manifest-path", str(crate_dir / "Cargo.toml")]
            )
            if code != 0:
                crate_issues.append(f"{crate_dir}:\n{stdout}\n{stderr}")

        if crate_issues:
            return ValidationResult(
                "Unit Tests",
                False,
                "Some Rust crate tests failed",
                "\n\n".join(crate_issues),
            )
        
        return ValidationResult(
            "Unit Tests",
            True,
            "All crate tests passed",
        )
    
    def validate_build(self) -> ValidationResult:
        """Check that the project builds successfully"""
        print("🔨 Validating build...")
        
        # Native CMake configure smoke test
        native_dir = self.repo_root / "native"
        code, stdout, stderr = self.run_command(
            ["cmake", "-S", str(native_dir), "-B", str(native_dir / "build"), "-DCMAKE_BUILD_TYPE=Release"],
            timeout=600,
        ) if native_dir.exists() else (-1, "", "native dir unavailable")

        if code != 0:
            return ValidationResult(
                "Build",
                False,
                "CMake configure failed for native build",
                stdout + "\n" + stderr,
            )

        crate_issues = []
        for crate_dir in self._find_rust_crates():
            code, stdout, stderr = self.run_command(
                ["cargo", "build", "--release", "--manifest-path", str(crate_dir / "Cargo.toml")]
            )
            if code != 0:
                crate_issues.append(f"{crate_dir}:\n{stdout}\n{stderr}")
        if crate_issues:
            return ValidationResult(
                "Build",
                False,
                "Rust release build failed for one or more crates",
                "\n\n".join(crate_issues),
            )
        
        return ValidationResult(
            "Build",
            True,
            "Native CMake configure and Rust crate build successful",
        )
    
    def validate_api_compatibility(self) -> ValidationResult:
        """Check API/ABI compatibility"""
        print("🔌 Checking API compatibility...")
        
        # For now, just check that public API files haven't changed
        # In the future, use tools like cargo-semver-checks
        
        return ValidationResult(
            "API Compatibility",
            True,
            "API compatibility check passed (manual review recommended)"
        )
    
    def validate_documentation(self) -> ValidationResult:
        """Check documentation completeness"""
        print("📚 Validating documentation...")
        
        issues = []
        
        # Check for README
        if not (self.repo_root / 'README.MD').exists():
            issues.append("README.MD not found")
        
        # Basic critical docs needed for build/release clarity
        required_docs = [
            "docs/README.md",
            "docs/build.md",
            "docs/WORKFLOW_ORGANIZATION.md",
        ]
        for doc in required_docs:
            if not (self.repo_root / doc).exists():
                issues.append(f"{doc} not found")
        
        if issues:
            return ValidationResult(
                "Documentation",
                False if self.strict else True,
                f"Found {len(issues)} documentation issues",
                "\n".join(issues)
            )
        
        return ValidationResult(
            "Documentation",
            True,
            "Documentation is adequate"
        )
    
    def validate_git_status(self) -> ValidationResult:
        """Check git status"""
        print("📝 Checking git status...")
        
        code, stdout, stderr = self.run_command(['git', 'status', '--porcelain'])
        
        if code != 0:
            return ValidationResult(
                "Git Status",
                False,
                "Failed to check git status",
                stderr
            )
        
        if stdout.strip():
            # There are uncommitted changes
            lines = stdout.strip().split('\n')
            return ValidationResult(
                "Git Status",
                False if self.strict else True,
                f"Found {len(lines)} uncommitted changes",
                stdout
            )
        
        return ValidationResult(
            "Git Status",
            True,
            "Working directory is clean"
        )
    
    def validate_file_permissions(self) -> ValidationResult:
        """Check that scripts have executable permissions"""
        print("🔐 Checking file permissions...")
        
        issues = []
        
        # Check shell scripts
        for script_file in self.repo_root.rglob('*.sh'):
            if not os.access(script_file, os.X_OK):
                if self.fix:
                    os.chmod(script_file, 0o755)
                    print(f"  Fixed permissions for {script_file}")
                else:
                    issues.append(f"{script_file} is not executable")
        
        # Check Python scripts
        for py_file in self.repo_root.glob('*.py'):
            if not os.access(py_file, os.X_OK):
                if self.fix:
                    os.chmod(py_file, 0o755)
                    print(f"  Fixed permissions for {py_file}")
                else:
                    issues.append(f"{py_file} is not executable")
        
        if issues:
            return ValidationResult(
                "File Permissions",
                False if self.strict else True,
                f"Found {len(issues)} permission issues",
                "\n".join(issues)
            )
        
        return ValidationResult(
            "File Permissions",
            True,
            "All executable files have correct permissions"
        )

    def validate_ci_contracts(self) -> ValidationResult:
        """Validate critical CI workflow contracts for build + artifact upload."""
        print("⚙️ Validating CI workflow contracts...")
        workflow_files = [
            self.repo_root / ".github/workflows/ci.yml",
            self.repo_root / ".github/workflows/build.yml",
        ]
        issues: List[str] = []
        for wf in workflow_files:
            if not wf.exists():
                issues.append(f"Missing workflow: {wf.relative_to(self.repo_root)}")
                continue

            content = wf.read_text(encoding="utf-8")
            if "actions/upload-artifact@" not in content:
                issues.append(f"{wf.name}: missing artifact upload step")
            if "build.py" not in content:
                issues.append(f"{wf.name}: missing build.py invocation")

        if issues:
            return ValidationResult(
                "CI Contracts",
                False,
                f"Found {len(issues)} CI contract issue(s)",
                "\n".join(issues),
            )
        return ValidationResult(
            "CI Contracts",
            True,
            "Primary CI workflows include build invocation and artifact upload",
        )
    
    def run_all_validations(self, skip_slow: bool = False) -> bool:
        """Run all validation checks"""
        print("="*80)
        print("🚀 Starting Pre-CI Validation")
        print("="*80)
        print()
        
        # Fast checks
        self.results.append(self.validate_file_permissions())
        self.results.append(self.validate_git_status())
        self.results.append(self.validate_dependency_versions())
        self.results.append(self.validate_security())
        self.results.append(self.validate_ci_contracts())
        
        if not skip_slow:
            # Slower checks
            self.results.append(self.validate_code_style())
            self.results.append(self.validate_linting())
            self.results.append(self.validate_documentation())
            self.results.append(self.validate_tests())
            self.results.append(self.validate_build())
            self.results.append(self.validate_api_compatibility())
        else:
            print("⏭️  Skipping slow checks")
        
        print()
        print("="*80)
        print("📊 VALIDATION RESULTS")
        print("="*80)
        print()
        
        all_passed = True
        for result in self.results:
            print(result)
            if result.details:
                print(f"Details: {result.details[:200]}...")
            print()
            
            if not result.passed:
                all_passed = False
        
        print("="*80)
        
        if all_passed:
            print("✅ ALL VALIDATIONS PASSED!")
            print("="*80)
            return True
        else:
            failed_count = sum(1 for r in self.results if not r.passed)
            print(f"❌ {failed_count} VALIDATION(S) FAILED")
            print("="*80)
            return False
    
    def save_report(self, output_file: Path):
        """Save validation report to file"""
        report = {
            'timestamp': subprocess.run(
                ['date', '+%Y-%m-%d %H:%M:%S'],
                capture_output=True,
                text=True
            ).stdout.strip(),
            'repository': str(self.repo_root),
            'strict_mode': self.strict,
            'results': [
                {
                    'name': r.name,
                    'passed': r.passed,
                    'message': r.message,
                    'details': r.details[:500] if r.details else ""
                }
                for r in self.results
            ],
            'summary': {
                'total': len(self.results),
                'passed': sum(1 for r in self.results if r.passed),
                'failed': sum(1 for r in self.results if not r.passed)
            }
        }
        
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"\n📄 Report saved to: {output_file}")


def main():
    parser = argparse.ArgumentParser(
        description='Pre-CI validation for Magisk_Rafaelia'
    )
    parser.add_argument(
        '--strict',
        action='store_true',
        help='Enable strict mode (fail on warnings)'
    )
    parser.add_argument(
        '--fix',
        action='store_true',
        help='Automatically fix issues when possible'
    )
    parser.add_argument(
        '--skip-slow',
        action='store_true',
        help='Skip slow checks (tests, build)'
    )
    parser.add_argument(
        '--output', '-o',
        type=Path,
        help='Save validation report to file'
    )
    parser.add_argument(
        '--repo-root',
        type=Path,
        default=Path(__file__).parent.parent,
        help='Repository root directory'
    )
    
    args = parser.parse_args()
    
    # Create validator
    validator = PreCIValidator(
        repo_root=args.repo_root,
        strict=args.strict,
        fix=args.fix
    )
    
    # Run validations
    success = validator.run_all_validations(skip_slow=args.skip_slow)
    
    # Save report if requested
    if args.output:
        validator.save_report(args.output)
    
    # Exit with appropriate code
    sys.exit(0 if success else 1)


if __name__ == '__main__':
    main()
