#!/usr/bin/env python3
"""
RAFAELIA State Validator
Validates state transitions in the 1008-state matrix
Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
"""

import json
import sys
import argparse
import csv
from pathlib import Path
from collections import defaultdict
from typing import Dict, Tuple, Optional, Any

class StateValidator:
    def __init__(self, state_matrix_path: Path):
        self.states = {}  # state_id -> state_info
        self.transitions = defaultdict(set)  # from_state -> set of to_states
        self.primitives = set()
        self.contexts = set()
        self.load_state_matrix(state_matrix_path)
        
    def load_state_matrix(self, matrix_path: Path):
        """Load the 1008-state matrix from CSV"""
        print(f"Loading state matrix from {matrix_path}...")
        
        if not matrix_path.exists():
            print(f"Error: State matrix file not found: {matrix_path}", file=sys.stderr)
            return
            
        try:
            with open(matrix_path, 'r') as f:
                reader = csv.DictReader(f)
                for row in reader:
                    state_id = row.get('state_id', '')
                    if state_id:
                        self.states[state_id] = row
                        primitive = row.get('primitive', '')
                        context = row.get('context', '')
                        if primitive:
                            self.primitives.add(primitive)
                        if context:
                            self.contexts.add(context)
                            
            print(f"Loaded {len(self.states)} states, {len(self.primitives)} primitives, {len(self.contexts)} contexts")
        except Exception as e:
            print(f"Error loading state matrix: {e}", file=sys.stderr)
            
    def validate_state_id(self, state_id: str) -> Tuple[bool, Optional[str]]:
        """Validate a state ID exists in the matrix"""
        if state_id in self.states:
            return True, None
        else:
            return False, f"State ID not found in matrix: {state_id}"
            
    def validate_primitive(self, primitive: str) -> Tuple[bool, Optional[str]]:
        """Validate a primitive exists"""
        if primitive in self.primitives:
            return True, None
        else:
            available = ', '.join(sorted(self.primitives)[:5])
            return False, f"Unknown primitive: {primitive}. Available: {available}, ..."
            
    def validate_context(self, context: str) -> Tuple[bool, Optional[str]]:
        """Validate a context exists"""
        if context in self.contexts:
            return True, None
        else:
            available = ', '.join(sorted(self.contexts)[:5])
            return False, f"Unknown context: {context}. Available: {available}, ..."
            
    def validate_state_combination(self, primitive: str, context: str) -> Tuple[bool, Optional[str]]:
        """Validate that a primitive+context combination is valid"""
        state_id = f"PRIM_{primitive}_CTX_{context}"
        return self.validate_state_id(state_id)
        
    def validate_transition(self, from_state: str, to_state: str) -> Tuple[bool, Optional[str]]:
        """Validate that a state transition is valid"""
        # For now, we accept all transitions between valid states
        # In a future version, we could define valid transition rules
        
        valid_from, msg_from = self.validate_state_id(from_state)
        if not valid_from:
            return False, msg_from
            
        valid_to, msg_to = self.validate_state_id(to_state)
        if not valid_to:
            return False, msg_to
            
        return True, None
        
    def validate_audit_log(self, audit_log_path: Path) -> Dict[str, Any]:
        """Validate all state transitions in an audit log"""
        print(f"Validating audit log: {audit_log_path}")
        
        results = {
            'total_entries': 0,
            'valid_entries': 0,
            'invalid_entries': 0,
            'errors': [],
            'warnings': [],
        }
        
        prev_state = None
        
        try:
            with open(audit_log_path, 'r') as f:
                for line_num, line in enumerate(f, 1):
                    if not line.strip():
                        continue
                        
                    results['total_entries'] += 1
                    
                    try:
                        entry = json.loads(line)
                        
                        # Validate primitive
                        primitive = entry.get('primitive', '')
                        valid, msg = self.validate_primitive(primitive)
                        if not valid:
                            results['invalid_entries'] += 1
                            results['errors'].append({
                                'line': line_num,
                                'error': msg,
                                'entry': entry
                            })
                            continue
                            
                        # Validate context
                        context = entry.get('context', '')
                        valid, msg = self.validate_context(context)
                        if not valid:
                            results['invalid_entries'] += 1
                            results['errors'].append({
                                'line': line_num,
                                'error': msg,
                                'entry': entry
                            })
                            continue
                            
                        # Validate state ID
                        state_id = entry.get('state_id', '')
                        expected_state_id = f"PRIM_{primitive}_CTX_{context}"
                        if state_id != expected_state_id:
                            results['warnings'].append({
                                'line': line_num,
                                'warning': f"State ID mismatch: got '{state_id}', expected '{expected_state_id}'",
                                'entry': entry
                            })
                            
                        valid, msg = self.validate_state_id(expected_state_id)
                        if not valid:
                            results['invalid_entries'] += 1
                            results['errors'].append({
                                'line': line_num,
                                'error': msg,
                                'entry': entry
                            })
                            continue
                            
                        # Validate transition if we have a previous state
                        if prev_state:
                            valid, msg = self.validate_transition(prev_state, expected_state_id)
                            if not valid:
                                results['warnings'].append({
                                    'line': line_num,
                                    'warning': msg,
                                    'entry': entry
                                })
                                
                        results['valid_entries'] += 1
                        prev_state = expected_state_id
                        
                    except json.JSONDecodeError as e:
                        results['invalid_entries'] += 1
                        results['errors'].append({
                            'line': line_num,
                            'error': f"JSON parse error: {e}",
                            'entry': None
                        })
                        
        except Exception as e:
            print(f"Error reading audit log: {e}", file=sys.stderr)
            
        return results
        
    def generate_report(self, results: Dict[str, Any], output_path: Optional[Path] = None):
        """Generate validation report"""
        report = []
        report.append("=" * 80)
        report.append("RAFAELIA State Validation Report")
        report.append("Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ")
        report.append("=" * 80)
        report.append("")
        
        report.append("Summary:")
        report.append(f"  Total Entries: {results['total_entries']}")
        report.append(f"  Valid Entries: {results['valid_entries']}")
        report.append(f"  Invalid Entries: {results['invalid_entries']}")
        
        if results['total_entries'] > 0:
            success_rate = (results['valid_entries'] / results['total_entries']) * 100
            report.append(f"  Success Rate: {success_rate:.2f}%")
        report.append("")
        
        if results['errors']:
            report.append(f"Errors ({len(results['errors'])}):")
            for i, error in enumerate(results['errors'][:10], 1):
                report.append(f"  {i}. Line {error['line']}: {error['error']}")
            if len(results['errors']) > 10:
                report.append(f"  ... and {len(results['errors']) - 10} more errors")
            report.append("")
            
        if results['warnings']:
            report.append(f"Warnings ({len(results['warnings'])}):")
            for i, warning in enumerate(results['warnings'][:10], 1):
                report.append(f"  {i}. Line {warning['line']}: {warning['warning']}")
            if len(results['warnings']) > 10:
                report.append(f"  ... and {len(results['warnings']) - 10} more warnings")
            report.append("")
            
        report.append("=" * 80)
        
        report_text = '\n'.join(report)
        
        if output_path:
            with open(output_path, 'w') as f:
                f.write(report_text)
            print(f"Report saved to {output_path}")
        else:
            print(report_text)
            
def main():
    parser = argparse.ArgumentParser(description='RAFAELIA State Validator')
    parser.add_argument('--matrix', type=Path, required=True,
                       help='Path to state matrix CSV file')
    parser.add_argument('--audit-log', type=Path, required=True,
                       help='Path to audit log file (JSONL format)')
    parser.add_argument('--output', type=Path,
                       help='Output report file (default: stdout)')
    parser.add_argument('--verbose', action='store_true',
                       help='Enable verbose output')
    
    args = parser.parse_args()
    
    # Create validator
    validator = StateValidator(args.matrix)
    
    # Validate audit log
    results = validator.validate_audit_log(args.audit_log)
    
    # Generate report
    validator.generate_report(results, args.output)
    
    # Exit with error if there were validation errors
    sys.exit(1 if results['invalid_entries'] > 0 else 0)
    
if __name__ == '__main__':
    main()
