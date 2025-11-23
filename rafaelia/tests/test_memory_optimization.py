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
