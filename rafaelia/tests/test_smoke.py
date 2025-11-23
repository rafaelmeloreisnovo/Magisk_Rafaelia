#!/usr/bin/env python3
"""
RAFAELIA TT Fullstack - Smoke Tests  Basic smoke tests to verify core functionality of RAFAELIA TT suite. Uses small tensor shapes for fast execution.  Part of RAFAELIA Fullstack Suite Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ Philosophy: VAZIO → VERBO → CHEIO → RETRO 

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

⚓ ANCHOR_ID: 10D1F4196D6F3C7F
⚓ FILE_PATH: rafaelia/tests/test_smoke.py
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 32BA31EC450AF8AF27ED66E9921E1000


"""


import sys
import os
import unittest
import numpy as np
from pathlib import Path

# Add parent directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent))

from RAFAELIA_TT_CROSS_FULL import TTCrossApproximation
from RAFAELIA_TT_UPDATE_FULL import TTLocalUpdate
from RAFAELIA_ENGINE_FULLSTACK import RAFAELIAEngine
from RAFAELIA_SPIRAL_FIBONACCI import FibonacciSpiral, GoldenRatioSampler
from RAFAELIA_TT_ACCEL import TTAccelerator, TTCache, PerformanceProfiler


class TestTTCross(unittest.TestCase):
    """Test TT-cross approximation."""
    
    def test_initialization(self):
        """Test TT-cross initialization."""
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        tt = TTCrossApproximation(shape, ranks, use_gpu=False)
        
        self.assertEqual(tt.d, 3)
        self.assertEqual(tt.shape, shape)
        self.assertEqual(len(tt.cores), 3)
    
    def test_evaluation(self):
        """Test TT evaluation at point."""
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        tt = TTCrossApproximation(shape, ranks, use_gpu=False)
        
        indices = [1, 2, 3]
        value = tt.evaluate(indices)
        
        self.assertIsInstance(value, float)
    
    def test_approximation(self):
        """Test TT-cross approximation of simple function."""
        def test_func(indices):
            return sum(indices) * 0.5
        
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        tt = TTCrossApproximation(shape, ranks, use_gpu=False, epsilon=1e-3)
        stats = tt.cross_approximation(test_func, max_iter=5, verbose=False)
        
        self.assertIn('iterations', stats)
        self.assertIn('error', stats)
        self.assertIn('converged', stats)
        self.assertGreater(stats['iterations'], 0)
    
    def test_checkpoint_save(self):
        """Test checkpoint saving."""
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        tt = TTCrossApproximation(shape, ranks, use_gpu=False)
        
        checkpoint_path = "/tmp/test_tt_cross_checkpoint.json"
        tt.save_checkpoint(checkpoint_path, metadata={'test': True})
        
        self.assertTrue(
            os.path.exists(checkpoint_path) or 
            os.path.exists(checkpoint_path + '.zst')
        )
    
    def test_checkpoint_load(self):
        """Test checkpoint loading."""
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        tt = TTCrossApproximation(shape, ranks, use_gpu=False)
        
        checkpoint_path = "/tmp/test_tt_cross_load.json"
        tt.save_checkpoint(checkpoint_path)
        
        # Load checkpoint
        tt_loaded = TTCrossApproximation.load_checkpoint(checkpoint_path)
        
        self.assertEqual(tt_loaded.shape, shape)
        self.assertEqual(len(tt_loaded.cores), 3)


class TestTTUpdate(unittest.TestCase):
    """Test TT local updates."""
    
    def test_initialization(self):
        """Test TTLocalUpdate initialization."""
        cores = [
            np.random.randn(1, 3, 2),
            np.random.randn(2, 4, 2),
            np.random.randn(2, 5, 1)
        ]
        
        tt_update = TTLocalUpdate(cores, use_gpu=False)
        
        self.assertEqual(tt_update.d, 3)
        self.assertEqual(tt_update.shape, [3, 4, 5])
    
    def test_core_update(self):
        """Test single core update."""
        cores = [
            np.random.randn(1, 3, 2) * 0.1,
            np.random.randn(2, 4, 2) * 0.1,
            np.random.randn(2, 5, 1) * 0.1
        ]
        
        tt_update = TTLocalUpdate(cores, use_gpu=False)
        
        target_indices = [(1, 2, 3), (0, 1, 2)]
        target_values = [1.0, 2.0]
        
        # Should not raise error
        tt_update.update_core(1, target_indices, target_values, learning_rate=0.01)
    
    def test_als_sweep(self):
        """Test ALS sweep."""
        cores = [
            np.random.randn(1, 3, 2) * 0.1,
            np.random.randn(2, 4, 2) * 0.1,
            np.random.randn(2, 5, 1) * 0.1
        ]
        
        tt_update = TTLocalUpdate(cores, use_gpu=False)
        
        target_data = {
            (1, 2, 3): 1.0,
            (0, 1, 2): 2.0,
            (2, 3, 4): 1.5
        }
        
        stats = tt_update.als_sweep(target_data, n_iterations=3, verbose=False)
        
        self.assertIn('iterations', stats)
        self.assertIn('final_error', stats)
        self.assertEqual(stats['iterations'], 3)
    
    def test_checkpoint_save(self):
        """Test checkpoint saving."""
        cores = [
            np.random.randn(1, 3, 2),
            np.random.randn(2, 4, 2),
            np.random.randn(2, 5, 1)
        ]
        
        tt_update = TTLocalUpdate(cores, use_gpu=False)
        
        checkpoint_path = "/tmp/test_tt_update_checkpoint.json"
        tt_update.save_checkpoint(checkpoint_path)
        
        self.assertTrue(os.path.exists(checkpoint_path))


class TestEngine(unittest.TestCase):
    """Test RAFAELIA engine."""
    
    def test_initialization(self):
        """Test engine initialization."""
        config = {
            'use_gpu': False,
            'checkpoint_dir': '/tmp/test_checkpoints',
            'auto_checkpoint': False
        }
        
        engine = RAFAELIAEngine(config)
        
        self.assertFalse(engine.use_gpu)
        self.assertFalse(engine.auto_checkpoint)
    
    def test_approximate_tensor(self):
        """Test tensor approximation."""
        def test_func(indices):
            return sum(indices)
        
        config = {'use_gpu': False, 'auto_checkpoint': False}
        engine = RAFAELIAEngine(config)
        
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        stats = engine.approximate_tensor(
            test_func, shape, ranks, max_iter=3, verbose=False
        )
        
        self.assertIn('iterations', stats)
        self.assertIn('elapsed_time', stats)
    
    def test_update_tensor(self):
        """Test tensor update."""
        def test_func(indices):
            return sum(indices)
        
        config = {'use_gpu': False, 'auto_checkpoint': False}
        engine = RAFAELIAEngine(config)
        
        # First approximate
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        engine.approximate_tensor(test_func, shape, ranks, max_iter=2, verbose=False)
        
        # Then update
        target_data = {(1, 2, 3): 5.0, (0, 1, 2): 3.0}
        stats = engine.update_tensor(target_data, n_iterations=2, verbose=False)
        
        self.assertIn('final_error', stats)
    
    def test_evaluate(self):
        """Test evaluation."""
        def test_func(indices):
            return sum(indices)
        
        config = {'use_gpu': False, 'auto_checkpoint': False}
        engine = RAFAELIAEngine(config)
        
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        engine.approximate_tensor(test_func, shape, ranks, max_iter=2, verbose=False)
        
        value = engine.evaluate([1, 2, 3])
        self.assertIsInstance(value, float)
    
    def test_generate_manifest(self):
        """Test manifest generation."""
        config = {'use_gpu': False, 'auto_checkpoint': False}
        engine = RAFAELIAEngine(config)
        
        manifest_path = "/tmp/test_manifest.json"
        manifest = engine.generate_manifest(manifest_path)
        
        self.assertIn('signature', manifest)
        self.assertIn('philosophy', manifest)
        self.assertIn('hashes', manifest)
        self.assertTrue(os.path.exists(manifest_path))


class TestFibonacciSpiral(unittest.TestCase):
    """Test Fibonacci spiral generator."""
    
    def test_initialization(self):
        """Test spiral initialization."""
        spiral = FibonacciSpiral(dimension=3, shape=[5, 6, 7])
        
        self.assertEqual(spiral.dimension, 3)
        self.assertEqual(spiral.shape, [5, 6, 7])
    
    def test_generate_points(self):
        """Test point generation."""
        spiral = FibonacciSpiral(dimension=3, shape=[5, 6, 7])
        
        points = spiral.generate_points(n_points=10)
        
        self.assertEqual(points.shape, (10, 3))
        # Check points are within bounds
        for i in range(3):
            self.assertTrue(np.all(points[:, i] >= 0))
            self.assertTrue(np.all(points[:, i] < spiral.shape[i]))
    
    def test_fibonacci_lattice(self):
        """Test Fibonacci lattice."""
        spiral = FibonacciSpiral(dimension=3, shape=[5, 6, 7])
        
        lattice = spiral.fibonacci_lattice(n_points=15)
        
        self.assertEqual(lattice.shape, (15, 3))
    
    def test_spherical_fibonacci(self):
        """Test spherical Fibonacci."""
        spiral = FibonacciSpiral(dimension=3, shape=[10, 10, 10])
        
        sphere_points = spiral.spherical_fibonacci(n_points=20)
        
        self.assertEqual(sphere_points.shape, (20, 3))
        
        # Check points are on unit sphere (approximately)
        norms = np.linalg.norm(sphere_points, axis=1)
        self.assertTrue(np.allclose(norms, 1.0, atol=1e-10))


class TestGoldenRatioSampler(unittest.TestCase):
    """Test golden ratio sampler."""
    
    def test_sample(self):
        """Test sampling."""
        sampler = GoldenRatioSampler(seed=42)
        
        samples = sampler.sample(n=10, dimension=2, bounds=[(0, 10), (0, 5)])
        
        self.assertEqual(samples.shape, (10, 2))
        # Check bounds
        self.assertTrue(np.all(samples[:, 0] >= 0))
        self.assertTrue(np.all(samples[:, 0] <= 10))
        self.assertTrue(np.all(samples[:, 1] >= 0))
        self.assertTrue(np.all(samples[:, 1] <= 5))


class TestAccelerator(unittest.TestCase):
    """Test TT accelerator."""
    
    def test_initialization(self):
        """Test accelerator initialization."""
        accel = TTAccelerator(use_gpu=False, use_cache=True, cache_size=100)
        
        self.assertFalse(accel.use_gpu)
        self.assertTrue(accel.use_cache)
        self.assertIsNotNone(accel.cache)
    
    def test_batch_evaluate(self):
        """Test batch evaluation."""
        cores = [
            np.random.randn(1, 3, 2),
            np.random.randn(2, 4, 2),
            np.random.randn(2, 5, 1)
        ]
        
        accel = TTAccelerator(use_gpu=False, use_cache=False)
        
        indices_batch = np.array([[0, 1, 2], [1, 2, 3], [2, 3, 4]])
        values = accel.batch_evaluate(cores, indices_batch)
        
        self.assertEqual(values.shape, (3,))
    
    def test_cache(self):
        """Test cache functionality."""
        cache = TTCache(max_size=10)
        
        cache.put((1, 2, 3), 42.0)
        value = cache.get((1, 2, 3))
        
        self.assertEqual(value, 42.0)
        
        stats = cache.stats()
        self.assertEqual(stats['hits'], 1)
    
    def test_profiler(self):
        """Test performance profiler."""
        profiler = PerformanceProfiler()
        
        profiler.start_operation('test_op')
        # Simulate work
        sum([i**2 for i in range(1000)])
        profiler.end_operation()
        
        report = profiler.get_report()
        self.assertEqual(report['total_operations'], 1)
        self.assertIn('test_op', report['by_operation'])


class TestIntegration(unittest.TestCase):
    """Integration tests."""
    
    def test_full_workflow(self):
        """Test complete workflow from approximation to update."""
        # Define function
        def test_func(indices):
            return sum(indices) * 0.5 + np.prod(indices) * 0.1
        
        # Initialize engine
        config = {'use_gpu': False, 'auto_checkpoint': False}
        engine = RAFAELIAEngine(config)
        
        # Approximate
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        approx_stats = engine.approximate_tensor(
            test_func, shape, ranks, max_iter=3, verbose=False
        )
        
        self.assertIn('iterations', approx_stats)
        
        # Update
        target_data = {
            (1, 2, 3): test_func([1, 2, 3]) + 0.1,
            (0, 1, 2): test_func([0, 1, 2]) - 0.1
        }
        update_stats = engine.update_tensor(target_data, n_iterations=2, verbose=False)
        
        self.assertIn('final_error', update_stats)
        
        # Evaluate
        value = engine.evaluate([1, 2, 3])
        self.assertIsInstance(value, float)
        
        # Generate manifest
        manifest = engine.generate_manifest()
        
        self.assertIn('signature', manifest)
        self.assertEqual(len(manifest['metadata']['operations']), 2)
    
    def test_checkpoint_and_hash_generation(self):
        """Test that checkpoints are created with hashes."""
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        tt = TTCrossApproximation(shape, ranks, use_gpu=False)
        
        checkpoint_path = "/tmp/test_hash_checkpoint.json"
        tt.save_checkpoint(checkpoint_path, metadata={'test': 'hashes'})
        
        # Check file exists
        self.assertTrue(
            os.path.exists(checkpoint_path) or 
            os.path.exists(checkpoint_path + '.zst')
        )
        
        # Load and verify structure
        import json
        if os.path.exists(checkpoint_path):
            with open(checkpoint_path, 'r') as f:
                data = json.load(f)
            
            self.assertIn('hashes', data)
            self.assertIn('sha256', data['hashes'])
            self.assertIn('rafaelia', data)


def run_smoke_tests():
    """Run all smoke tests."""
    print("=" * 60)
    print("RAFAELIA TT Fullstack - Smoke Tests")
    print("=" * 60)
    print()
    
    # Create test suite
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    
    # Add all test classes
    suite.addTests(loader.loadTestsFromTestCase(TestTTCross))
    suite.addTests(loader.loadTestsFromTestCase(TestTTUpdate))
    suite.addTests(loader.loadTestsFromTestCase(TestEngine))
    suite.addTests(loader.loadTestsFromTestCase(TestFibonacciSpiral))
    suite.addTests(loader.loadTestsFromTestCase(TestGoldenRatioSampler))
    suite.addTests(loader.loadTestsFromTestCase(TestAccelerator))
    suite.addTests(loader.loadTestsFromTestCase(TestIntegration))
    
    # Run tests
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    print()
    print("=" * 60)
    print("Test Summary")
    print("=" * 60)
    print(f"Tests run: {result.testsRun}")
    print(f"Successes: {result.testsRun - len(result.failures) - len(result.errors)}")
    print(f"Failures: {len(result.failures)}")
    print(f"Errors: {len(result.errors)}")
    print()
    
    if result.wasSuccessful():
        print("✅ All tests passed!")
        print()
        print("RAFAELIA Philosophy: VAZIO → VERBO → CHEIO → RETRO")
        return 0
    else:
        print("❌ Some tests failed")
        return 1


if __name__ == '__main__':
    sys.exit(run_smoke_tests())
