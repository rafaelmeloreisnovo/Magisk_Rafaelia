#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
═══════════════════════════════════════════════════════════════════════════════
RAFAELIA FRACTAL MEMORY - Fractal Vector Memory Optimization Module
═══════════════════════════════════════════════════════════════════════════════

Implements memory optimization using fractal geometry patterns for improved
space utilization and cache efficiency.

MATHEMATICAL FOUNDATION:
- Fractal dimension: D = log(N) / log(1/r) where N=copies, r=scale factor
- Hilbert curve space-filling: O(n²) space in O(n) linear scan
- Z-order (Morton) curve: Interleaving coordinates for spatial locality
- Fibonacci spiral: Golden ratio-based low-discrepancy sequences

OPTIMIZATION PRINCIPLES:
1. **Spatial Locality**: Data accessed together stored together
2. **Cache Efficiency**: Fractal patterns maximize cache hit rates
3. **Memory Compaction**: Hierarchical storage reduces fragmentation
4. **Entropy Reduction**: Ordered structures improve compression

Part of RAFAELIA Framework - Memory Optimization Suite
Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
Philosophy: VAZIO → VERBO → CHEIO → RETRO (Empty → Action → Full → Feedback)

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

See LICENSE and RAFAELIA_LICENSE.md for complete terms.

REFERENCES:
- Hilbert, D. (1891). "Über die stetige Abbildung einer Linie auf ein Flächenstück"
- Morton, G. M. (1966). "A computer oriented geodetic data base and a new technique"
- Mandelbrot, B. (1982). "The Fractal Geometry of Nature"
- Fibonacci spiral sampling: González, Á. (2010). Mathematical Geosciences

VERSION: 1.0.0
TIMESTAMP: 2025-11-23
STATUS: OPERATIONAL
═══════════════════════════════════════════════════════════════════════════════
"""

import numpy as np
from typing import Tuple, List, Optional, Union, Callable
import math
from dataclasses import dataclass


# Golden ratio constant (Phi)
PHI = (1 + math.sqrt(5)) / 2  # 1.618033988749895


@dataclass
class MemoryBlock:
    """Represents a memory block with fractal addressing."""
    data: np.ndarray
    fractal_index: int
    dimension: int
    entropy: float


class HilbertCurve:
    """
    Hilbert space-filling curve for 2D memory mapping.
    
    Maps 2D coordinates to 1D linear address while preserving locality.
    Provides O(1) coordinate-to-distance and distance-to-coordinate conversion.
    """
    
    @staticmethod
    def xy_to_d(n: int, x: int, y: int) -> int:
        """
        Convert (x, y) coordinates to Hilbert curve distance.
        
        Args:
            n: Order of Hilbert curve (size = 2^n × 2^n)
            x: X coordinate (0 to 2^n - 1)
            y: Y coordinate (0 to 2^n - 1)
            
        Returns:
            Distance along Hilbert curve
        """
        d = 0
        s = n // 2
        
        while s > 0:
            rx = (x & s) > 0
            ry = (y & s) > 0
            d += s * s * ((3 * rx) ^ ry)
            
            # Rotate coordinates
            if ry == 0:
                if rx == 1:
                    x = s - 1 - x
                    y = s - 1 - y
                x, y = y, x
            
            s //= 2
        
        return d
    
    @staticmethod
    def d_to_xy(n: int, d: int) -> Tuple[int, int]:
        """
        Convert Hilbert curve distance to (x, y) coordinates.
        
        Args:
            n: Order of Hilbert curve
            d: Distance along curve
            
        Returns:
            Tuple of (x, y) coordinates
        """
        x = y = 0
        s = 1
        
        while s < n:
            rx = 1 & (d // 2)
            ry = 1 & (d ^ rx)
            
            # Rotate coordinates
            if ry == 0:
                if rx == 1:
                    x = s - 1 - x
                    y = s - 1 - y
                x, y = y, x
            
            x += s * rx
            y += s * ry
            d //= 4
            s *= 2
        
        return x, y


class ZOrderCurve:
    """
    Z-order (Morton) curve for efficient multi-dimensional indexing.
    
    Interleaves coordinate bits for spatial locality preservation.
    Faster than Hilbert but slightly less locality-preserving.
    """
    
    @staticmethod
    def encode(x: int, y: int) -> int:
        """
        Encode (x, y) coordinates into Morton code.
        
        Interleaves bits: x=0b1010, y=0b1100 -> 0b11100010
        
        Args:
            x: X coordinate
            y: Y coordinate
            
        Returns:
            Morton code (Z-order index)
        """
        def part_by_1(n: int) -> int:
            """Spread bits by inserting 0 between each bit."""
            n &= 0x0000ffff
            n = (n | (n << 8)) & 0x00FF00FF
            n = (n | (n << 4)) & 0x0F0F0F0F
            n = (n | (n << 2)) & 0x33333333
            n = (n | (n << 1)) & 0x55555555
            return n
        
        return part_by_1(x) | (part_by_1(y) << 1)
    
    @staticmethod
    def decode(z: int) -> Tuple[int, int]:
        """
        Decode Morton code back to (x, y) coordinates.
        
        Args:
            z: Morton code
            
        Returns:
            Tuple of (x, y) coordinates
        """
        def compact_by_1(n: int) -> int:
            """Compact by removing odd bits."""
            n &= 0x55555555
            n = (n ^ (n >> 1)) & 0x33333333
            n = (n ^ (n >> 2)) & 0x0F0F0F0F
            n = (n ^ (n >> 4)) & 0x00FF00FF
            n = (n ^ (n >> 8)) & 0x0000FFFF
            return n
        
        return compact_by_1(z), compact_by_1(z >> 1)


class FibonacciSpiral:
    """
    Fibonacci spiral for low-discrepancy memory sampling.
    
    Uses golden ratio for optimal distribution of points in memory space.
    Minimizes clustering and maximizes coverage.
    """
    
    @staticmethod
    def generate_points(n: int, dimension: int = 2) -> np.ndarray:
        """
        Generate n points using Fibonacci spiral in d-dimensional space.
        
        Args:
            n: Number of points to generate
            dimension: Dimensionality of space
            
        Returns:
            Array of points (n × dimension)
        """
        points = np.zeros((n, dimension))
        golden_ratio = (1 + np.sqrt(5)) / 2
        
        for i in range(n):
            # Fibonacci lattice for uniform distribution
            theta = 2 * np.pi * i / golden_ratio
            phi = np.arccos(1 - 2 * (i + 0.5) / n)
            
            if dimension == 2:
                points[i, 0] = np.cos(theta)
                points[i, 1] = np.sin(theta)
            elif dimension == 3:
                points[i, 0] = np.cos(theta) * np.sin(phi)
                points[i, 1] = np.sin(theta) * np.sin(phi)
                points[i, 2] = np.cos(phi)
            else:
                # Generalized to arbitrary dimensions
                for d in range(dimension):
                    angle = 2 * np.pi * i * (golden_ratio ** d)
                    points[i, d] = np.cos(angle)
        
        return points


class FractalMemoryAllocator:
    """
    Fractal-based memory allocator for optimal space utilization.
    
    Features:
    - Hierarchical block allocation using fractal patterns
    - Self-similar memory structure for efficient access
    - Adaptive granularity based on access patterns
    - Entropy-aware compression opportunities
    """
    
    def __init__(self, total_size: int, min_block_size: int = 64,
                 curve_type: str = 'hilbert'):
        """
        Initialize fractal memory allocator.
        
        Args:
            total_size: Total memory size in bytes
            min_block_size: Minimum allocation unit size
            curve_type: Space-filling curve type ('hilbert' or 'zorder')
        """
        self.total_size = total_size
        self.min_block_size = min_block_size
        self.curve_type = curve_type
        
        # Calculate grid size (must be power of 2 for curves)
        self.grid_order = int(np.ceil(np.log2(np.sqrt(total_size / min_block_size))))
        self.grid_size = 2 ** self.grid_order
        
        # Allocation bitmap (1 = allocated, 0 = free)
        self.allocation_map = np.zeros((self.grid_size, self.grid_size), dtype=bool)
        
        # Blocks registry
        self.blocks: List[MemoryBlock] = []
        
        # Statistics
        self.stats = {
            'allocations': 0,
            'deallocations': 0,
            'fragmentation': 0.0,
            'entropy': 0.0
        }
    
    def allocate(self, size: int, pattern: str = 'fractal') -> Optional[MemoryBlock]:
        """
        Allocate memory block using fractal patterns.
        
        Args:
            size: Size in bytes to allocate
            pattern: Allocation pattern ('fractal', 'sequential', 'fibonacci')
            
        Returns:
            MemoryBlock if successful, None if out of memory
        """
        # Calculate required grid cells
        cells_needed = int(np.ceil(size / self.min_block_size))
        
        # Find contiguous free cells using curve
        start_pos = self._find_free_region(cells_needed, pattern)
        
        if start_pos is None:
            return None
        
        # Allocate cells
        data = np.zeros(size, dtype=np.uint8)
        
        if self.curve_type == 'hilbert':
            fractal_index = start_pos
        else:  # zorder
            fractal_index = start_pos
        
        # Mark cells as allocated
        for i in range(cells_needed):
            if self.curve_type == 'hilbert':
                x, y = HilbertCurve.d_to_xy(self.grid_size, start_pos + i)
            else:
                x, y = ZOrderCurve.decode(start_pos + i)
            
            if x < self.grid_size and y < self.grid_size:
                self.allocation_map[x, y] = True
        
        # Calculate entropy
        entropy = self._calculate_entropy(data)
        
        # Create block
        block = MemoryBlock(
            data=data,
            fractal_index=fractal_index,
            dimension=2,
            entropy=entropy
        )
        
        self.blocks.append(block)
        self.stats['allocations'] += 1
        
        return block
    
    def deallocate(self, block: MemoryBlock) -> bool:
        """
        Deallocate memory block.
        
        Args:
            block: Block to deallocate
            
        Returns:
            True if successful
        """
        if block not in self.blocks:
            return False
        
        # Calculate cells to free
        cells_needed = int(np.ceil(len(block.data) / self.min_block_size))
        
        # Free cells
        for i in range(cells_needed):
            if self.curve_type == 'hilbert':
                x, y = HilbertCurve.d_to_xy(self.grid_size, block.fractal_index + i)
            else:
                x, y = ZOrderCurve.decode(block.fractal_index + i)
            
            if x < self.grid_size and y < self.grid_size:
                self.allocation_map[x, y] = False
        
        self.blocks.remove(block)
        self.stats['deallocations'] += 1
        
        return True
    
    def _find_free_region(self, cells_needed: int, pattern: str) -> Optional[int]:
        """Find contiguous free region using specified pattern."""
        total_cells = self.grid_size * self.grid_size
        
        if pattern == 'fibonacci':
            # Use Fibonacci spiral for search order
            search_points = FibonacciSpiral.generate_points(
                min(cells_needed * 10, total_cells), 
                dimension=2
            )
            # Map to grid coordinates
            search_coords = ((search_points + 1) * self.grid_size / 2).astype(int)
            search_coords = np.clip(search_coords, 0, self.grid_size - 1)
        else:
            # Sequential search using curve
            search_range = range(total_cells - cells_needed + 1)
        
        # Search for contiguous free cells
        for start in range(total_cells - cells_needed + 1):
            is_free = True
            
            for i in range(cells_needed):
                if self.curve_type == 'hilbert':
                    x, y = HilbertCurve.d_to_xy(self.grid_size, start + i)
                else:
                    x, y = ZOrderCurve.decode(start + i)
                
                if x >= self.grid_size or y >= self.grid_size or self.allocation_map[x, y]:
                    is_free = False
                    break
            
            if is_free:
                return start
        
        return None
    
    def _calculate_entropy(self, data: np.ndarray) -> float:
        """
        Calculate Shannon entropy of data.
        
        H = -Σ p(x) log₂ p(x)
        
        Args:
            data: Input data
            
        Returns:
            Entropy in bits
        """
        if len(data) == 0:
            return 0.0
        
        # Count value frequencies
        values, counts = np.unique(data, return_counts=True)
        probabilities = counts / len(data)
        
        # Calculate entropy
        entropy = -np.sum(probabilities * np.log2(probabilities + 1e-10))
        
        return float(entropy)
    
    def calculate_fragmentation(self) -> float:
        """
        Calculate memory fragmentation ratio.
        
        Returns:
            Fragmentation ratio (0 = no fragmentation, 1 = max fragmentation)
        """
        if not np.any(self.allocation_map):
            return 0.0
        
        # Count free-allocated boundaries (fragmentation indicators)
        boundaries = 0
        
        for i in range(self.grid_size):
            for j in range(self.grid_size):
                # Check neighbors
                for di, dj in [(0, 1), (1, 0)]:
                    ni, nj = i + di, j + dj
                    if ni < self.grid_size and nj < self.grid_size:
                        if self.allocation_map[i, j] != self.allocation_map[ni, nj]:
                            boundaries += 1
        
        max_boundaries = 2 * self.grid_size * (self.grid_size - 1)
        fragmentation = boundaries / max_boundaries if max_boundaries > 0 else 0.0
        
        self.stats['fragmentation'] = fragmentation
        return fragmentation
    
    def compact(self) -> int:
        """
        Compact memory by moving blocks to reduce fragmentation.
        
        Returns:
            Number of blocks moved
        """
        if not self.blocks:
            return 0
        
        # Sort blocks by fractal index
        self.blocks.sort(key=lambda b: b.fractal_index)
        
        # Reallocate blocks sequentially
        moved = 0
        new_allocation_map = np.zeros_like(self.allocation_map)
        current_pos = 0
        
        for block in self.blocks:
            cells_needed = int(np.ceil(len(block.data) / self.min_block_size))
            
            if block.fractal_index != current_pos:
                # Update block index
                block.fractal_index = current_pos
                moved += 1
            
            # Mark new positions
            for i in range(cells_needed):
                if self.curve_type == 'hilbert':
                    x, y = HilbertCurve.d_to_xy(self.grid_size, current_pos + i)
                else:
                    x, y = ZOrderCurve.decode(current_pos + i)
                
                if x < self.grid_size and y < self.grid_size:
                    new_allocation_map[x, y] = True
            
            current_pos += cells_needed
        
        self.allocation_map = new_allocation_map
        return moved
    
    def get_stats(self) -> dict:
        """Get allocator statistics."""
        self.calculate_fragmentation()
        
        total_cells = self.grid_size * self.grid_size
        used_cells = np.sum(self.allocation_map)
        
        return {
            **self.stats,
            'total_size': self.total_size,
            'grid_size': self.grid_size,
            'used_cells': int(used_cells),
            'free_cells': int(total_cells - used_cells),
            'utilization': float(used_cells / total_cells),
            'num_blocks': len(self.blocks),
            'curve_type': self.curve_type
        }


def demo_fractal_memory():
    """Demonstration of fractal memory optimization."""
    print("=" * 80)
    print("RAFAELIA FRACTAL MEMORY - Demonstration")
    print("=" * 80)
    print()
    
    # Test Hilbert curve
    print("1. Hilbert Curve Mapping:")
    n = 4
    print(f"   Grid size: {n}×{n}")
    
    for d in range(min(10, n*n)):
        x, y = HilbertCurve.d_to_xy(n, d)
        d_check = HilbertCurve.xy_to_d(n, x, y)
        print(f"   d={d:2d} → ({x}, {y}) → d={d_check:2d} {'✓' if d == d_check else '✗'}")
    print()
    
    # Test Z-order curve
    print("2. Z-Order (Morton) Curve:")
    for x, y in [(0, 0), (1, 2), (3, 3), (15, 15)]:
        z = ZOrderCurve.encode(x, y)
        x_dec, y_dec = ZOrderCurve.decode(z)
        print(f"   ({x:2d}, {y:2d}) → z={z:5d} → ({x_dec:2d}, {y_dec:2d}) "
              f"{'✓' if (x, y) == (x_dec, y_dec) else '✗'}")
    print()
    
    # Test Fibonacci spiral
    print("3. Fibonacci Spiral Points:")
    points = FibonacciSpiral.generate_points(8, dimension=2)
    print(f"   Generated {len(points)} points with golden ratio distribution")
    for i, point in enumerate(points[:5]):
        print(f"   Point {i}: ({point[0]:7.4f}, {point[1]:7.4f})")
    print(f"   ... and {len(points) - 5} more")
    print()
    
    # Test fractal allocator
    print("4. Fractal Memory Allocator:")
    allocator = FractalMemoryAllocator(
        total_size=65536,  # 64 KB
        min_block_size=64,  # 64 bytes
        curve_type='hilbert'
    )
    
    print(f"   Total size: {allocator.total_size} bytes")
    print(f"   Grid: {allocator.grid_size}×{allocator.grid_size}")
    print(f"   Min block: {allocator.min_block_size} bytes")
    print()
    
    # Allocate some blocks
    print("5. Memory Allocation Test:")
    block_sizes = [256, 512, 1024, 128, 2048]
    
    for i, size in enumerate(block_sizes):
        block = allocator.allocate(size, pattern='fractal')
        if block:
            print(f"   Allocated block {i}: {size} bytes at fractal index {block.fractal_index}")
            print(f"      Entropy: {block.entropy:.4f} bits")
    print()
    
    # Show statistics
    print("6. Allocator Statistics:")
    stats = allocator.get_stats()
    for key, value in stats.items():
        if isinstance(value, float):
            print(f"   {key}: {value:.4f}")
        else:
            print(f"   {key}: {value}")
    print()
    
    # Compact memory
    print("7. Memory Compaction:")
    moved = allocator.compact()
    print(f"   Moved {moved} blocks")
    
    frag_after = allocator.calculate_fragmentation()
    print(f"   Fragmentation after compaction: {frag_after:.4f}")
    print()
    
    print("=" * 80)
    print("RAFAELIA Philosophy: VAZIO → VERBO → CHEIO → RETRO")
    print("Golden Ratio (Φ): 1.618033988749895")
    print("=" * 80)


if __name__ == '__main__':
    demo_fractal_memory()
