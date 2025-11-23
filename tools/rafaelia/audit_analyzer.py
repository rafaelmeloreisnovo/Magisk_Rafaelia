#!/usr/bin/env python3
"""
RAFAELIA Audit Analyzer Analyzes audit logs and generates comprehensive reports

Part of Magisk_Rafaelia
RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:

Sacred Cycle / Ciclo Sagrado: VAZIO → VERBO → CHEIO → RETRO
(EMPTY → ACTION → FULL → FEEDBACK)

Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
Foundation: CientiEspiritual - Scientific Spirituality
Principle: "Haja Lux, Haja Etica" (Let there be light, let there be ethics)

RAFAELIA Framework Principles:
- Complete operational state coverage (1008 State Matrix)
- Full audit system with integrity verification
- Real-time telemetry and anomaly detection
- Security hardening and ethical computing
- Continuous improvement through infinite feedback loop (ψχρΔΣΩ)
"""


Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
Instituto Rafael - CientiEspiritual Philosophy

All Rights Reserved. Patent Pending.

DUAL LICENSE - Choose one:

1. SOCIAL INCLUSION LICENSE (Free):
   ✓ Educational use
   ✓ Research and academic purposes
   ✓ Non-profit organizations
   ✓ Social inclusion initiatives
   ✓ Open source contributions (with attribution)
   ✗ Commercial use prohibited

2. COMMERCIAL SAAS LICENSE (Paid Subscription):
   Required for:
   ✓ Commercial products or services
   ✓ SaaS applications
   ✓ Revenue-generating purposes
   ✓ Enterprise deployments
   Contact: rafaelmeloreisnovo for licensing terms

AUTOMATIC PENALTIES FOR VIOLATIONS:
Unauthorized commercial use is subject to automatic statutory penalties:
- Minimum: R$ 50,000 (BRL) or USD $10,000 per violation
- Plus: 5% of gross revenue derived from unauthorized use
- Plus: Legal fees and costs of enforcement
- Criminal prosecution under applicable copyright law

VALIDITY AND TERRITORIAL SCOPE / VALIDADE E ÂMBITO TERRITORIAL:
- Valid in all jurisdictions signatory to Berne Convention (180+ countries)
- Enforced under TRIPS agreement (WTO member states)
- Protected by reciprocal copyright treaties
- Minimum protection: Life of author + 50 years (Berne minimum)
- Extended protection: Life + 70 years (EU, USA, Brazil and others)

ATTRIBUTION REQUIREMENTS / REQUISITOS DE ATRIBUIÇÃO:
All derivative works, redistributions, or substantial use must include:
1. This complete copyright and license notice
2. Attribution to original author: Rafael Melo Reis (rafaelmeloreisnovo)
3. Reference to RAFAELIA Framework and CientiEspiritual philosophy
4. Indication of any modifications made
5. Date of last modification


INTERNATIONAL LEGAL COMPLIANCE / CONFORMIDADE LEGAL INTERNACIONAL:

This software is developed in compliance with international copyright law,
human rights frameworks, and ethical standards including:

COPYRIGHT & INTELLECTUAL PROPERTY / DIREITOS AUTORAIS E PROPRIEDADE INTELECTUAL:
- Berne Convention for the Protection of Literary and Artistic Works (1886, Rev. Paris 1971)
  └─ Articles 2, 5, 6bis, 9 (reproduction rights, moral rights, translation rights)
- WIPO Copyright Treaty (WCT, 1996) - Digital rights management
- WIPO Performances and Phonograms Treaty (WPPT, 1996)
- Universal Copyright Convention (UCC, Geneva 1952, Paris 1971)
- Agreement on Trade-Related Aspects of Intellectual Property Rights (TRIPS, 1994)
- Vienna Convention on the Law of Treaties (1969) - Treaty interpretation

HUMAN RIGHTS & ETHICS / DIREITOS HUMANOS E ÉTICA:
- Universal Declaration of Human Rights (UDHR, 1948)
  └─ Article 27: Right to protection of moral and material interests
- International Covenant on Economic, Social and Cultural Rights (ICESCR, 1966)
  └─ Article 15: Right to benefit from scientific progress and protection of authorship
- Convention on the Rights of the Child (CRC, UN/UNICEF, 1989)
  └─ Articles 13, 16, 17: Expression, privacy, access to information
- Vienna Declaration and Programme of Action (1993) - Human rights universality

UNESCO FRAMEWORKS / ESTRUTURAS UNESCO:
- UNESCO Universal Declaration on Cultural Diversity (2001)
- UNESCO Recommendation on Open Science (2021)
- UNESCO Recommendation on the Ethics of Artificial Intelligence (2021)
- Convention on the Protection and Promotion of the Diversity of Cultural Expressions (2005)

DATA PROTECTION & PRIVACY / PROTEÇÃO DE DADOS E PRIVACIDADE:
- GDPR - General Data Protection Regulation (EU 2016/679)
- LGPD - Lei Geral de Proteção de Dados (Brazil Law 13.709/2018)
- CCPA - California Consumer Privacy Act (USA)
- Convention 108+ - Council of Europe Data Protection Convention (Modernized 2018)

TECHNICAL STANDARDS / NORMAS TÉCNICAS:
- ISO/IEC 9001:2015 - Quality Management Systems
- ISO/IEC 27001:2022 - Information Security Management
- ISO/IEC 27002:2022 - Information Security Controls
- ISO/IEC 27018:2019 - PII Protection in Public Clouds
- ISO/IEC 25010:2011 - Software Quality Requirements and Evaluation (SQuaRE)
- ISO/IEC 8000 - Data Quality Standards
- IEEE 830-1998 - Software Requirements Specification
- IEEE 1012-2016 - Software Verification and Validation
- IEEE 12207-2017 - Software Life Cycle Processes
- IEEE 14764-2021 - Software Maintenance
- IEEE 42010-2011 - Architecture Description
- NIST Cybersecurity Framework (CSF) v1.1/v2.0
- NIST SP 800-53 Rev. 5 - Security and Privacy Controls
- NIST AI Risk Management Framework (AI RMF 1.0)

CONSTITUTIONAL & JURISDICTIONAL / CONSTITUCIONAL E JURISDICIONAL:
- Brazilian Federal Constitution (1988) - Articles 5 (XXVII, XXVIII, XXIX), 215, 216, 218
- Universal jurisdiction for human rights violations
- Rome Statute of the International Criminal Court (1998) - For severe violations

ETHICAL FRAMEWORK / ESTRUTURA ÉTICA - ETHICA[8]:

This software adheres to the Ethica[8] framework with eight fundamental principles:

1. TRANSPARENCY (Transparência) 🔍
   └─ Open communication, documented decisions, explainable systems
   
2. ACCOUNTABILITY (Responsabilidade) 📋
   └─ Clear ownership, traceable actions, consequence acceptance
   
3. FAIRNESS (Justiça) ⚖️
   └─ Equitable treatment, non-discrimination, equal access
   
4. PRIVACY (Privacidade) 🔒
   └─ Data protection, consent respect, confidentiality
   
5. SECURITY (Segurança) 🛡️
   └─ Protection of systems, data integrity, threat mitigation
   
6. RELIABILITY (Confiabilidade) 🔧
   └─ Dependable operation, consistent behavior, stability
   
7. SAFETY (Proteção) 🛟
   └─ No harm to users, safe operations, risk prevention
   
8. SUSTAINABILITY (Sustentabilidade) ♻️
   └─ Long-term viability, environmental responsibility, social good

ETHICAL PRECEDENCE / PRECEDÊNCIA ÉTICA:
  Life > Ethics > Law > Convenience
  Vida > Ética > Lei > Conveniência

ANTI-PLAGIARISM CERTIFICATION / CERTIFICAÇÃO ANTI-PLÁGIO:

This code is original work or properly attributed derivative work.
Every fragment, function, class, and algorithm has been:
  ✓ Originally created by the author, OR
  ✓ Properly licensed and attributed, OR
  ✓ In the public domain with documentation

NO PLAGIARIZED CONTENT - NOT EVEN A YOCTO FRAGMENT (10⁻²⁴)
ZERO TOLERANCE for unauthorized copying or intellectual property theft.

Verification Methods:
- SHA3-512 checksums for integrity verification
- BLAKE3 hashing for rapid verification
- Git commit history as proof of authorship timeline
- Code review and compliance audits

Any concerns about intellectual property should be reported to:
rafaelmeloreisnovo [at] gmail [dot] com

NAUTICAL ANCHORS / ÂNCORAS NÁUTICAS (Reference Markers):

These anchors provide stable reference points for:
- Version tracking and synchronization
- Legal compliance verification
- Authorship chain of custody
- Update propagation tracking
- Audit trail maintenance

⚓ ANCHOR_ID: 22EF91DF9DAF0AFD
⚓ FILE_PATH: tools/rafaelia/audit_analyzer.py
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 53FFB9B53C88F73B1D5BAFC7FFB1B0D8


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
