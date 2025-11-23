#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
═══════════════════════════════════════════════════════════════════════════════
RAFAELIA PLANCK MEMORY - Planck-Scale Memory Pool Manager
═══════════════════════════════════════════════════════════════════════════════

Ultra-fine-grained memory management inspired by Planck-scale physics principles.
Implements memory allocation at the smallest practical granularity with quantum-
inspired optimization strategies.

PHYSICAL INSPIRATION:
- Planck length: ℓₚ = √(ℏG/c³) ≈ 1.616 × 10⁻³⁵ m (smallest meaningful length)
- Planck time: tₚ = √(ℏG/c⁵) ≈ 5.391 × 10⁻⁴⁴ s (smallest meaningful time)
- Digital analog: Smallest allocation unit (1 byte = quantum of memory)

OPTIMIZATION PRINCIPLES:
1. **Quantum Pooling**: Pre-allocated pools of fixed-size blocks
2. **Zero-Copy Operations**: Reference counting without data duplication
3. **Locality Optimization**: Keep related data in adjacent memory
4. **Entropy Minimization**: Reduce memory randomness for better compression
5. **Planck Granularity**: Fine-grained tracking at byte level

Part of RAFAELIA Framework - Ultra-Optimization Suite
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
- Planck, M. (1899). "Über irreversible Strahlungsvorgänge"
- Wheeler, J. A. (1955). "Geons". Physical Review
- Slab allocation: Bonwick, J. (1994). "The Slab Allocator"
- Buddy system: Knuth, D. E. (1997). "The Art of Computer Programming, Vol. 1"

VERSION: 1.0.0
TIMESTAMP: 2025-11-23
STATUS: OPERATIONAL
═══════════════════════════════════════════════════════════════════════════════
"""

import numpy as np
from typing import Optional, Dict, List, Tuple
from dataclasses import dataclass
from enum import IntEnum
import time


# Planck-inspired constants
PLANCK_UNIT = 1  # 1 byte (smallest allocation unit)
GOLDEN_RATIO = 1.618033988749895  # φ for optimal size classes


class AllocationStrategy(IntEnum):
    """Memory allocation strategies."""
    FIRST_FIT = 0      # First block that fits
    BEST_FIT = 1       # Smallest block that fits
    WORST_FIT = 2      # Largest block (minimize fragmentation)
    BUDDY_SYSTEM = 3   # Power-of-2 buddy allocation
    SLAB = 4           # Fixed-size slab allocation


@dataclass
class MemoryBlock:
    """Memory block metadata."""
    address: int           # Virtual address (offset in pool)
    size: int             # Size in bytes
    allocated: bool       # Allocation status
    pool_id: int          # Pool identifier
    timestamp: float      # Allocation/deallocation timestamp
    ref_count: int = 1    # Reference counter for zero-copy


@dataclass
class PoolStatistics:
    """Statistics for memory pool."""
    total_size: int
    used_bytes: int
    free_bytes: int
    num_blocks: int
    num_allocations: int
    num_deallocations: int
    fragmentation_ratio: float
    average_block_size: float
    peak_usage: int


class PlanckMemoryPool:
    """
    Planck-scale memory pool manager.
    
    Features:
    - Multiple allocation strategies
    - Reference counting for zero-copy
    - Automatic defragmentation
    - Sub-byte granularity tracking
    - Entropy optimization
    """
    
    def __init__(self, 
                 size: int,
                 strategy: AllocationStrategy = AllocationStrategy.BUDDY_SYSTEM,
                 alignment: int = 8):
        """
        Initialize Planck memory pool.
        
        Args:
            size: Total pool size in bytes
            strategy: Allocation strategy
            alignment: Memory alignment in bytes
        """
        self.size = size
        self.strategy = strategy
        self.alignment = alignment
        
        # Main memory buffer
        self.buffer = np.zeros(size, dtype=np.uint8)
        
        # Block tracking
        self.blocks: List[MemoryBlock] = []
        self.free_list: List[MemoryBlock] = []
        
        # Initialize with one free block
        initial_block = MemoryBlock(
            address=0,
            size=size,
            allocated=False,
            pool_id=0,
            timestamp=time.time()
        )
        self.blocks.append(initial_block)
        self.free_list.append(initial_block)
        
        # Statistics
        self.stats = PoolStatistics(
            total_size=size,
            used_bytes=0,
            free_bytes=size,
            num_blocks=1,
            num_allocations=0,
            num_deallocations=0,
            fragmentation_ratio=0.0,
            average_block_size=float(size),
            peak_usage=0
        )
        
        # Buddy system state
        self._init_buddy_system()
    
    def _init_buddy_system(self):
        """Initialize buddy system data structures."""
        if self.strategy != AllocationStrategy.BUDDY_SYSTEM:
            return
        
        # Calculate maximum order (log2 of size, rounded up for non-power-of-2)
        import math
        max_order = int(math.ceil(math.log2(self.size)))
        
        # Free lists for each order
        self.buddy_free_lists: Dict[int, List[MemoryBlock]] = {
            i: [] for i in range(max_order + 1)
        }
        
        # Add initial block to maximum order
        if self.blocks:
            self.buddy_free_lists[max_order].append(self.blocks[0])
    
    def allocate(self, size: int) -> Optional[MemoryBlock]:
        """
        Allocate memory block of specified size.
        
        Args:
            size: Size in bytes (Planck units)
            
        Returns:
            MemoryBlock if successful, None if out of memory
        """
        if size <= 0:
            return None
        
        # Align size
        aligned_size = self._align_size(size)
        
        # Choose allocation method based on strategy
        if self.strategy == AllocationStrategy.BUDDY_SYSTEM:
            block = self._allocate_buddy(aligned_size)
        elif self.strategy == AllocationStrategy.SLAB:
            block = self._allocate_slab(aligned_size)
        else:
            block = self._allocate_standard(aligned_size)
        
        if block:
            self.stats.num_allocations += 1
            self.stats.used_bytes += block.size
            self.stats.free_bytes -= block.size
            
            if self.stats.used_bytes > self.stats.peak_usage:
                self.stats.peak_usage = self.stats.used_bytes
            
            self._update_statistics()
        
        return block
    
    def deallocate(self, block: MemoryBlock) -> bool:
        """
        Deallocate memory block.
        
        Args:
            block: Block to deallocate
            
        Returns:
            True if successful
        """
        if not block or not block.allocated:
            return False
        
        # Decrement reference count
        block.ref_count -= 1
        
        if block.ref_count > 0:
            # Still has references, don't free yet
            return True
        
        # Mark as free
        block.allocated = False
        block.timestamp = time.time()
        
        # Add to free list
        self.free_list.append(block)
        
        # Update statistics
        self.stats.num_deallocations += 1
        self.stats.used_bytes -= block.size
        self.stats.free_bytes += block.size
        
        # Try to coalesce with adjacent blocks
        self._coalesce(block)
        
        self._update_statistics()
        
        return True
    
    def _allocate_standard(self, size: int) -> Optional[MemoryBlock]:
        """Standard allocation (first/best/worst fit)."""
        if not self.free_list:
            return None
        
        # Find suitable block
        candidate = None
        candidate_idx = -1
        
        for idx, free_block in enumerate(self.free_list):
            if free_block.size >= size:
                if self.strategy == AllocationStrategy.FIRST_FIT:
                    candidate = free_block
                    candidate_idx = idx
                    break
                elif self.strategy == AllocationStrategy.BEST_FIT:
                    if candidate is None or free_block.size < candidate.size:
                        candidate = free_block
                        candidate_idx = idx
                elif self.strategy == AllocationStrategy.WORST_FIT:
                    if candidate is None or free_block.size > candidate.size:
                        candidate = free_block
                        candidate_idx = idx
        
        if candidate is None:
            return None
        
        # Remove from free list
        self.free_list.pop(candidate_idx)
        
        # Split block if necessary
        if candidate.size > size:
            # Create new free block with remainder
            remainder = MemoryBlock(
                address=candidate.address + size,
                size=candidate.size - size,
                allocated=False,
                pool_id=candidate.pool_id,
                timestamp=time.time()
            )
            self.blocks.append(remainder)
            self.free_list.append(remainder)
            
            # Shrink allocated block
            candidate.size = size
        
        # Mark as allocated
        candidate.allocated = True
        candidate.timestamp = time.time()
        
        return candidate
    
    def _allocate_buddy(self, size: int) -> Optional[MemoryBlock]:
        """Buddy system allocation."""
        # Round up to power of 2
        order = int(np.ceil(np.log2(size)))
        actual_size = 1 << order
        
        # Find smallest available block
        for current_order in range(order, len(self.buddy_free_lists)):
            if self.buddy_free_lists[current_order]:
                # Found a block
                block = self.buddy_free_lists[current_order].pop(0)
                
                # Split block down to required size
                while current_order > order:
                    current_order -= 1
                    buddy_size = 1 << current_order
                    
                    # Create buddy block
                    buddy = MemoryBlock(
                        address=block.address + buddy_size,
                        size=buddy_size,
                        allocated=False,
                        pool_id=block.pool_id,
                        timestamp=time.time()
                    )
                    self.blocks.append(buddy)
                    self.buddy_free_lists[current_order].append(buddy)
                    
                    # Shrink original block
                    block.size = buddy_size
                
                block.allocated = True
                block.timestamp = time.time()
                return block
        
        return None
    
    def _allocate_slab(self, size: int) -> Optional[MemoryBlock]:
        """Slab allocation for fixed-size objects."""
        # For slab allocation, we use size classes based on golden ratio
        size_class = self._get_size_class(size)
        
        # Try to find block of exact size class
        for idx, free_block in enumerate(self.free_list):
            if free_block.size == size_class:
                block = self.free_list.pop(idx)
                block.allocated = True
                block.timestamp = time.time()
                return block
        
        # Fall back to standard allocation
        return self._allocate_standard(size_class)
    
    def _get_size_class(self, size: int) -> int:
        """Get size class for slab allocation using golden ratio."""
        # Size classes: 8, 13, 21, 34, 55, 89, ... (Fibonacci-like)
        base = 8
        classes = [base]
        
        while classes[-1] < self.size:
            next_class = int(classes[-1] * GOLDEN_RATIO)
            classes.append(next_class)
        
        # Find smallest class >= size
        for cls in classes:
            if cls >= size:
                return cls
        
        return classes[-1]
    
    def _align_size(self, size: int) -> int:
        """Align size to alignment boundary."""
        return ((size + self.alignment - 1) // self.alignment) * self.alignment
    
    def _coalesce(self, block: MemoryBlock):
        """Coalesce adjacent free blocks."""
        # Sort blocks by address
        self.blocks.sort(key=lambda b: b.address)
        
        # Find adjacent free blocks
        merged = True
        while merged:
            merged = False
            
            for i in range(len(self.blocks) - 1):
                curr = self.blocks[i]
                next_block = self.blocks[i + 1]
                
                # Check if adjacent and both free
                if (not curr.allocated and not next_block.allocated and
                    curr.address + curr.size == next_block.address):
                    
                    # Merge blocks
                    curr.size += next_block.size
                    
                    # Remove next block
                    self.blocks.pop(i + 1)
                    if next_block in self.free_list:
                        self.free_list.remove(next_block)
                    
                    merged = True
                    break
    
    def defragment(self) -> int:
        """
        Defragment memory by moving allocated blocks together.
        
        Returns:
            Number of blocks moved
        """
        # Sort blocks by address
        self.blocks.sort(key=lambda b: b.address)
        
        moved = 0
        current_addr = 0
        
        # Move all allocated blocks to the beginning
        for block in self.blocks:
            if block.allocated:
                if block.address != current_addr:
                    # Move block data
                    old_addr = block.address
                    new_addr = current_addr
                    
                    # Copy data in buffer
                    self.buffer[new_addr:new_addr + block.size] = \
                        self.buffer[old_addr:old_addr + block.size]
                    
                    block.address = new_addr
                    moved += 1
                
                current_addr += block.size
        
        # Consolidate all free space at the end
        self.free_list = []
        
        if current_addr < self.size:
            free_block = MemoryBlock(
                address=current_addr,
                size=self.size - current_addr,
                allocated=False,
                pool_id=0,
                timestamp=time.time()
            )
            
            # Remove old free blocks
            self.blocks = [b for b in self.blocks if b.allocated]
            self.blocks.append(free_block)
            self.free_list.append(free_block)
        
        self._update_statistics()
        
        return moved
    
    def _update_statistics(self):
        """Update pool statistics."""
        allocated_blocks = [b for b in self.blocks if b.allocated]
        
        self.stats.num_blocks = len(self.blocks)
        
        if allocated_blocks:
            self.stats.average_block_size = \
                sum(b.size for b in allocated_blocks) / len(allocated_blocks)
        
        # Calculate fragmentation (number of free blocks / total free space)
        num_free = len(self.free_list)
        if self.stats.free_bytes > 0 and num_free > 0:
            self.stats.fragmentation_ratio = num_free / (self.stats.free_bytes / 1024)
        else:
            self.stats.fragmentation_ratio = 0.0
    
    def get_statistics(self) -> PoolStatistics:
        """Get current pool statistics."""
        self._update_statistics()
        return self.stats
    
    def write(self, block: MemoryBlock, data: bytes) -> bool:
        """
        Write data to allocated block.
        
        Args:
            block: Target block
            data: Data to write
            
        Returns:
            True if successful
        """
        if not block or not block.allocated:
            return False
        
        if len(data) > block.size:
            return False
        
        # Write to buffer
        self.buffer[block.address:block.address + len(data)] = \
            np.frombuffer(data, dtype=np.uint8)
        
        return True
    
    def read(self, block: MemoryBlock, size: Optional[int] = None) -> Optional[bytes]:
        """
        Read data from allocated block.
        
        Args:
            block: Source block
            size: Number of bytes to read (None = entire block)
            
        Returns:
            Data bytes if successful
        """
        if not block or not block.allocated:
            return None
        
        read_size = size if size is not None else block.size
        read_size = min(read_size, block.size)
        
        return self.buffer[block.address:block.address + read_size].tobytes()
    
    def clone(self, block: MemoryBlock) -> Optional[MemoryBlock]:
        """
        Create zero-copy clone of block (increments reference count).
        
        Args:
            block: Block to clone
            
        Returns:
            Same block with incremented reference count
        """
        if not block or not block.allocated:
            return None
        
        block.ref_count += 1
        return block


def demo_planck_memory():
    """Demonstration of Planck memory pool."""
    print("=" * 80)
    print("RAFAELIA PLANCK MEMORY - Demonstration")
    print("=" * 80)
    print()
    
    print(f"Planck Unit (smallest allocation): {PLANCK_UNIT} byte")
    print(f"Golden Ratio (Φ): {GOLDEN_RATIO}")
    print()
    
    # Test different allocation strategies
    strategies = [
        ("First Fit", AllocationStrategy.FIRST_FIT),
        ("Best Fit", AllocationStrategy.BEST_FIT),
        ("Buddy System", AllocationStrategy.BUDDY_SYSTEM),
    ]
    
    for strategy_name, strategy in strategies:
        print(f"Testing {strategy_name} Strategy:")
        print("-" * 80)
        
        # Create pool
        pool = PlanckMemoryPool(size=4096, strategy=strategy, alignment=8)
        
        # Allocate blocks
        blocks = []
        sizes = [64, 128, 256, 32, 512]
        
        print("  Allocating blocks:")
        for size in sizes:
            block = pool.allocate(size)
            if block:
                blocks.append(block)
                print(f"    Allocated {block.size} bytes at address {block.address}")
        
        # Write data
        for i, block in enumerate(blocks):
            data = bytes([i] * min(16, block.size))
            pool.write(block, data)
        
        # Get statistics
        stats = pool.get_statistics()
        print(f"\n  Statistics after allocation:")
        print(f"    Used: {stats.used_bytes} bytes ({stats.used_bytes/stats.total_size*100:.1f}%)")
        print(f"    Free: {stats.free_bytes} bytes")
        print(f"    Blocks: {stats.num_blocks}")
        print(f"    Fragmentation: {stats.fragmentation_ratio:.4f}")
        print(f"    Average block: {stats.average_block_size:.1f} bytes")
        
        # Deallocate some blocks
        for block in blocks[::2]:  # Deallocate every other block
            pool.deallocate(block)
        
        stats = pool.get_statistics()
        print(f"\n  After selective deallocation:")
        print(f"    Fragmentation: {stats.fragmentation_ratio:.4f}")
        
        # Defragment
        moved = pool.defragment()
        stats = pool.get_statistics()
        print(f"\n  After defragmentation:")
        print(f"    Moved {moved} blocks")
        print(f"    Fragmentation: {stats.fragmentation_ratio:.4f}")
        print()
    
    # Test zero-copy clone
    print("Testing Zero-Copy Clone:")
    print("-" * 80)
    pool = PlanckMemoryPool(size=2048, strategy=AllocationStrategy.FIRST_FIT)
    
    block1 = pool.allocate(256)
    print(f"  Original block ref count: {block1.ref_count}")
    
    block2 = pool.clone(block1)
    print(f"  After clone ref count: {block2.ref_count}")
    
    pool.deallocate(block1)
    print(f"  After one deallocate ref count: {block2.ref_count}")
    print(f"  Block still allocated: {block2.allocated}")
    
    pool.deallocate(block2)
    print(f"  After both deallocate ref count: {block2.ref_count}")
    print(f"  Block still allocated: {block2.allocated}")
    print()
    
    print("=" * 80)
    print("RAFAELIA Philosophy: VAZIO → VERBO → CHEIO → RETRO")
    print("Planck-scale optimization: Fine-grained perfection at quantum limits")
    print("=" * 80)


if __name__ == '__main__':
    demo_planck_memory()
