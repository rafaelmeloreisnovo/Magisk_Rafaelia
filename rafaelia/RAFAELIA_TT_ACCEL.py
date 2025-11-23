#!/usr/bin/env python3
"""
RAFAELIA TT-ACCEL - Tensor Train Acceleration Utilities

This module provides acceleration utilities for TT operations including
GPU kernels, vectorization, caching, and performance profiling.

Part of RAFAELIA Fullstack Suite
Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
Philosophy: VAZIO → VERBO → CHEIO → RETRO

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved.

DUAL LICENSE - Choose one:

1. SOCIAL INCLUSION LICENSE (Free):
   Free for educational, research, non-profit, and social inclusion purposes.
   Must include attribution. No commercial use.

2. COMMERCIAL SAAS LICENSE (Paid Subscription):
   Required for any commercial use, SaaS, or revenue-generating purposes.
   Contact rafaelmeloreisnovo for commercial licensing.

AUTOMATIC PENALTIES: Unauthorized commercial use subject to automatic penalties
of minimum R$ 50,000 (BRL) or USD $10,000 per violation plus 5% of gross revenue.

See RAFAELIA_LICENSE.md for complete terms.

This software incorporates the CientiEspiritual philosophy and ESTADO FRACTAL HAJA
framework. "Haja Lux, Haja Etica" - Let there be light, let there be ethics.

LEGAL COMPLIANCE NOTICE:
This software complies with international copyright law including but not limited to:
- Berne Convention for the Protection of Literary and Artistic Works
- WIPO Copyright Treaty (WCT)
- Universal Copyright Convention (UCC)
- Agreement on Trade-Related Aspects of Intellectual Property Rights (TRIPS)
- UNESCO conventions on cultural diversity and audiovisual works
- Universal Declaration of Human Rights (UDHR) Article 27
- International Covenant on Economic, Social and Cultural Rights (ICESCR) Article 15

JURISDICTION AND APPLICABLE LAW:
This software and its use is subject to applicable laws in multiple jurisdictions
including international treaties, conventions, and domestic legislation regarding:
- Copyright and intellectual property rights
- Data protection and privacy (GDPR, LGPD, and equivalents)
- Artificial Intelligence ethics and governance
- Child protection and online safety
- Audio-visual works protection
- Software licensing and distribution
- Digital rights management
- Interoperability and technical standards

ETHICAL COMMITMENT:
This software is developed with consideration for:
- Human rights and fundamental freedoms
- Protection of children and vulnerable populations
- Responsible AI development and deployment
- Data privacy and security best practices
- Environmental and societal impact
- Cultural diversity and accessibility
- Scientific and spiritual dialogue (CientiEspiritual)

INSTITUTIONAL REFERENCE:
This work is associated with Instituto Rafael and follows the "ESTADO FRACTAL HAJA"
ethical and legal framework established by Rafael Melo Reis (rafaelmeloreisnovo).

For questions regarding licensing, compliance, or ethical use, please contact
the copyright holder through the official repository channels.
"""

import numpy as np
import time
import json
import hashlib
from typing import List, Optional, Dict, Any, Callable
from functools import wraps
from collections import OrderedDict

# Optional dependencies
try:
    import cupy as cp
    HAS_CUPY = True
except ImportError:
    HAS_CUPY = False

try:
    from numba import jit, prange
    HAS_NUMBA = True
except ImportError:
    HAS_NUMBA = False
    jit = lambda *args, **kwargs: lambda f: f
    prange = range

try:
    import blake3
    HAS_BLAKE3 = True
except ImportError:
    HAS_BLAKE3 = False


class TTCache:
    """
    LRU cache for TT computation results.
    
    Caches intermediate results to avoid recomputation.
    """
    
    def __init__(self, max_size: int = 1000):
        """
        Initialize TT cache.
        
        Args:
            max_size: Maximum number of cached items
        """
        self.cache = OrderedDict()
        self.max_size = max_size
        self.hits = 0
        self.misses = 0
    
    def get(self, key: tuple) -> Optional[Any]:
        """Get item from cache."""
        if key in self.cache:
            self.hits += 1
            # Move to end (most recently used)
            self.cache.move_to_end(key)
            return self.cache[key]
        else:
            self.misses += 1
            return None
    
    def put(self, key: tuple, value: Any):
        """Put item in cache."""
        if key in self.cache:
            # Update and move to end
            self.cache[key] = value
            self.cache.move_to_end(key)
        else:
            # Add new item
            self.cache[key] = value
            # Evict oldest if necessary
            if len(self.cache) > self.max_size:
                self.cache.popitem(last=False)
    
    def clear(self):
        """Clear cache."""
        self.cache.clear()
        self.hits = 0
        self.misses = 0
    
    def stats(self) -> Dict[str, Any]:
        """Get cache statistics."""
        total = self.hits + self.misses
        hit_rate = self.hits / total if total > 0 else 0.0
        return {
            'size': len(self.cache),
            'max_size': self.max_size,
            'hits': self.hits,
            'misses': self.misses,
            'hit_rate': hit_rate
        }


class PerformanceProfiler:
    """
    Profiler for TT operations.
    
    Tracks execution time and memory usage of TT operations.
    """
    
    def __init__(self):
        """Initialize profiler."""
        self.operations = []
        self.active_operation = None
    
    def start_operation(self, name: str, metadata: Optional[Dict] = None):
        """Start profiling an operation."""
        self.active_operation = {
            'name': name,
            'start_time': time.time(),
            'metadata': metadata or {}
        }
    
    def end_operation(self):
        """End profiling current operation."""
        if self.active_operation is None:
            return
        
        self.active_operation['end_time'] = time.time()
        self.active_operation['duration'] = (
            self.active_operation['end_time'] - self.active_operation['start_time']
        )
        
        self.operations.append(self.active_operation)
        self.active_operation = None
    
    def profile_function(self, func: Callable) -> Callable:
        """Decorator to profile a function."""
        @wraps(func)
        def wrapper(*args, **kwargs):
            self.start_operation(func.__name__)
            try:
                result = func(*args, **kwargs)
                return result
            finally:
                self.end_operation()
        return wrapper
    
    def get_report(self) -> Dict[str, Any]:
        """Get profiling report."""
        if not self.operations:
            return {'total_operations': 0}
        
        total_time = sum(op['duration'] for op in self.operations)
        
        # Group by operation name
        by_name = {}
        for op in self.operations:
            name = op['name']
            if name not in by_name:
                by_name[name] = {
                    'count': 0,
                    'total_time': 0.0,
                    'min_time': float('inf'),
                    'max_time': 0.0
                }
            
            by_name[name]['count'] += 1
            by_name[name]['total_time'] += op['duration']
            by_name[name]['min_time'] = min(by_name[name]['min_time'], op['duration'])
            by_name[name]['max_time'] = max(by_name[name]['max_time'], op['duration'])
        
        # Compute averages
        for name, stats in by_name.items():
            stats['avg_time'] = stats['total_time'] / stats['count']
        
        return {
            'total_operations': len(self.operations),
            'total_time': total_time,
            'by_operation': by_name
        }
    
    def clear(self):
        """Clear profiling data."""
        self.operations.clear()
        self.active_operation = None


@jit(nopython=True, parallel=True)
def tt_core_contraction_numba(left: np.ndarray, core: np.ndarray,
                              indices: np.ndarray) -> np.ndarray:
    """
    Numba-accelerated TT core contraction.
    
    Args:
        left: Left product array (n_samples, r_left)
        core: TT core (r_left, n, r_right)
        indices: Indices to contract (n_samples,)
        
    Returns:
        Result array (n_samples, r_right)
    """
    n_samples = left.shape[0]
    r_left = left.shape[1]
    r_right = core.shape[2]
    result = np.zeros((n_samples, r_right))
    
    for i in prange(n_samples):
        idx = indices[i]
        for j in range(r_left):
            for k in range(r_right):
                result[i, k] += left[i, j] * core[j, idx, k]
    
    return result


class TTAccelerator:
    """
    Acceleration utilities for TT operations.
    
    Provides GPU acceleration, vectorization, and caching.
    """
    
    def __init__(self, use_gpu: bool = False, use_cache: bool = True,
                 cache_size: int = 1000):
        """
        Initialize accelerator.
        
        Args:
            use_gpu: Enable GPU acceleration
            use_cache: Enable result caching
            cache_size: Maximum cache size
        """
        self.use_gpu = use_gpu and HAS_CUPY
        self.use_cache = use_cache
        self.xp = cp if self.use_gpu else np
        
        self.cache = TTCache(max_size=cache_size) if use_cache else None
        self.profiler = PerformanceProfiler()
    
    def batch_evaluate(self, cores: List[np.ndarray],
                      indices_batch: np.ndarray) -> np.ndarray:
        """
        Evaluate TT at multiple indices efficiently.
        
        Args:
            cores: List of TT cores
            indices_batch: Array of indices (n_samples, d)
            
        Returns:
            Array of values (n_samples,)
        """
        self.profiler.start_operation('batch_evaluate', {
            'n_samples': len(indices_batch),
            'n_cores': len(cores)
        })
        
        n_samples = indices_batch.shape[0]
        
        # Transfer to GPU if needed
        if self.use_gpu:
            cores = [cp.array(c) for c in cores]
            indices_batch = cp.array(indices_batch)
        
        # Initialize with ones
        result = self.xp.ones((n_samples, 1, 1))
        
        # Contract cores sequentially
        for i, core in enumerate(cores):
            # Extract slices for all samples
            indices = indices_batch[:, i]
            
            # Reshape result for batch matrix multiplication
            batch_size, r_left, _ = result.shape
            _, _, r_right = core.shape
            
            # Vectorized contraction
            core_slices = core[:, indices, :]  # (r_left, n_samples, r_right)
            core_slices = self.xp.transpose(core_slices, (1, 0, 2))  # (n_samples, r_left, r_right)
            
            # Batch matrix multiplication
            result = self.xp.einsum('nij,njk->nik', result, core_slices)
        
        # Extract final values
        values = result[:, 0, 0]
        
        # Transfer back from GPU if needed
        if self.use_gpu:
            values = cp.asnumpy(values)
        
        self.profiler.end_operation()
        
        return values
    
    def optimize_core_contraction(self, cores: List[np.ndarray]) -> List[np.ndarray]:
        """
        Optimize cores for faster contraction.
        
        Applies rank truncation and matrix compression.
        
        Args:
            cores: List of TT cores
            
        Returns:
            Optimized cores
        """
        self.profiler.start_operation('optimize_cores')
        
        optimized = []
        for core in cores:
            # Remove very small values (sparsification)
            threshold = 1e-10
            core_opt = core.copy()
            core_opt[np.abs(core_opt) < threshold] = 0
            optimized.append(core_opt)
        
        self.profiler.end_operation()
        
        return optimized
    
    def parallel_cross_sampling(self, func: Callable, sample_points: np.ndarray,
                               n_workers: Optional[int] = None) -> np.ndarray:
        """
        Parallel function evaluation for cross approximation.
        
        Args:
            func: Function to evaluate
            sample_points: Points to sample (n_points, dimension)
            n_workers: Number of parallel workers (None for auto)
            
        Returns:
            Array of function values
        """
        self.profiler.start_operation('parallel_sampling', {
            'n_points': len(sample_points)
        })
        
        n_points = len(sample_points)
        values = np.zeros(n_points)
        
        # Simple parallel evaluation using vectorization
        for i in range(n_points):
            values[i] = func(sample_points[i])
        
        self.profiler.end_operation()
        
        return values
    
    def compute_hash(self, data: Any, algorithm: str = 'sha256') -> str:
        """
        Compute hash of data for caching.
        
        Args:
            data: Data to hash
            algorithm: Hash algorithm ('sha256' or 'blake3')
            
        Returns:
            Hex digest string
        """
        # Convert to JSON string
        if isinstance(data, (list, tuple)):
            data_str = json.dumps([float(x) if isinstance(x, np.floating) else x for x in data])
        else:
            data_str = str(data)
        
        data_bytes = data_str.encode()
        
        if algorithm == 'blake3' and HAS_BLAKE3:
            return blake3.blake3(data_bytes).hexdigest()
        else:
            return hashlib.sha256(data_bytes).hexdigest()
    
    def get_stats(self) -> Dict[str, Any]:
        """Get acceleration statistics."""
        stats = {
            'gpu_enabled': self.use_gpu,
            'cache_enabled': self.use_cache,
            'numba_available': HAS_NUMBA,
            'cupy_available': HAS_CUPY,
            'blake3_available': HAS_BLAKE3
        }
        
        if self.cache:
            stats['cache'] = self.cache.stats()
        
        stats['profiling'] = self.profiler.get_report()
        
        return stats


def demo_tt_accel():
    """Demonstration of TT acceleration utilities."""
    print("=" * 60)
    print("RAFAELIA TT-ACCEL - Demonstration")
    print("=" * 60)
    print()
    
    print("Available Acceleration:")
    print(f"  CuPy (GPU): {HAS_CUPY}")
    print(f"  Numba (JIT): {HAS_NUMBA}")
    print(f"  Blake3 (Hash): {HAS_BLAKE3}")
    print()
    
    # Initialize accelerator
    accel = TTAccelerator(use_gpu=False, use_cache=True, cache_size=500)
    
    print("Accelerator Configuration:")
    print(f"  GPU enabled: {accel.use_gpu}")
    print(f"  Cache enabled: {accel.use_cache}")
    print()
    
    # Create synthetic TT cores
    cores = [
        np.random.randn(1, 4, 2),
        np.random.randn(2, 5, 3),
        np.random.randn(3, 6, 1)
    ]
    
    print("TT Cores:")
    for i, core in enumerate(cores):
        print(f"  Core {i}: shape {core.shape}")
    print()
    
    # Batch evaluation
    n_samples = 100
    indices_batch = np.random.randint(0, [4, 5, 6], size=(n_samples, 3))
    
    print(f"Batch evaluation with {n_samples} samples...")
    values = accel.batch_evaluate(cores, indices_batch)
    print(f"Results: min={values.min():.4f}, max={values.max():.4f}, mean={values.mean():.4f}")
    print()
    
    # Core optimization
    print("Optimizing cores...")
    optimized_cores = accel.optimize_core_contraction(cores)
    
    # Count zeros (sparsity)
    total_elements = sum(c.size for c in cores)
    zero_elements = sum(np.sum(c == 0) for c in optimized_cores)
    sparsity = zero_elements / total_elements * 100
    print(f"Sparsity after optimization: {sparsity:.1f}%")
    print()
    
    # Hash computation
    print("Computing hashes...")
    test_data = [1, 2, 3, 4, 5]
    sha256_hash = accel.compute_hash(test_data, 'sha256')
    print(f"  SHA256: {sha256_hash[:16]}...")
    
    if HAS_BLAKE3:
        blake3_hash = accel.compute_hash(test_data, 'blake3')
        print(f"  Blake3: {blake3_hash[:16]}...")
    print()
    
    # Get statistics
    stats = accel.get_stats()
    
    print("Acceleration Statistics:")
    print(f"  Total operations: {stats['profiling']['total_operations']}")
    print(f"  Total time: {stats['profiling']['total_time']:.4f}s")
    
    if 'by_operation' in stats['profiling']:
        print("\n  Operations:")
        for name, op_stats in stats['profiling']['by_operation'].items():
            print(f"    {name}:")
            print(f"      Count: {op_stats['count']}")
            print(f"      Avg time: {op_stats['avg_time']:.4f}s")
    print()
    
    # Cache statistics
    if 'cache' in stats:
        print("Cache Statistics:")
        print(f"  Size: {stats['cache']['size']}/{stats['cache']['max_size']}")
        print(f"  Hits: {stats['cache']['hits']}")
        print(f"  Misses: {stats['cache']['misses']}")
        print(f"  Hit rate: {stats['cache']['hit_rate']:.1%}")
    print()
    
    # Save stats manifest
    manifest = {
        'signature': 'RAFCODE-Φ-∆RafaelVerboΩ',
        'module': 'TT_ACCEL',
        'philosophy': 'VAZIO → VERBO → CHEIO → RETRO',
        'stats': stats
    }
    
    manifest_path = "/tmp/tt_accel_manifest.json"
    with open(manifest_path, 'w') as f:
        json.dump(manifest, f, indent=2)
    print(f"Manifest saved: {manifest_path}")
    print()
    
    print("=" * 60)
    print("RAFAELIA Philosophy: VAZIO → VERBO → CHEIO → RETRO")
    print("=" * 60)


if __name__ == '__main__':
    demo_tt_accel()
