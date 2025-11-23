#!/usr/bin/env python3
"""
RAFAELIA Audit Analyzer
Analyzes audit logs and generates comprehensive reports
"""

import json
import sys
import argparse
from pathlib import Path
from datetime import datetime, timedelta
from collections import defaultdict, Counter
from typing import Dict, List, Any

class AuditAnalyzer:
    def __init__(self):
        self.entries = []
        self.stats = defaultdict(int)
        self.errors = []
        self.warnings = []
        self.primitives = Counter()
        self.contexts = Counter()
        
    def load_audit_logs(self, log_files: List[Path]):
        """Load audit logs from JSON lines files"""
        for log_file in log_files:
            print(f"Loading {log_file}...")
            try:
                with open(log_file, 'r') as f:
                    for line in f:
                        if line.strip():
                            entry = json.loads(line)
                            self.entries.append(entry)
                            self._process_entry(entry)
            except Exception as e:
                print(f"Error loading {log_file}: {e}", file=sys.stderr)
                
    def _process_entry(self, entry: Dict[str, Any]):
        """Process a single audit entry"""
        # Update statistics
        self.stats['total'] += 1
        
        status = entry.get('result', {}).get('status', 'UNKNOWN')
        self.stats[status] += 1
        
        # Track primitives and contexts
        self.primitives[entry.get('primitive', 'unknown')] += 1
        self.contexts[entry.get('context', 'unknown')] += 1
        
        # Collect errors and warnings
        if status == 'ERROR' or status == 'CRITICAL':
            self.errors.append(entry)
        elif status == 'WARN':
            self.warnings.append(entry)
            
    def generate_summary(self) -> Dict[str, Any]:
        """Generate summary statistics"""
        total = self.stats['total']
        
        return {
            'total_operations': total,
            'successful': self.stats.get('SUCCESS', 0),
            'warnings': self.stats.get('WARN', 0),
            'errors': self.stats.get('ERROR', 0),
            'critical': self.stats.get('CRITICAL', 0),
            'success_rate': (self.stats.get('SUCCESS', 0) / total * 100) if total > 0 else 0,
            'top_primitives': self.primitives.most_common(10),
            'top_contexts': self.contexts.most_common(10),
        }
        
    def analyze_performance(self) -> Dict[str, Any]:
        """Analyze performance metrics"""
        durations = []
        cpu_usage = []
        memory_usage = []
        
        for entry in self.entries:
            perf = entry.get('performance', {})
            if 'duration_ms' in perf:
                durations.append(perf['duration_ms'])
            if 'cpu_usage' in perf:
                cpu_usage.append(perf['cpu_usage'])
            if 'memory_mb' in perf:
                memory_usage.append(perf['memory_mb'])
                
        return {
            'duration_ms': {
                'min': min(durations) if durations else 0,
                'max': max(durations) if durations else 0,
                'avg': sum(durations) / len(durations) if durations else 0,
                'p95': self._percentile(durations, 0.95) if durations else 0,
                'p99': self._percentile(durations, 0.99) if durations else 0,
            },
            'cpu_percent': {
                'min': min(cpu_usage) if cpu_usage else 0,
                'max': max(cpu_usage) if cpu_usage else 0,
                'avg': sum(cpu_usage) / len(cpu_usage) if cpu_usage else 0,
            },
            'memory_mb': {
                'min': min(memory_usage) if memory_usage else 0,
                'max': max(memory_usage) if memory_usage else 0,
                'avg': sum(memory_usage) / len(memory_usage) if memory_usage else 0,
            },
        }
        
    def _percentile(self, data: List[float], percentile: float) -> float:
        """Calculate percentile"""
        if not data:
            return 0
        sorted_data = sorted(data)
        index = int(len(sorted_data) * percentile)
        return sorted_data[min(index, len(sorted_data) - 1)]
        
    def analyze_errors(self) -> List[Dict[str, Any]]:
        """Analyze error patterns"""
        error_types = Counter()
        error_primitives = Counter()
        
        for error in self.errors:
            msg = error.get('result', {}).get('message', 'Unknown error')
            error_types[msg] += 1
            error_primitives[error.get('primitive', 'unknown')] += 1
            
        return {
            'total_errors': len(self.errors),
            'error_types': error_types.most_common(10),
            'error_primitives': error_primitives.most_common(10),
            'recent_errors': self.errors[-10:] if len(self.errors) > 10 else self.errors,
        }
        
    def generate_html_report(self, output_file: Path):
        """Generate HTML report"""
        summary = self.generate_summary()
        performance = self.analyze_performance()
        errors = self.analyze_errors()
        
        html = f"""<!DOCTYPE html>
<html>
<head>
    <title>RAFAELIA Audit Report</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 20px;
            background-color: #f5f5f5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #2c3e50;
            border-bottom: 3px solid #3498db;
            padding-bottom: 10px;
        }}
        h2 {{
            color: #34495e;
            margin-top: 30px;
            border-left: 4px solid #3498db;
            padding-left: 10px;
        }}
        .stat-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin: 20px 0;
        }}
        .stat-card {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        .stat-value {{
            font-size: 2em;
            font-weight: bold;
        }}
        .stat-label {{
            font-size: 0.9em;
            opacity: 0.9;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
        }}
        th, td {{
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}
        th {{
            background-color: #3498db;
            color: white;
        }}
        tr:hover {{
            background-color: #f5f5f5;
        }}
        .success {{ color: #27ae60; }}
        .warning {{ color: #f39c12; }}
        .error {{ color: #e74c3c; }}
        .critical {{ color: #c0392b; font-weight: bold; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🔍 RAFAELIA Audit Report</h1>
        <p>Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
        <p>Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ</p>
        
        <h2>📊 Summary Statistics</h2>
        <div class="stat-grid">
            <div class="stat-card">
                <div class="stat-value">{summary['total_operations']}</div>
                <div class="stat-label">Total Operations</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{summary['successful']}</div>
                <div class="stat-label">Successful</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{summary['warnings']}</div>
                <div class="stat-label">Warnings</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{summary['errors']}</div>
                <div class="stat-label">Errors</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{summary['success_rate']:.1f}%</div>
                <div class="stat-label">Success Rate</div>
            </div>
        </div>
        
        <h2>🎯 Top Primitives</h2>
        <table>
            <tr>
                <th>Primitive</th>
                <th>Count</th>
                <th>Percentage</th>
            </tr>
"""
        
        for primitive, count in summary['top_primitives']:
            percentage = (count / summary['total_operations'] * 100)
            html += f"""
            <tr>
                <td>{primitive}</td>
                <td>{count}</td>
                <td>{percentage:.1f}%</td>
            </tr>
"""
        
        html += f"""
        </table>
        
        <h2>🔧 Top Contexts</h2>
        <table>
            <tr>
                <th>Context</th>
                <th>Count</th>
                <th>Percentage</th>
            </tr>
"""
        
        for context, count in summary['top_contexts']:
            percentage = (count / summary['total_operations'] * 100)
            html += f"""
            <tr>
                <td>{context}</td>
                <td>{count}</td>
                <td>{percentage:.1f}%</td>
            </tr>
"""
        
        html += f"""
        </table>
        
        <h2>⚡ Performance Metrics</h2>
        <table>
            <tr>
                <th>Metric</th>
                <th>Min</th>
                <th>Avg</th>
                <th>Max</th>
                <th>P95</th>
                <th>P99</th>
            </tr>
            <tr>
                <td>Duration (ms)</td>
                <td>{performance['duration_ms']['min']:.2f}</td>
                <td>{performance['duration_ms']['avg']:.2f}</td>
                <td>{performance['duration_ms']['max']:.2f}</td>
                <td>{performance['duration_ms']['p95']:.2f}</td>
                <td>{performance['duration_ms']['p99']:.2f}</td>
            </tr>
            <tr>
                <td>CPU (%)</td>
                <td>{performance['cpu_percent']['min']:.2f}</td>
                <td>{performance['cpu_percent']['avg']:.2f}</td>
                <td>{performance['cpu_percent']['max']:.2f}</td>
                <td>-</td>
                <td>-</td>
            </tr>
            <tr>
                <td>Memory (MB)</td>
                <td>{performance['memory_mb']['min']:.2f}</td>
                <td>{performance['memory_mb']['avg']:.2f}</td>
                <td>{performance['memory_mb']['max']:.2f}</td>
                <td>-</td>
                <td>-</td>
            </tr>
        </table>
        
        <h2>❌ Error Analysis</h2>
        <p>Total Errors: <span class="error">{errors['total_errors']}</span></p>
        
        <h3>Error Types</h3>
        <table>
            <tr>
                <th>Error Message</th>
                <th>Count</th>
            </tr>
"""
        
        for error_msg, count in errors['error_types']:
            html += f"""
            <tr>
                <td>{error_msg}</td>
                <td>{count}</td>
            </tr>
"""
        
        html += """
        </table>
        
        <p style="margin-top: 40px; text-align: center; color: #7f8c8d;">
            RAFAELIA Framework v1.0.0 | ∆RafaelVerboΩ<br>
            VAZIO → VERBO → CHEIO → RETRO
        </p>
    </div>
</body>
</html>
"""
        
        with open(output_file, 'w') as f:
            f.write(html)
            
        print(f"Report generated: {output_file}")

def main():
    parser = argparse.ArgumentParser(description='RAFAELIA Audit Analyzer')
    parser.add_argument('--input', nargs='+', required=True, help='Audit log files (*.jsonl)')
    parser.add_argument('--output', default='audit_report.html', help='Output HTML file')
    
    args = parser.parse_args()
    
    analyzer = AuditAnalyzer()
    
    # Load audit logs
    log_files = []
    for pattern in args.input:
        log_files.extend(Path('.').glob(pattern))
        
    if not log_files:
        print("No audit log files found!", file=sys.stderr)
        sys.exit(1)
        
    analyzer.load_audit_logs(log_files)
    
    # Generate report
    analyzer.generate_html_report(Path(args.output))
    
    # Print summary to console
    summary = analyzer.generate_summary()
    print("\n=== Summary ===")
    print(f"Total operations: {summary['total_operations']}")
    print(f"Success rate: {summary['success_rate']:.1f}%")
    print(f"Errors: {summary['errors']}")
    print(f"Warnings: {summary['warnings']}")

if __name__ == '__main__':
    main()
