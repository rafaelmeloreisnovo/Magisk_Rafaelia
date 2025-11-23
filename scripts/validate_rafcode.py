#!/usr/bin/env python3
"""
RAFCODE-Φ Validation Script
════════════════════════════════════════════════════════════════════════════════
RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
Selo: ΣΩΔΦBITRAF
════════════════════════════════════════════════════════════════════════════════

Validates RAFCODE-Φ signature presence and format in project files.
"""

import os
import sys
import re
from pathlib import Path
from typing import List, Tuple, Dict

# Expected RAFCODE-Φ format
RAFCODE_PATTERN = re.compile(
    r'RAFCODE-Φ.*Rafael.*Ω|RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ',
    re.IGNORECASE
)

# Core files that MUST contain RAFCODE-Φ
REQUIRED_FILES = [
    'AUTHORS_RAFAELIA.md',
    'ETHICS.md',
    'LEGAL.md',
    'RAFAELIA_CORE.md',
    'RAFAELIA_MANIFEST.json',
]

# Directories to scan for RAFCODE presence
SCAN_PATTERNS = [
    '**/*.md',
    '**/*.json',
    '**/*.yml',
    '**/*.yaml',
]

class RAFCODEValidator:
    """Validator for RAFCODE-Φ signatures."""
    
    def __init__(self, root_dir: str = '.'):
        self.root_dir = Path(root_dir)
        self.errors: List[str] = []
        self.warnings: List[str] = []
        self.successes: List[str] = []
        
    def validate_file(self, filepath: Path) -> Tuple[bool, str]:
        """
        Validate RAFCODE-Φ presence in a file.
        
        Returns:
            Tuple of (is_valid, message)
        """
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
                
            if RAFCODE_PATTERN.search(content):
                return True, f"✅ RAFCODE-Φ present in {filepath}"
            else:
                return False, f"❌ RAFCODE-Φ missing in {filepath}"
                
        except Exception as e:
            return False, f"⚠️  Error reading {filepath}: {e}"
    
    def validate_required_files(self) -> bool:
        """Validate that all required files have RAFCODE-Φ."""
        print("🔐 Validating RAFCODE-Φ in required files...\n")
        
        all_valid = True
        for filename in REQUIRED_FILES:
            filepath = self.root_dir / filename
            
            if not filepath.exists():
                self.errors.append(f"❌ Required file missing: {filename}")
                all_valid = False
                continue
            
            is_valid, message = self.validate_file(filepath)
            
            if is_valid:
                self.successes.append(message)
            else:
                self.errors.append(message)
                all_valid = False
                
        return all_valid
    
    def scan_project_files(self) -> Dict[str, int]:
        """Scan project files for RAFCODE-Φ presence."""
        print("\n🔍 Scanning project files for RAFCODE-Φ presence...\n")
        
        stats = {
            'total': 0,
            'with_rafcode': 0,
            'without_rafcode': 0,
        }
        
        # Exclude certain directories
        exclude_dirs = {'.git', 'node_modules', '__pycache__', '.pytest_cache', 
                       'build', 'dist', 'out', '.gradle', '.idea'}
        
        for pattern in SCAN_PATTERNS:
            for filepath in self.root_dir.glob(pattern):
                # Skip excluded directories
                if any(excl in filepath.parts for excl in exclude_dirs):
                    continue
                
                # Skip if not a file
                if not filepath.is_file():
                    continue
                    
                stats['total'] += 1
                
                is_valid, _ = self.validate_file(filepath)
                if is_valid:
                    stats['with_rafcode'] += 1
                else:
                    stats['without_rafcode'] += 1
        
        return stats
    
    def validate_signature_format(self) -> bool:
        """Validate that RAFCODE-Φ signatures have proper format."""
        print("\n📋 Validating RAFCODE-Φ signature format...\n")
        
        # Full expected format
        full_pattern = re.compile(r'RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ')
        
        has_full_format = False
        for filename in REQUIRED_FILES:
            filepath = self.root_dir / filename
            
            if not filepath.exists():
                continue
                
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                    
                if full_pattern.search(content):
                    self.successes.append(
                        f"✅ Full RAFCODE-Φ format in {filename}"
                    )
                    has_full_format = True
                    
            except Exception as e:
                self.warnings.append(f"⚠️  Error reading {filepath}: {e}")
        
        if has_full_format:
            return True
        else:
            self.warnings.append(
                "⚠️  Full RAFCODE-Φ format (RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ) "
                "recommended in core files"
            )
            return True  # Not a critical error
    
    def check_author_attribution(self) -> bool:
        """Check that Rafael Melo Reis Novo attribution is present."""
        print("\n👤 Checking author attribution...\n")
        
        author_pattern = re.compile(
            r'Rafael\s+Melo\s+Reis\s+Novo|Rafael Melo Reis Novo',
            re.IGNORECASE
        )
        
        attribution_count = 0
        for filename in ['AUTHORS_RAFAELIA.md', 'README.MD', 'RAFAELIA_CORE.md']:
            filepath = self.root_dir / filename
            
            if not filepath.exists():
                continue
                
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                    
                if author_pattern.search(content):
                    self.successes.append(
                        f"✅ Author attribution in {filename}"
                    )
                    attribution_count += 1
                    
            except Exception as e:
                self.warnings.append(f"⚠️  Error reading {filepath}: {e}")
        
        if attribution_count >= 2:
            return True
        else:
            self.errors.append(
                "❌ Author attribution (Rafael Melo Reis Novo) required "
                "in at least 2 core files"
            )
            return False
    
    def print_report(self) -> bool:
        """Print validation report."""
        print("\n" + "="*80)
        print("📊 RAFCODE-Φ VALIDATION REPORT")
        print("="*80 + "\n")
        
        # Print successes
        if self.successes:
            print("✅ SUCCESSES:")
            for success in self.successes:
                print(f"  {success}")
            print()
        
        # Print warnings
        if self.warnings:
            print("⚠️  WARNINGS:")
            for warning in self.warnings:
                print(f"  {warning}")
            print()
        
        # Print errors
        if self.errors:
            print("❌ ERRORS:")
            for error in self.errors:
                print(f"  {error}")
            print()
        
        # Overall status
        if self.errors:
            print("❌ VALIDATION FAILED")
            print("\nRAFCODE-Φ validation found critical issues.")
            return False
        elif self.warnings:
            print("⚠️  VALIDATION PASSED WITH WARNINGS")
            print("\nRAFCODE-Φ validation successful with minor issues.")
            return True
        else:
            print("✅ VALIDATION PASSED")
            print("\nRAFCODE-Φ validation successful!")
            return True
    
    def run(self) -> bool:
        """Run complete validation."""
        print("🚀 Starting RAFCODE-Φ validation...\n")
        print("RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ")
        print("Selo: ΣΩΔΦBITRAF\n")
        
        # Validate required files
        required_valid = self.validate_required_files()
        
        # Scan project files
        stats = self.scan_project_files()
        print(f"\n📊 Scan Statistics:")
        print(f"  Total files scanned: {stats['total']}")
        print(f"  Files with RAFCODE-Φ: {stats['with_rafcode']}")
        print(f"  Files without RAFCODE-Φ: {stats['without_rafcode']}")
        
        # Validate signature format
        format_valid = self.validate_signature_format()
        
        # Check author attribution
        attribution_valid = self.check_author_attribution()
        
        # Print final report
        all_valid = self.print_report()
        
        return all_valid and required_valid and attribution_valid


def main():
    """Main entry point."""
    # Get root directory from command line or use current directory
    root_dir = sys.argv[1] if len(sys.argv) > 1 else '.'
    
    validator = RAFCODEValidator(root_dir)
    
    if validator.run():
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == '__main__':
    main()
