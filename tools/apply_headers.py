#!/usr/bin/env python3
"""
Apply Comprehensive Legal and Ethical Headers to Source Files

This script applies standardized headers with international legal references
to all source files in the Magisk_Rafaelia project.

Part of Magisk_Rafaelia Governance Framework
Author: Rafael Melo Reis (rafaelmeloreisnovo)
"""

import os
import sys
import re
import hashlib
from pathlib import Path
from datetime import datetime
from typing import List, Tuple, Optional

# Add tools directory to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

try:
    from tools.header_templates import (
        generate_python_header,
        generate_rust_header,
        generate_cpp_header,
        generate_shell_header,
        PRIMARY_AUTHOR,
        CURRENT_YEAR
    )
except ImportError:
    print("ERROR: Cannot import header_templates module")
    print("Please ensure tools/header_templates.py exists")
    sys.exit(1)


class HeaderApplicator:
    """Apply headers to source files."""
    
    def __init__(self, repo_root: str, dry_run: bool = False):
        """
        Initialize header applicator.
        
        Args:
            repo_root: Root directory of repository
            dry_run: If True, don't modify files, just report
        """
        self.repo_root = Path(repo_root)
        self.dry_run = dry_run
        self.stats = {
            'processed': 0,
            'modified': 0,
            'skipped': 0,
            'errors': 0
        }
        
    def should_process_file(self, file_path: Path) -> bool:
        """Check if file should be processed."""
        # Skip build artifacts, dependencies, etc.
        skip_dirs = {
            '.git', '__pycache__', 'node_modules', 'build', 'out',
            '.gradle', 'dist', 'target', '.idea', '.vscode'
        }
        
        # Check if any parent is in skip_dirs
        for parent in file_path.parents:
            if parent.name in skip_dirs:
                return False
                
        # Skip if file is too large (>1MB)
        try:
            if file_path.stat().st_size > 1_000_000:
                return False
        except Exception:
            return False
            
        return True
        
    def extract_file_description(self, file_path: Path, content: str) -> str:
        """Extract or generate file description."""
        # Try to find existing description
        lines = content.split('\n')
        
        # For Python files
        if file_path.suffix == '.py':
            for i, line in enumerate(lines[:30]):
                if '"""' in line and i < 20:
                    # Found docstring
                    desc_lines = []
                    in_docstring = True
                    for j in range(i+1, min(i+10, len(lines))):
                        if '"""' in lines[j]:
                            break
                        desc_lines.append(lines[j].strip())
                    if desc_lines:
                        return ' '.join(desc_lines)
                        
        # For other files with comments
        comment_lines = []
        for line in lines[:20]:
            line = line.strip()
            if line.startswith('#') or line.startswith('//') or line.startswith('*'):
                clean = line.lstrip('#/*').strip()
                if clean and not clean.startswith('!') and len(clean) > 10:
                    comment_lines.append(clean)
                    
        if comment_lines:
            return ' '.join(comment_lines[:3])
            
        # Generate default description
        return f"{file_path.name} - Part of Magisk_Rafaelia"
        
    def has_comprehensive_header(self, content: str) -> bool:
        """Check if file already has comprehensive header."""
        # Look for key markers of our comprehensive header
        markers = [
            'INTERNATIONAL LEGAL COMPLIANCE',
            'Berne Convention',
            'ETHICA[8]',
            'NAUTICAL ANCHORS',
            'RAFCODE'
        ]
        
        # File has comprehensive header if it contains most markers
        found = sum(1 for marker in markers if marker in content[:5000])
        return found >= 3
        
    def remove_old_header(self, content: str, file_type: str) -> str:
        """Remove old header to replace with new one."""
        lines = content.split('\n')
        
        if file_type == 'python':
            # Find end of shebang and old docstring
            start_idx = 0
            if lines and lines[0].startswith('#!'):
                start_idx = 1
                
            # Skip old docstring if exists
            if start_idx < len(lines) and '"""' in lines[start_idx]:
                for i in range(start_idx + 1, len(lines)):
                    if '"""' in lines[i]:
                        start_idx = i + 1
                        break
                        
            # Skip blank lines and old copyright
            while start_idx < len(lines):
                line = lines[start_idx].strip()
                if not line or line.startswith('#') and (
                    'copyright' in line.lower() or 
                    'license' in line.lower() or
                    'author' in line.lower()
                ):
                    start_idx += 1
                else:
                    break
                    
            return '\n'.join(lines[start_idx:])
            
        elif file_type in ['rust', 'cpp']:
            # Skip old comment blocks
            start_idx = 0
            in_comment = False
            
            for i, line in enumerate(lines):
                stripped = line.strip()
                
                if stripped.startswith('/*'):
                    in_comment = True
                    
                if in_comment:
                    if '*/' in stripped:
                        in_comment = False
                        start_idx = i + 1
                    continue
                    
                if stripped.startswith('//'):
                    start_idx = i + 1
                    continue
                    
                if stripped and not stripped.startswith('#'):
                    break
                    
            return '\n'.join(lines[start_idx:])
            
        elif file_type == 'shell':
            # Skip shebang and old comments
            start_idx = 0
            if lines and lines[0].startswith('#!'):
                start_idx = 1
                
            while start_idx < len(lines):
                line = lines[start_idx].strip()
                if not line or (line.startswith('#') and not line.startswith('#!')):
                    start_idx += 1
                else:
                    break
                    
            return '\n'.join(lines[start_idx:])
            
        return content
        
    def apply_header_to_file(self, file_path: Path) -> bool:
        """
        Apply comprehensive header to a single file.
        
        Returns:
            True if file was modified, False otherwise
        """
        try:
            # Read file
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                
            # Check if already has comprehensive header
            if self.has_comprehensive_header(content):
                print(f"  ✓ Already has comprehensive header: {file_path.relative_to(self.repo_root)}")
                self.stats['skipped'] += 1
                return False
                
            # Determine file type
            suffix = file_path.suffix
            file_type = None
            header_generator = None
            
            if suffix == '.py':
                file_type = 'python'
                header_generator = generate_python_header
            elif suffix == '.rs':
                file_type = 'rust'
                header_generator = generate_rust_header
            elif suffix in ['.cpp', '.hpp', '.cc', '.h']:
                file_type = 'cpp'
                header_generator = generate_cpp_header
            elif suffix == '.sh':
                file_type = 'shell'
                header_generator = generate_shell_header
            else:
                self.stats['skipped'] += 1
                return False
                
            # Extract description
            description = self.extract_file_description(file_path, content)
            
            # Remove old header
            clean_content = self.remove_old_header(content, file_type)
            
            # Generate new header
            rel_path = str(file_path.relative_to(self.repo_root))
            new_header = header_generator(
                file_path=rel_path,
                description=description
            )
            
            # Combine header and content
            new_content = new_header + '\n' + clean_content.lstrip()
            
            # Write file (if not dry run)
            if not self.dry_run:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                    
            print(f"  ✓ Applied header to: {rel_path}")
            self.stats['modified'] += 1
            return True
            
        except Exception as e:
            print(f"  ✗ Error processing {file_path}: {e}")
            self.stats['errors'] += 1
            return False
            
    def process_directory(self, directory: Path, 
                         extensions: List[str]) -> None:
        """
        Process all files in directory with given extensions.
        
        Args:
            directory: Directory to process
            extensions: List of file extensions to process (e.g., ['.py', '.rs'])
        """
        print(f"\nProcessing directory: {directory.relative_to(self.repo_root)}")
        print(f"Looking for files with extensions: {', '.join(extensions)}")
        
        for ext in extensions:
            pattern = f"**/*{ext}"
            files = list(directory.glob(pattern))
            
            if not files:
                continue
                
            print(f"\nFound {len(files)} {ext} files")
            
            for file_path in sorted(files):
                if not self.should_process_file(file_path):
                    continue
                    
                self.stats['processed'] += 1
                self.apply_header_to_file(file_path)
                
    def run(self) -> None:
        """Run header application on entire repository."""
        print("=" * 70)
        print("RAFAELIA Comprehensive Header Applicator")
        print("=" * 70)
        print(f"Repository: {self.repo_root}")
        print(f"Mode: {'DRY RUN (no files modified)' if self.dry_run else 'LIVE (files will be modified)'}")
        print()
        
        # Process Python files
        self.process_directory(
            self.repo_root,
            ['.py']
        )
        
        # Process Rust files
        native_dir = self.repo_root / 'native'
        if native_dir.exists():
            self.process_directory(
                native_dir,
                ['.rs']
            )
            
        # Process C++ files
        if native_dir.exists():
            self.process_directory(
                native_dir,
                ['.cpp', '.hpp', '.cc', '.h']
            )
            
        # Process Shell scripts
        self.process_directory(
            self.repo_root,
            ['.sh']
        )
        
        # Print summary
        print("\n" + "=" * 70)
        print("SUMMARY")
        print("=" * 70)
        print(f"Files processed: {self.stats['processed']}")
        print(f"Files modified:  {self.stats['modified']}")
        print(f"Files skipped:   {self.stats['skipped']}")
        print(f"Errors:          {self.stats['errors']}")
        print()
        
        if self.dry_run:
            print("This was a DRY RUN. No files were actually modified.")
            print("Run with --apply to actually modify files.")
        else:
            print("✓ Headers applied successfully!")
            print("Please review the changes and commit if satisfied.")


def main():
    """Main entry point."""
    import argparse
    
    parser = argparse.ArgumentParser(
        description='Apply comprehensive legal and ethical headers to source files'
    )
    parser.add_argument(
        '--apply',
        action='store_true',
        help='Actually modify files (default is dry-run)'
    )
    parser.add_argument(
        '--repo',
        type=str,
        default='/home/runner/work/Magisk_Rafaelia/Magisk_Rafaelia',
        help='Repository root directory'
    )
    
    args = parser.parse_args()
    
    # Verify repository exists
    repo_path = Path(args.repo)
    if not repo_path.exists():
        print(f"ERROR: Repository not found at {repo_path}")
        sys.exit(1)
        
    # Run applicator
    applicator = HeaderApplicator(
        repo_root=str(repo_path),
        dry_run=not args.apply
    )
    applicator.run()
    
    sys.exit(0 if applicator.stats['errors'] == 0 else 1)


if __name__ == '__main__':
    main()
