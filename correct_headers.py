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
Copyright Header Correction Script
Automatically corrects GPL-3.0 violating headers in source files

Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
Licensed under GPL-3.0 as part of Magisk_Rafaelia
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Tuple, Optional

# GPL-3.0 compliant header templates
CPP_HEADER_TEMPLATE = '''/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
{historical_copyrights}
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */
'''

PYTHON_HEADER_TEMPLATE = '''# Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
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
'''

# Historical copyrights to detect and preserve
HISTORICAL_COPYRIGHTS = [
    (r'Copyright.*Pierre-Hugues Husson.*<phh@phh.me>', ' * Copyright 2015, Pierre-Hugues Husson <phh@phh.me>\n'),
    (r'Copyright.*Adam Shanks.*@ChainsDD', ' * Copyright 2010, Adam Shanks (@ChainsDD)\n'),
    (r'Copyright.*Zinx Verituse.*@zinxv', ' * Copyright 2008, Zinx Verituse (@zinxv)\n'),
]

def find_header_end(content: str, file_ext: str) -> Optional[int]:
    """Find the end position of the existing header"""
    if file_ext in ['.cpp', '.c', '.h', '.hpp', '.rs', '.java', '.kt']:
        # Find end of C-style comment
        match = re.search(r'\*/\s*\n', content)
        if match:
            return match.end()
    elif file_ext == '.py':
        # Find end of Python comment block
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line and not line.strip().startswith('#') and line.strip():
                return sum(len(l) + 1 for l in lines[:i])
    return None

def extract_historical_copyrights(content: str) -> str:
    """Extract and preserve historical copyright notices"""
    historical = ""
    for pattern, copyright_line in HISTORICAL_COPYRIGHTS:
        if re.search(pattern, content, re.IGNORECASE):
            historical += copyright_line
    return historical

def detect_problematic_header(content: str) -> bool:
    """Detect if file has problematic GPL-violating header"""
    indicators = [
        'All Rights Reserved',
        'DUAL LICENSE',
        'COMMERCIAL SAAS LICENSE',
        'Patent Pending',
        'AUTOMATIC PENALTIES',
    ]
    return any(indicator in content for indicator in indicators)

def get_file_extension(filepath: Path) -> str:
    """Get file extension"""
    return filepath.suffix.lower()

def correct_file_header(filepath: Path, dry_run: bool = True) -> Tuple[bool, str]:
    """
    Correct the header of a single file
    
    Returns:
        (success, message)
    """
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if file needs correction
        if not detect_problematic_header(content):
            return (False, f"SKIP: No problematic header detected")
        
        ext = get_file_extension(filepath)
        
        # Find where header ends
        header_end = find_header_end(content, ext)
        if header_end is None:
            return (False, f"SKIP: Could not determine header boundary")
        
        # Extract historical copyrights
        historical = extract_historical_copyrights(content)
        
        # Get code content (after header)
        code_content = content[header_end:]
        
        # Generate new header based on file type
        if ext in ['.cpp', '.c', '.h', '.hpp', '.rs', '.java', '.kt']:
            new_header = CPP_HEADER_TEMPLATE.format(
                historical_copyrights=historical + ' *' if historical else ' *'
            )
        elif ext == '.py':
            new_header = PYTHON_HEADER_TEMPLATE
        else:
            return (False, f"SKIP: Unsupported file type {ext}")
        
        # Combine new header with code
        new_content = new_header + '\n' + code_content.lstrip()
        
        if dry_run:
            return (True, f"WOULD CORRECT: {len(content)} -> {len(new_content)} bytes")
        else:
            # Write corrected content
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return (True, f"CORRECTED: {len(content)} -> {len(new_content)} bytes")
            
    except Exception as e:
        return (False, f"ERROR: {str(e)}")

def find_files_needing_correction(root_dir: Path) -> List[Path]:
    """Find all source files that need header correction"""
    extensions = ['.cpp', '.c', '.h', '.hpp', '.rs', '.java', '.kt', '.py']
    exclude_dirs = ['.git', 'external', '__pycache__', 'build', '.gradle']
    
    files = []
    for ext in extensions:
        for filepath in root_dir.rglob(f'*{ext}'):
            # Skip excluded directories
            if any(excl in str(filepath) for excl in exclude_dirs):
                continue
            # Check if file needs correction
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                    if detect_problematic_header(content):
                        files.append(filepath)
            except:
                pass  # Skip unreadable files
    
    return files

def main():
    """Main execution"""
    import argparse
    
    parser = argparse.ArgumentParser(
        description='Correct GPL-violating copyright headers in source files'
    )
    parser.add_argument(
        'path',
        type=str,
        nargs='?',
        default='.',
        help='Root directory to search (default: current directory)'
    )
    parser.add_argument(
        '--apply',
        action='store_true',
        help='Actually apply corrections (default is dry-run)'
    )
    parser.add_argument(
        '--file',
        type=str,
        help='Correct a single file instead of scanning directory'
    )
    
    args = parser.parse_args()
    
    root_dir = Path(args.path).resolve()
    
    if args.file:
        # Correct single file
        filepath = Path(args.file)
        if not filepath.exists():
            print(f"ERROR: File not found: {filepath}")
            return 1
        
        success, message = correct_file_header(filepath, dry_run=not args.apply)
        print(f"{filepath}: {message}")
        return 0 if success else 1
    
    else:
        # Scan and correct multiple files
        print(f"Scanning for files with problematic headers in: {root_dir}")
        files = find_files_needing_correction(root_dir)
        
        print(f"\nFound {len(files)} files needing correction\n")
        
        if not files:
            print("No files need correction!")
            return 0
        
        if not args.apply:
            print("DRY RUN MODE - No files will be modified")
            print("Use --apply to actually correct files\n")
        
        corrected = 0
        skipped = 0
        errors = 0
        
        for filepath in files:
            rel_path = filepath.relative_to(root_dir)
            success, message = correct_file_header(filepath, dry_run=not args.apply)
            
            status = "✓" if success else "✗"
            print(f"{status} {rel_path}: {message}")
            
            if success:
                corrected += 1
            elif "ERROR" in message:
                errors += 1
            else:
                skipped += 1
        
        print(f"\nSummary:")
        print(f"  Corrected: {corrected}")
        print(f"  Skipped: {skipped}")
        print(f"  Errors: {errors}")
        
        if not args.apply and corrected > 0:
            print(f"\nTo apply corrections, run with --apply flag")
        
        return 0 if errors == 0 else 1

if __name__ == '__main__':
    sys.exit(main())
