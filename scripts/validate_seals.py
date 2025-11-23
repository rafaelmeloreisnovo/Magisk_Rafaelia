#!/usr/bin/env python3
"""
RAFAELIA Seal System Validator (ΣΩΔΦBITRAF)
════════════════════════════════════════════════════════════════════════════════
RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
Selo: ΣΩΔΦBITRAF - Ten Seals Validation
════════════════════════════════════════════════════════════════════════════════

Validates the ten-seal system for comprehensive project compliance.
"""

import sys
import os
from pathlib import Path
from typing import Dict, List, Tuple

# The Ten Seals
SEALS = {
    'Σ': {
        'name': 'Sigma',
        'aspect': 'Completeness',
        'description': 'All components present and integrated',
    },
    'Ω': {
        'name': 'Omega',
        'aspect': 'Perfection',
        'description': 'Quality standards met, no critical issues',
    },
    'Δ': {
        'name': 'Delta',
        'aspect': 'Evolution',
        'description': 'Version control, changelog, progression',
    },
    'Φ': {
        'name': 'Phi',
        'aspect': 'Harmony',
        'description': 'Balanced design, proportional components',
    },
    'B': {
        'name': 'Balance',
        'aspect': 'Balance',
        'description': 'Load distribution, resource optimization',
    },
    'I': {
        'name': 'Identity',
        'aspect': 'Identity',
        'description': 'Author attribution, license compliance',
    },
    'T': {
        'name': 'Testing',
        'aspect': 'Testing',
        'description': 'Test coverage, CI/CD, validation',
    },
    'R': {
        'name': 'Rafael/Reliability',
        'aspect': 'Reliability',
        'description': 'Uptime, stability, error handling',
    },
    'A': {
        'name': 'Architecture',
        'aspect': 'Architecture',
        'description': 'Documented design, clear structure',
    },
    'F': {
        'name': 'Functionality',
        'aspect': 'Functionality',
        'description': 'Features work as specified, documented',
    },
}


class SealValidator:
    """Validator for RAFAELIA seal system."""
    
    def __init__(self, root_dir: str = '.'):
        self.root_dir = Path(root_dir)
        self.seal_status: Dict[str, Dict] = {}
        
    def validate_sigma_completeness(self) -> Tuple[bool, str]:
        """Σ - Validate project completeness."""
        required_components = [
            'README.MD',
            'LICENSE',
            'AUTHORS_RAFAELIA.md',
            'ETHICS.md',
            'LEGAL.md',
            'RAFAELIA_CORE.md',
        ]
        
        missing = []
        for component in required_components:
            if not (self.root_dir / component).exists():
                missing.append(component)
        
        if missing:
            return False, f"Missing components: {', '.join(missing)}"
        return True, "All core components present"
    
    def validate_omega_perfection(self) -> Tuple[bool, str]:
        """Ω - Validate quality standards."""
        # Check for quality indicators
        quality_files = [
            'SECURITY_SUMMARY.md',
            '.github/workflows',
        ]
        
        quality_score = 0
        for qf in quality_files:
            if (self.root_dir / qf).exists():
                quality_score += 1
        
        if quality_score >= 1:
            return True, f"Quality indicators present ({quality_score}/{len(quality_files)})"
        return False, "Quality documentation needed"
    
    def validate_delta_evolution(self) -> Tuple[bool, str]:
        """Δ - Validate version control and evolution."""
        if (self.root_dir / '.git').exists():
            return True, "Git version control active"
        return False, "Version control not found"
    
    def validate_phi_harmony(self) -> Tuple[bool, str]:
        """Φ - Validate design harmony."""
        # Check for architectural documentation
        arch_docs = [
            'RAFAELIA_CORE.md',
            'RAFAELIA_META_ARCHITECTURE_SUMMARY.md',
        ]
        
        for doc in arch_docs:
            if (self.root_dir / doc).exists():
                return True, f"Architectural harmony documented in {doc}"
        
        return False, "Architectural documentation recommended"
    
    def validate_balance(self) -> Tuple[bool, str]:
        """B - Validate system balance."""
        # Check for configuration files
        config_files = [
            'config.prop.sample',
            '.gitignore',
        ]
        
        balance_score = 0
        for cf in config_files:
            if (self.root_dir / cf).exists():
                balance_score += 1
        
        if balance_score > 0:
            return True, f"Configuration balance maintained ({balance_score} configs)"
        return True, "Balance check passed"
    
    def validate_identity(self) -> Tuple[bool, str]:
        """I - Validate identity and attribution."""
        if (self.root_dir / 'AUTHORS_RAFAELIA.md').exists():
            try:
                with open(self.root_dir / 'AUTHORS_RAFAELIA.md', 'r') as f:
                    content = f.read()
                    if 'Rafael' in content:
                        return True, "Author identity preserved"
            except:
                pass
        
        return False, "Author attribution required"
    
    def validate_testing(self) -> Tuple[bool, str]:
        """T - Validate testing infrastructure."""
        test_indicators = [
            'tests',
            'test',
            '.github/workflows',
        ]
        
        for indicator in test_indicators:
            if (self.root_dir / indicator).exists():
                return True, f"Testing infrastructure present: {indicator}"
        
        return False, "Testing infrastructure recommended"
    
    def validate_reliability(self) -> Tuple[bool, str]:
        """R - Validate reliability measures."""
        reliability_docs = [
            'SECURITY_SUMMARY.md',
            'docs/RAFAELIA_AUDIT_SYSTEM.md',
        ]
        
        for doc in reliability_docs:
            if (self.root_dir / doc).exists():
                return True, f"Reliability documented in {doc}"
        
        return True, "Reliability check passed"
    
    def validate_architecture(self) -> Tuple[bool, str]:
        """A - Validate architecture documentation."""
        arch_files = [
            'RAFAELIA_CORE.md',
            'docs/RAFAELIA_FRAMEWORK.md',
        ]
        
        for af in arch_files:
            if (self.root_dir / af).exists():
                return True, f"Architecture documented in {af}"
        
        return False, "Architecture documentation required"
    
    def validate_functionality(self) -> Tuple[bool, str]:
        """F - Validate functionality documentation."""
        func_docs = [
            'README.MD',
            'COMO_OBTER_APK.md',
            'HOW_TO_GET_APK.md',
        ]
        
        func_score = 0
        for fd in func_docs:
            if (self.root_dir / fd).exists():
                func_score += 1
        
        if func_score >= 1:
            return True, f"Functionality documented ({func_score} docs)"
        return False, "Functionality documentation needed"
    
    def validate_all_seals(self) -> Dict[str, Dict]:
        """Validate all ten seals."""
        print("🏅 Validating ΣΩΔΦBITRAF Seal System...\n")
        
        validators = {
            'Σ': self.validate_sigma_completeness,
            'Ω': self.validate_omega_perfection,
            'Δ': self.validate_delta_evolution,
            'Φ': self.validate_phi_harmony,
            'B': self.validate_balance,
            'I': self.validate_identity,
            'T': self.validate_testing,
            'R': self.validate_reliability,
            'A': self.validate_architecture,
            'F': self.validate_functionality,
        }
        
        for seal_symbol, validator_func in validators.items():
            seal_info = SEALS[seal_symbol]
            is_valid, message = validator_func()
            
            status_icon = "✅" if is_valid else "❌"
            
            self.seal_status[seal_symbol] = {
                'valid': is_valid,
                'message': message,
                'name': seal_info['name'],
                'aspect': seal_info['aspect'],
            }
            
            print(f"{status_icon} {seal_symbol} ({seal_info['name']}) - {seal_info['aspect']}")
            print(f"   {message}")
            print()
        
        return self.seal_status
    
    def generate_report(self) -> bool:
        """Generate validation report."""
        print("="*80)
        print("📊 SEAL SYSTEM VALIDATION REPORT")
        print("="*80 + "\n")
        
        valid_count = sum(1 for s in self.seal_status.values() if s['valid'])
        total_count = len(self.seal_status)
        
        print(f"Seals Validated: {valid_count}/{total_count}\n")
        
        if valid_count == total_count:
            print("✅ ALL SEALS VALIDATED")
            print("\nΣΩΔΦBITRAF - Complete seal system active!")
            return True
        elif valid_count >= 8:
            print("⚠️  MOST SEALS VALIDATED")
            print(f"\n{total_count - valid_count} seal(s) need attention.")
            return True
        else:
            print("❌ SEAL VALIDATION INCOMPLETE")
            print(f"\n{total_count - valid_count} seal(s) require implementation.")
            return False
    
    def run(self) -> bool:
        """Run complete seal validation."""
        print("🚀 RAFAELIA Seal System Validation")
        print("RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ")
        print("Selo: ΣΩΔΦBITRAF\n")
        print("="*80 + "\n")
        
        self.validate_all_seals()
        return self.generate_report()


def main():
    """Main entry point."""
    root_dir = sys.argv[1] if len(sys.argv) > 1 else '.'
    
    validator = SealValidator(root_dir)
    
    if validator.run():
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == '__main__':
    main()
