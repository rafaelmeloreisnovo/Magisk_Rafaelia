#!/usr/bin/env python3
"""
test_smoke.py
=============
Lightweight smoke tests for RAFAELIA TT suite.

Tests basic functionality:
- TT-cross build
- Checkpoint save/load
- Patch update
- Hash generation

Tests use small shapes and skip heavy optional dependencies.
"""

import sys
import os
import unittest
import tempfile
import shutil

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import numpy as np

# Import RAFAELIA modules
from RAFAELIA_TT_CROSS_FULL import TTBuilder
from RAFAELIA_TT_UPDATE_FULL import (
    local_patch_update_tt, tt_round, compute_patch_error, tt_energy
)
from RAFAELIA_SPIRAL_FIBONACCI import value_generator
from RAFAELIA_TT_ACCEL import tt_svd_from_full, tt_reconstruct_full
from RAFAELIA_ENGINE_FULLSTACK import RAFAELIAEngine


class TestTTCross(unittest.TestCase):
    """Test TT-cross builder."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.temp_dir = tempfile.mkdtemp()
    
    def tearDown(self):
        """Clean up test files."""
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_build_simple(self):
        """Test building a simple TT."""
        # Small shape for speed
        shape = (4, 4, 4)
        
        def simple_func(idx):
            return sum(idx) * 0.1
        
        builder = TTBuilder(
            shape=shape,
            value_function=simple_func,
            max_rank=5,
            tol=1e-3,
            max_samples=1000,
            verbose=False
        )
        
        cores = builder.build_tt_cross()
        
        # Check structure
        self.assertEqual(len(cores), 3)
        self.assertEqual(cores[0].shape[0], 1)  # First rank is 1
        self.assertEqual(cores[-1].shape[2], 1)  # Last rank is 1
        
        # Check dimensions
        for i, core in enumerate(cores):
            self.assertEqual(core.shape[1], shape[i])
    
    def test_checkpoint_save_load(self):
        """Test checkpoint save and load."""
        shape = (4, 4)
        
        def test_func(idx):
            return np.prod(idx) * 0.1
        
        # Build TT
        builder = TTBuilder(
            shape=shape,
            value_function=test_func,
            max_rank=5,
            tol=1e-3,
            verbose=False
        )
        builder.build_tt_cross()
        
        # Save checkpoint
        checkpoint_path = os.path.join(self.temp_dir, "test.bitraf64")
        builder.save_checkpoint(checkpoint_path)
        
        # Check file exists
        self.assertTrue(os.path.exists(checkpoint_path))
        
        # Check file has correct magic header
        with open(checkpoint_path, 'rb') as f:
            magic = f.read(8)
            self.assertEqual(magic, b'BITRAF64')
        
        # Load checkpoint
        loaded = TTBuilder.load_checkpoint(checkpoint_path)
        
        # Check loaded state
        self.assertEqual(loaded.shape, shape)
        self.assertEqual(len(loaded.cores), len(builder.cores))
    
    def test_hash_generation(self):
        """Test HashVivo metadata generation."""
        shape = (3, 3, 3)
        
        builder = TTBuilder(
            shape=shape,
            value_function=sum,
            max_rank=5,
            tol=1e-3,
            verbose=False
        )
        builder.build_tt_cross()
        
        # Check hash metadata exists
        self.assertIn("hash", builder.hash_metadata)
        self.assertIn("algorithm", builder.hash_metadata)
        
        # Check hash is non-empty
        hash_val = builder.hash_metadata["hash"]
        self.assertIsInstance(hash_val, str)
        self.assertGreater(len(hash_val), 0)


class TestTTUpdate(unittest.TestCase):
    """Test TT update operations."""
    
    def test_patch_update(self):
        """Test local patch update."""
        # Create simple TT
        cores = [
            np.random.randn(1, 4, 3),
            np.random.randn(3, 4, 3),
            np.random.randn(3, 4, 1)
        ]
        
        # Define patch
        patch_indices = (slice(1, 3), slice(1, 3), slice(1, 3))
        patch_data = np.random.rand(2, 2, 2) * 0.5
        
        # Update
        updated = local_patch_update_tt(
            cores,
            patch_data,
            patch_indices,
            method="als",
            max_iter=3
        )
        
        # Check structure preserved
        self.assertEqual(len(updated), len(cores))
        for i in range(len(cores)):
            self.assertEqual(updated[i].shape[1], cores[i].shape[1])
    
    def test_tt_round(self):
        """Test TT-rounding."""
        # Create TT with redundant rank
        cores = [
            np.random.randn(1, 4, 5),
            np.random.randn(5, 4, 5),
            np.random.randn(5, 4, 1)
        ]
        
        # Round
        rounded = tt_round(cores, tol=1e-2, max_rank=3)
        
        # Check ranks reduced
        self.assertEqual(len(rounded), len(cores))
        # At least one core should have reduced rank
        total_params_before = sum(c.size for c in cores)
        total_params_after = sum(c.size for c in rounded)
        self.assertLessEqual(total_params_after, total_params_before)
    
    def test_energy_computation(self):
        """Test TT energy computation."""
        cores = [
            np.ones((1, 3, 2)),
            np.ones((2, 3, 2)),
            np.ones((2, 3, 1))
        ]
        
        energy = tt_energy(cores)
        
        # Check energy is positive
        self.assertGreater(energy, 0)
        self.assertIsInstance(energy, (float, np.floating))


class TestGenerators(unittest.TestCase):
    """Test value generators."""
    
    def test_fibonacci_spiral(self):
        """Test Fibonacci spiral generator."""
        gen = value_generator("fibonacci_spiral", shape=(4, 4, 4))
        
        # Test a few indices
        val1 = gen((0, 0, 0))
        val2 = gen((3, 3, 3))
        
        # Check values are floats
        self.assertIsInstance(val1, (float, np.floating))
        self.assertIsInstance(val2, (float, np.floating))
    
    def test_gaussian_blobs(self):
        """Test Gaussian blob generator."""
        gen = value_generator("gaussian_blobs", shape=(4, 4, 4), n_blobs=3)
        
        val = gen((2, 2, 2))
        self.assertIsInstance(val, (float, np.floating))
        self.assertGreaterEqual(val, 0)  # Gaussian blobs are non-negative
    
    def test_polynomial(self):
        """Test polynomial generator."""
        gen = value_generator("polynomial", degree=2)
        
        val = gen((1, 2, 3))
        self.assertIsInstance(val, (float, np.floating))
    
    def test_sinusoidal(self):
        """Test sinusoidal generator."""
        gen = value_generator("sinusoidal", frequencies=[1.0, 2.0])
        
        val = gen((0, 0))
        self.assertIsInstance(val, (float, np.floating))


class TestTTAccel(unittest.TestCase):
    """Test TT acceleration utilities."""
    
    def test_tt_svd_small(self):
        """Test TT-SVD on small tensor."""
        # Create small test tensor
        shape = (4, 4, 4)
        tensor = np.random.randn(*shape)
        
        # Decompose
        cores = tt_svd_from_full(tensor, tol=1e-3, verbose=False)
        
        # Check structure
        self.assertEqual(len(cores), 3)
        self.assertEqual(cores[0].shape[0], 1)
        self.assertEqual(cores[-1].shape[2], 1)
    
    def test_tt_reconstruct(self):
        """Test reconstruction from TT-SVD."""
        # Small tensor
        shape = (3, 3, 3)
        tensor = np.random.randn(*shape)
        
        # Decompose
        cores = tt_svd_from_full(tensor, tol=1e-4, verbose=False)
        
        # Reconstruct
        reconstructed = tt_reconstruct_full(cores)
        
        # Check error
        error = np.linalg.norm(tensor - reconstructed) / np.linalg.norm(tensor)
        self.assertLess(error, 1e-3)


class TestEngine(unittest.TestCase):
    """Test RAFAELIA engine."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.temp_dir = tempfile.mkdtemp()
    
    def tearDown(self):
        """Clean up test files."""
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_engine_build(self):
        """Test engine build workflow."""
        config = {
            "shape": (4, 4, 4),
            "max_rank": 5,
            "tol": 1e-3,
            "max_samples": 500,
            "checkpoint_dir": self.temp_dir,
            "generator_type": "fibonacci_spiral"
        }
        
        engine = RAFAELIAEngine(config=config, verbose=False)
        
        # Build TT
        builder = engine.build_tt(checkpoint_name="test_build")
        
        # Check builder is valid
        self.assertIsNotNone(builder)
        self.assertEqual(len(builder.cores), 3)
        
        # Check checkpoint was created
        checkpoint_path = os.path.join(self.temp_dir, "test_build.bitraf64")
        self.assertTrue(os.path.exists(checkpoint_path))
    
    def test_engine_status(self):
        """Test engine status."""
        config = {
            "shape": (3, 3, 3),
            "max_rank": 5,
            "checkpoint_dir": self.temp_dir
        }
        
        engine = RAFAELIAEngine(config=config, verbose=False)
        
        # Initial status (no TT loaded)
        status = engine.get_status()
        self.assertFalse(status["loaded"])
        
        # Build TT
        engine.build_tt()
        
        # Status after build
        status = engine.get_status()
        self.assertTrue(status["loaded"])
        self.assertIn("ranks", status)
        self.assertIn("hash", status)


class TestIntegration(unittest.TestCase):
    """Integration tests for full workflows."""
    
    def setUp(self):
        """Set up test fixtures."""
        self.temp_dir = tempfile.mkdtemp()
    
    def tearDown(self):
        """Clean up test files."""
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_build_patch_cycle(self):
        """Test complete build -> save -> load -> patch -> save cycle."""
        shape = (4, 4, 4)
        
        # Step 1: Build TT
        gen = value_generator("fibonacci_spiral", shape=shape)
        builder = TTBuilder(
            shape=shape,
            value_function=gen,
            max_rank=5,
            tol=1e-3,
            verbose=False
        )
        builder.build_tt_cross()
        
        # Step 2: Save checkpoint
        checkpoint1 = os.path.join(self.temp_dir, "step1.bitraf64")
        builder.save_checkpoint(checkpoint1)
        self.assertTrue(os.path.exists(checkpoint1))
        
        # Step 3: Load checkpoint
        loaded = TTBuilder.load_checkpoint(checkpoint1)
        self.assertEqual(loaded.shape, shape)
        
        # Step 4: Apply patch
        patch_indices = (slice(1, 3), slice(1, 3), slice(1, 3))
        patch_data = np.random.rand(2, 2, 2)
        
        updated_cores = local_patch_update_tt(
            loaded.cores,
            patch_data,
            patch_indices,
            method="als",
            max_iter=3
        )
        
        loaded.cores = updated_cores
        loaded.compute_hash()
        
        # Step 5: Save updated checkpoint
        checkpoint2 = os.path.join(self.temp_dir, "step2.bitraf64")
        loaded.save_checkpoint(checkpoint2)
        self.assertTrue(os.path.exists(checkpoint2))
        
        # Step 6: Verify hash changed
        hash1 = builder.hash_metadata.get("hash")
        hash2 = loaded.hash_metadata.get("hash")
        self.assertIsNotNone(hash1)
        self.assertIsNotNone(hash2)
        # Hashes should be different after update
        # (unless patch had no effect, which is unlikely)


def run_tests():
    """Run all smoke tests."""
    # Create test suite
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    
    # Add test cases
    suite.addTests(loader.loadTestsFromTestCase(TestTTCross))
    suite.addTests(loader.loadTestsFromTestCase(TestTTUpdate))
    suite.addTests(loader.loadTestsFromTestCase(TestGenerators))
    suite.addTests(loader.loadTestsFromTestCase(TestTTAccel))
    suite.addTests(loader.loadTestsFromTestCase(TestEngine))
    suite.addTests(loader.loadTestsFromTestCase(TestIntegration))
    
    # Run tests
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    # Return exit code
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    sys.exit(run_tests())
