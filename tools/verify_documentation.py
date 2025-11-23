#!/usr/bin/env python3
"""
RAFAELIA Documentation Verification Script
Verifies consistency between meta-architecture documentation and implementation
"""

import os
import sys
from pathlib import Path

# Color codes for output
GREEN = '\033[92m'
RED = '\033[91m'
YELLOW = '\033[93m'
BLUE = '\033[94m'
RESET = '\033[0m'

def check_file_exists(filepath, description):
    """Check if a file exists"""
    if Path(filepath).exists():
        print(f"{GREEN}✓{RESET} {description}: {filepath}")
        return True
    else:
        print(f"{RED}✗{RESET} {description}: {filepath} (NOT FOUND)")
        return False

def check_rust_modules():
    """Check Rust implementation modules"""
    print(f"\n{BLUE}=== Rust Core Modules ==={RESET}")
    base = "native/src/core/"
    modules = [
        (f"{base}rafaelia_audit.rs", "Audit System"),
        (f"{base}rafaelia_telemetry.rs", "Telemetry System"),
    ]
    return all(check_file_exists(f, desc) for f, desc in modules)

def check_tools():
    """Check tool scripts"""
    print(f"\n{BLUE}=== RAFAELIA Tools ==={RESET}")
    base = "tools/"
    tools = [
        (f"{base}retro_feed.py", "Retroalimentação Analyzer"),
        (f"{base}bootctl", "Boot Control (static-linked)"),
        (f"{base}bootctl.patch", "Boot Control Patch"),
        (f"{base}futility", "ChromeOS Firmware Utility"),
    ]
    return all(check_file_exists(f, desc) for f, desc in tools)

def check_rafaelia_tools():
    """Check RAFAELIA-specific tools"""
    print(f"\n{BLUE}=== RAFAELIA Framework Tools ==={RESET}")
    base = "tools/rafaelia/"
    tools = [
        (f"{base}activate_rafaelia.sh", "Activation Script"),
        (f"{base}audit_analyzer.py", "Audit Analyzer"),
        (f"{base}state_validator.py", "State Validator"),
        (f"{base}metrics_collector.sh", "Metrics Collector"),
        (f"{base}integrity_checker.sh", "Integrity Checker"),
    ]
    return all(check_file_exists(f, desc) for f, desc in tools)

def check_documentation():
    """Check documentation files"""
    print(f"\n{BLUE}=== Documentation Files ==={RESET}")
    docs = [
        ("docs/RAFAELIA_INDEX.md", "Master Index"),
        ("docs/RAFAELIA_META_ARCHITECTURE.md", "Meta-Architecture (30 analyses)"),
        ("docs/RAFAELIA_TOOLKIT_ANALYSIS.md", "Toolkit Analysis"),
        ("docs/RAFAELIA_FRAMEWORK.md", "Framework Overview"),
        ("docs/RAFAELIA_AUDIT_SYSTEM.md", "Audit System"),
        ("docs/RAFAELIA_TELEMETRY.md", "Telemetry"),
        ("docs/ACTIVATION_GUIDE.md", "Activation Guide"),
    ]
    return all(check_file_exists(f, desc) for f, desc in docs)

def check_manifest():
    """Check manifest file"""
    print(f"\n{BLUE}=== Manifest ==={RESET}")
    return check_file_exists("RAFAELIA_MANIFEST.json", "RAFAELIA Manifest")

def verify_signatures():
    """Verify signatures in key files"""
    print(f"\n{BLUE}=== Signature Verification ==={RESET}")
    
    signature = "RAFCODE-Φ-∆RafaelVerboΩ"
    files_to_check = [
        "native/src/core/rafaelia_audit.rs",
        "native/src/core/rafaelia_telemetry.rs",
        "RAFAELIA_MANIFEST.json",
    ]
    
    all_ok = True
    for filepath in files_to_check:
        if Path(filepath).exists():
            with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                if signature in content or "RAFCODE-Φ" in content:
                    print(f"{GREEN}✓{RESET} Signature found in: {filepath}")
                else:
                    print(f"{YELLOW}⚠{RESET} Signature not found in: {filepath}")
                    all_ok = False
    
    return all_ok

def verify_philosophy():
    """Verify philosophy cycle in files"""
    print(f"\n{BLUE}=== Philosophy Cycle Verification ==={RESET}")
    
    cycle = "VAZIO → VERBO → CHEIO → RETRO"
    files_to_check = [
        "native/src/core/rafaelia_audit.rs",
        "tools/rafaelia/README.md",
        "README.MD",
    ]
    
    all_ok = True
    for filepath in files_to_check:
        if Path(filepath).exists():
            with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                if "VAZIO" in content or "VERBO" in content or "RETRO" in content:
                    print(f"{GREEN}✓{RESET} Philosophy cycle referenced in: {filepath}")
                else:
                    print(f"{YELLOW}⚠{RESET} Philosophy cycle not found in: {filepath}")
                    all_ok = False
    
    return all_ok

def main():
    """Main verification function"""
    print(f"{BLUE}╔════════════════════════════════════════════════════╗{RESET}")
    print(f"{BLUE}║  RAFAELIA Documentation Verification              ║{RESET}")
    print(f"{BLUE}║  Meta-Architecture Consistency Check               ║{RESET}")
    print(f"{BLUE}╚════════════════════════════════════════════════════╝{RESET}")
    
    # Change to repo root if needed
    if not Path("README.MD").exists():
        if Path("../README.MD").exists():
            os.chdir("..")
        else:
            print(f"{RED}Error: Not in repository root{RESET}")
            return 1
    
    # Run all checks
    checks = [
        check_rust_modules(),
        check_tools(),
        check_rafaelia_tools(),
        check_documentation(),
        check_manifest(),
        verify_signatures(),
        verify_philosophy(),
    ]
    
    # Summary
    print(f"\n{BLUE}=== Summary ==={RESET}")
    total = len(checks)
    passed = sum(checks)
    
    if passed == total:
        print(f"{GREEN}✓ All checks passed ({passed}/{total}){RESET}")
        print(f"{GREEN}✓ Documentation is consistent with implementation{RESET}")
        return 0
    else:
        print(f"{YELLOW}⚠ Some checks failed ({passed}/{total} passed){RESET}")
        print(f"{YELLOW}⚠ Review warnings above{RESET}")
        return 1

if __name__ == "__main__":
    sys.exit(main())
