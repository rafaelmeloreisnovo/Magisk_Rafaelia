#!/usr/bin/env python3
"""
═══════════════════════════════════════════════════════════════════════════════ RAFAELIA MEMORY OPTIMIZATION - Integration Tests ═══════════════════════════════════════════════════════════════════════════════  Comprehensive test suite for memory optimization modules: - Fractal memory allocation - ECC buffer validation (via ctypes FFI) - Entropy analysis (via PyO3 FFI if available) - Planck-scale memory pooling

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

⚓ ANCHOR_ID: 7F0FA9DD2F0815E1
⚓ FILE_PATH: rafaelia/tests/test_memory_optimization.py
⚓ CREATION_DATE: 2025-11-23
⚓ LAST_MODIFIED: 2025-11-23
⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
⚓ ETHICA_VERSION: Ethica[8]_v1.0
⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
⚓ INTEGRITY_HASH: 26AC801089EA89EAAB2C64645DE3C35E


"""


# -*- coding: utf-8 -*-
"""
═══════════════════════════════════════════════════════════════════════════════
RAFAELIA MEMORY OPTIMIZATION - Integration Tests
═══════════════════════════════════════════════════════════════════════════════

Comprehensive test suite for memory optimization modules:
- Fractal memory allocation
- ECC buffer validation (via ctypes FFI)
- Entropy analysis (via PyO3 FFI if available)
- Planck-scale memory pooling
- Matrix fractal operations

Part of RAFAELIA Framework - Testing Suite
Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved.
═══════════════════════════════════════════════════════════════════════════════
"""

import unittest
import numpy as np
import sys
import os

# Add parent directory to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../..'))

from rafaelia.utils.fractal_memory import (
    HilbertCurve, ZOrderCurve, FibonacciSpiral, FractalMemoryAllocator
)
from rafaelia.utils.planck_memory import (
    PlanckMemoryPool, AllocationStrategy
)
from rafaelia.core.matrix_ops import FractalMatrixOptimizer, MatrixOperations


class TestHilbertCurve(unittest.TestCase):
    """Test Hilbert space-filling curve operations."""
    
    def test_xy_to_d_bijection(self):
        """Test that xy_to_d is bijective (one-to-one)."""
        n = 8
        seen_distances = set()
        
        for x in range(n):
            for y in range(n):
                d = HilbertCurve.xy_to_d(n, x, y)
                self.assertNotIn(d, seen_distances, 
                               f"Distance {d} already seen for ({x}, {y})")
                seen_distances.add(d)
        
        self.assertEqual(len(seen_distances), n * n)
    
    def test_d_to_xy_inverse(self):
        """Test that d_to_xy is inverse of xy_to_d."""
        n = 16
        
        for d in range(n * n):
            x, y = HilbertCurve.d_to_xy(n, d)
            d_check = HilbertCurve.xy_to_d(n, x, y)
            self.assertEqual(d, d_check, 
                           f"Inverse failed: d={d} -> ({x},{y}) -> {d_check}")
    
    def test_locality_preservation(self):
        """Test that nearby points in curve are nearby in space."""
        n = 32
        
        # Sample consecutive points
        for d in range(0, n * n - 1, 10):
            x1, y1 = HilbertCurve.d_to_xy(n, d)
            x2, y2 = HilbertCurve.d_to_xy(n, d + 1)
            
            # Manhattan distance should be 1 for most consecutive points
            manhattan = abs(x2 - x1) + abs(y2 - y1)
            self.assertLessEqual(manhattan, 2, 
                               f"Poor locality at d={d}: ({x1},{y1}) to ({x2},{y2})")


class TestZOrderCurve(unittest.TestCase):
    """Test Z-order (Morton) curve operations."""
    
    def test_encode_decode_inverse(self):
        """Test that decode is inverse of encode."""
        test_coords = [(0, 0), (1, 2), (15, 15), (100, 200), (255, 255)]
        
        for x, y in test_coords:
            z = ZOrderCurve.encode(x, y)
            x_dec, y_dec = ZOrderCurve.decode(z)
            self.assertEqual((x, y), (x_dec, y_dec),
                           f"Inverse failed: ({x},{y}) -> {z} -> ({x_dec},{y_dec})")
    
    def test_interleaving(self):
        """Test bit interleaving property."""
        x, y = 0b1010, 0b1100
        z = ZOrderCurve.encode(x, y)
        
        # Expected: y1x1y0x0 = 1 1 1 0 0 0 1 0 = 0b11100010 = 226
        self.assertEqual(z, 226, f"Bit interleaving failed: got {z}, expected 226")


class TestFibonacciSpiral(unittest.TestCase):
    """Test Fibonacci spiral sampling."""
    
    def test_point_generation(self):
        """Test generation of uniform points."""
        n = 100
        points = FibonacciSpiral.generate_points(n, dimension=2)
        
        self.assertEqual(points.shape, (n, 2))
        
        # Check that points are roughly uniformly distributed
        # (variance should be relatively low)
        for dim in range(2):
            mean = np.mean(points[:, dim])
            std = np.std(points[:, dim])
            self.assertLess(abs(mean), 0.2, f"Mean too far from 0 in dim {dim}")
            self.assertLess(std, 1.0, f"Std too high in dim {dim}")
    
    def test_golden_ratio_spacing(self):
        """Test that points follow golden ratio distribution."""
        n = 50
        points = FibonacciSpiral.generate_points(n, dimension=2)
        
        # Calculate angles
        angles = np.arctan2(points[:, 1], points[:, 0])
        
        # Angles should be roughly evenly spaced modulo 2π
        # with golden ratio step
        phi = (1 + np.sqrt(5)) / 2
        expected_step = 2 * np.pi / phi
        
        # Check a few consecutive angle differences
        for i in range(1, min(10, n)):
            angle_diff = abs(angles[i] - angles[i-1])
            # Allow for wrapping around 2π
            angle_diff = min(angle_diff, 2 * np.pi - angle_diff)
            
            # Should be close to expected step (within 50% tolerance)
            self.assertLess(abs(angle_diff - expected_step) / expected_step, 0.5)


class TestFractalMemoryAllocator(unittest.TestCase):
    """Test fractal memory allocator."""
    
    def setUp(self):
        """Set up test allocator."""
        self.allocator = FractalMemoryAllocator(
            total_size=16384,
            min_block_size=64,
            curve_type='hilbert'
        )
    
    def test_allocation_success(self):
        """Test successful allocation."""
        block = self.allocator.allocate(256)
        
        self.assertIsNotNone(block)
        self.assertEqual(len(block.data), 256)
        self.assertGreaterEqual(block.fractal_index, 0)
    
    def test_allocation_failure_oom(self):
        """Test allocation failure when out of memory."""
        # Allocate entire space
        large_block = self.allocator.allocate(self.allocator.total_size)
        self.assertIsNotNone(large_block)
        
        # Try to allocate more
        small_block = self.allocator.allocate(64)
        self.assertIsNone(small_block)
    
    def test_deallocation(self):
        """Test deallocation and reuse."""
        block1 = self.allocator.allocate(256)
        self.assertIsNotNone(block1)
        
        success = self.allocator.deallocate(block1)
        self.assertTrue(success)
        
        # Should be able to allocate again
        block2 = self.allocator.allocate(256)
        self.assertIsNotNone(block2)
    
    def test_fragmentation_measurement(self):
        """Test fragmentation calculation."""
        # Allocate multiple blocks
        blocks = [self.allocator.allocate(128) for _ in range(10)]
        
        # Deallocate every other block
        for block in blocks[::2]:
            self.allocator.deallocate(block)
        
        frag = self.allocator.calculate_fragmentation()
        
        # Fragmentation should be non-zero
        self.assertGreater(frag, 0.0)
        self.assertLess(frag, 1.0)
    
    def test_compaction(self):
        """Test memory compaction."""
        # Create fragmented memory
        blocks = [self.allocator.allocate(128) for _ in range(10)]
        for block in blocks[::2]:
            self.allocator.deallocate(block)
        
        frag_before = self.allocator.calculate_fragmentation()
        
        # Compact
        moved = self.allocator.compact()
        
        frag_after = self.allocator.calculate_fragmentation()
        
        # Fragmentation should decrease
        self.assertLessEqual(frag_after, frag_before)


class TestPlanckMemoryPool(unittest.TestCase):
    """Test Planck-scale memory pool."""
    
    def test_first_fit_allocation(self):
        """Test first-fit allocation strategy."""
        pool = PlanckMemoryPool(
            size=4096,
            strategy=AllocationStrategy.FIRST_FIT,
            alignment=8
        )
        
        block = pool.allocate(256)
        self.assertIsNotNone(block)
        self.assertGreaterEqual(block.size, 256)
        self.assertTrue(block.allocated)
    
    def test_buddy_system_power_of_two(self):
        """Test buddy system uses power-of-2 sizes."""
        pool = PlanckMemoryPool(
            size=4096,
            strategy=AllocationStrategy.BUDDY_SYSTEM,
            alignment=8
        )
        
        # Allocate non-power-of-2 size
        block = pool.allocate(100)
        self.assertIsNotNone(block)
        
        # Size should be rounded up to power of 2
        self.assertEqual(block.size & (block.size - 1), 0,
                        f"Buddy block size {block.size} is not power of 2")
    
    def test_write_read(self):
        """Test writing and reading data."""
        pool = PlanckMemoryPool(size=2048, strategy=AllocationStrategy.FIRST_FIT)
        
        block = pool.allocate(100)
        self.assertIsNotNone(block)
        
        # Write data
        test_data = b"Hello, Planck Memory!"
        success = pool.write(block, test_data)
        self.assertTrue(success)
        
        # Read data
        read_data = pool.read(block)
        self.assertIsNotNone(read_data)
        self.assertTrue(read_data.startswith(test_data))
    
    def test_zero_copy_clone(self):
        """Test zero-copy cloning."""
        pool = PlanckMemoryPool(size=2048, strategy=AllocationStrategy.FIRST_FIT)
        
        block1 = pool.allocate(128)
        self.assertEqual(block1.ref_count, 1)
        
        # Clone
        block2 = pool.clone(block1)
        self.assertIs(block1, block2)
        self.assertEqual(block2.ref_count, 2)
        
        # Deallocate one reference
        pool.deallocate(block1)
        self.assertTrue(block2.allocated)
        self.assertEqual(block2.ref_count, 1)
        
        # Deallocate last reference
        pool.deallocate(block2)
        self.assertFalse(block2.allocated)
        self.assertEqual(block2.ref_count, 0)
    
    def test_defragmentation(self):
        """Test memory defragmentation."""
        pool = PlanckMemoryPool(size=4096, strategy=AllocationStrategy.FIRST_FIT)
        
        # Create fragmentation
        blocks = [pool.allocate(128) for _ in range(10)]
        for block in blocks[::2]:
            pool.deallocate(block)
        
        stats_before = pool.get_statistics()
        
        # Defragment
        moved = pool.defragment()
        
        stats_after = pool.get_statistics()
        
        # Should have moved some blocks
        self.assertGreater(moved, 0)
        
        # Fragmentation should decrease
        self.assertLessEqual(
            stats_after.fragmentation_ratio,
            stats_before.fragmentation_ratio
        )


class TestFractalMatrixOptimizer(unittest.TestCase):
    """Test fractal matrix optimization."""
    
    def setUp(self):
        """Set up test optimizer."""
        self.optimizer = FractalMatrixOptimizer()
    
    def test_fractal_block_decomposition(self):
        """Test fractal block decomposition."""
        # Create sparse matrix
        matrix = np.zeros((64, 64))
        matrix[0:8, 0:8] = 1.0
        matrix[32:40, 32:40] = 2.0
        
        blocks = self.optimizer.fractal_block_decomposition(matrix, block_size=8)
        
        # Should have some blocks
        self.assertGreater(len(blocks), 0)
        
        # All blocks should have position and size
        for block in blocks:
            self.assertIn('position', block)
            self.assertIn('size', block)
            self.assertIn('data', block)
    
    def test_matrix_entropy_calculation(self):
        """Test matrix entropy calculation."""
        # Uniform matrix (high entropy)
        uniform = np.random.rand(32, 32)
        entropy_uniform = self.optimizer.calculate_matrix_entropy(uniform)
        
        # Constant matrix (zero entropy)
        constant = np.ones((32, 32))
        entropy_constant = self.optimizer.calculate_matrix_entropy(constant)
        
        self.assertGreater(entropy_uniform, entropy_constant)
        self.assertAlmostEqual(entropy_constant, 0.0, places=1)
    
    def test_sparse_compression(self):
        """Test sparse matrix compression."""
        # Create sparse matrix
        matrix = np.zeros((64, 64))
        matrix[0:8, 0:8] = 1.0
        
        result = self.optimizer.compress_sparse_fractal(matrix)
        
        self.assertIn('blocks', result)
        self.assertIn('compression_ratio', result)
        self.assertIn('entropy', result)
        
        # Should achieve good compression on sparse matrix
        self.assertGreater(result['compression_ratio'], 1.0)


class TestIntegration(unittest.TestCase):
    """Integration tests combining multiple modules."""
    
    def test_fractal_memory_with_entropy(self):
        """Test fractal allocator with entropy tracking."""
        allocator = FractalMemoryAllocator(
            total_size=8192,
            min_block_size=64,
            curve_type='hilbert'
        )
        
        # Allocate blocks with different patterns
        blocks = []
        
        # Random data (high entropy)
        block1 = allocator.allocate(256)
        block1.data[:] = np.random.randint(0, 256, 256, dtype=np.uint8)
        blocks.append(block1)
        
        # Constant data (zero entropy)
        block2 = allocator.allocate(256)
        block2.data[:] = 42
        blocks.append(block2)
        
        # Check that entropy is tracked correctly
        self.assertAlmostEqual(block2.entropy, 0.0, places=1)
        self.assertGreater(block1.entropy, block2.entropy)
    
    def test_planck_pool_with_fractal_allocation(self):
        """Test Planck pool using fractal allocation pattern."""
        pool = PlanckMemoryPool(
            size=8192,
            strategy=AllocationStrategy.BUDDY_SYSTEM,
            alignment=8
        )
        
        # Allocate using Fibonacci-like sizes
        sizes = [8, 13, 21, 34, 55, 89, 144]
        blocks = []
        
        for size in sizes:
            block = pool.allocate(size)
            self.assertIsNotNone(block)
            blocks.append(block)
        
        # Verify buddy system alignment
        for block in blocks:
            # Size should be power of 2
            self.assertEqual(block.size & (block.size - 1), 0)


def run_tests():
    """Run all tests with detailed output."""
    print("=" * 80)
    print("RAFAELIA MEMORY OPTIMIZATION - Test Suite")
    print("=" * 80)
    print()
    
    # Create test suite
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    
    # Add all test classes
    suite.addTests(loader.loadTestsFromTestCase(TestHilbertCurve))
    suite.addTests(loader.loadTestsFromTestCase(TestZOrderCurve))
    suite.addTests(loader.loadTestsFromTestCase(TestFibonacciSpiral))
    suite.addTests(loader.loadTestsFromTestCase(TestFractalMemoryAllocator))
    suite.addTests(loader.loadTestsFromTestCase(TestPlanckMemoryPool))
    suite.addTests(loader.loadTestsFromTestCase(TestFractalMatrixOptimizer))
    suite.addTests(loader.loadTestsFromTestCase(TestIntegration))
    
    # Run tests
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    print()
    print("=" * 80)
    print("Test Summary:")
    print(f"  Tests run: {result.testsRun}")
    print(f"  Successes: {result.testsRun - len(result.failures) - len(result.errors)}")
    print(f"  Failures: {len(result.failures)}")
    print(f"  Errors: {len(result.errors)}")
    print("=" * 80)
    
    return 0 if result.wasSuccessful() else 1


if __name__ == '__main__':
    sys.exit(run_tests())
