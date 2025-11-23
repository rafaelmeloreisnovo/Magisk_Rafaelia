#!/usr/bin/env python3
"""
RAFAELIA TT-UPDATE FULL - Tensor Train Local Update Algorithm

This module implements efficient local update algorithms for Tensor Train
decompositions, enabling online adaptation and incremental refinement.

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
import hashlib
import json
from typing import List, Tuple, Optional, Dict, Any
import os

# Optional dependencies with safe fallbacks
try:
    import cupy as cp
    HAS_CUPY = True
except ImportError:
    HAS_CUPY = False
    cp = None

try:
    from numba import jit
    HAS_NUMBA = True
except ImportError:
    HAS_NUMBA = False
    jit = lambda f: f  # No-op decorator

try:
    from scipy import linalg as scipy_linalg
    HAS_SCIPY = True
except ImportError:
    HAS_SCIPY = False
    scipy_linalg = None

try:
    import blake3
    HAS_BLAKE3 = True
except ImportError:
    HAS_BLAKE3 = False


class TTLocalUpdate:
    """
    Tensor Train local update using ALS (Alternating Least Squares).
    
    Supports efficient updating of individual TT cores while maintaining
    the TT structure and rank constraints.
    """
    
    def __init__(self, cores: List[np.ndarray], use_gpu: bool = False):
        """
        Initialize TT local update.
        
        Args:
            cores: List of TT cores with shapes [r_{i-1}, n_i, r_i]
            use_gpu: Use CuPy for GPU acceleration if available
        """
        self.cores = [np.array(c, dtype=np.float64) for c in cores]
        self.d = len(cores)  # Number of cores
        self.use_gpu = use_gpu and HAS_CUPY
        
        # Extract shape and ranks
        self.shape = [c.shape[1] for c in cores]
        self.ranks = [cores[0].shape[0]] + [c.shape[2] for c in cores]
        
        # Select backend
        self.xp = cp if self.use_gpu else np
        
        # Convert to GPU if needed
        if self.use_gpu:
            self.cores = [cp.array(c) for c in self.cores]
    
    def update_core(self, core_idx: int, target_indices: List[Tuple],
                   target_values: List[float], learning_rate: float = 0.1):
        """
        Update a single TT core using gradient descent on target values.
        
        Args:
            core_idx: Index of core to update (0 to d-1)
            target_indices: List of full tensor indices
            target_values: List of target values at those indices
            learning_rate: Learning rate for gradient descent
        """
        if not (0 <= core_idx < self.d):
            raise ValueError(f"Core index must be in [0, {self.d-1}]")
        
        # Compute gradient
        gradient = self._compute_gradient(core_idx, target_indices, target_values)
        
        # Update core with gradient descent
        self.cores[core_idx] = self.cores[core_idx] - learning_rate * gradient
    
    def _compute_gradient(self, core_idx: int, target_indices: List[Tuple],
                         target_values: List[float]) -> np.ndarray:
        """
        Compute gradient of loss with respect to specified core.
        
        Uses backpropagation through the TT structure.
        """
        r_left = self.ranks[core_idx]
        n = self.shape[core_idx]
        r_right = self.ranks[core_idx + 1]
        
        gradient = self.xp.zeros((r_left, n, r_right), dtype=np.float64)
        
        for indices, target_value in zip(target_indices, target_values):
            # Forward pass to get current value
            current_value = self._evaluate(indices)
            
            # Compute error
            error = current_value - target_value
            
            # Backward pass to compute gradient contribution
            grad_contrib = self._backward_pass(core_idx, indices, error)
            gradient += grad_contrib
        
        # Average gradient
        if len(target_indices) > 0:
            gradient /= len(target_indices)
        
        return gradient
    
    def _evaluate(self, indices: Tuple) -> float:
        """Evaluate TT at given indices."""
        result = self.xp.ones((1, 1))
        
        for i in range(self.d):
            idx = indices[i]
            core_slice = self.cores[i][:, idx, :]
            result = result @ core_slice
        
        if self.use_gpu:
            return float(result[0, 0].get())
        return float(result[0, 0])
    
    def _backward_pass(self, core_idx: int, indices: Tuple,
                       error: float) -> np.ndarray:
        """Compute gradient contribution for single sample."""
        # Compute left product (cores before core_idx)
        left_prod = self.xp.ones((1, 1))
        for i in range(core_idx):
            idx = indices[i]
            core_slice = self.cores[i][:, idx, :]
            # Ensure proper reshaping for matrix multiplication
            if i == 0:
                left_prod = core_slice.reshape(1, -1)
            else:
                left_prod = left_prod @ core_slice
        
        # Flatten to vector
        if core_idx == 0:
            left_vec = self.xp.ones(self.ranks[core_idx])
        else:
            left_vec = left_prod.flatten()
        
        # Compute right product (cores after core_idx)
        right_prod = self.xp.ones((1, 1))
        for i in range(self.d - 1, core_idx, -1):
            idx = indices[i]
            core_slice = self.cores[i][:, idx, :]
            # Build from right to left
            if i == self.d - 1:
                right_prod = core_slice.reshape(-1, 1)
            else:
                right_prod = core_slice @ right_prod
        
        # Flatten to vector
        if core_idx == self.d - 1:
            right_vec = self.xp.ones(self.ranks[core_idx + 1])
        else:
            right_vec = right_prod.flatten()
        
        # Compute gradient for this core
        idx = indices[core_idx]
        gradient = self.xp.zeros(self.cores[core_idx].shape, dtype=np.float64)
        
        # Gradient at specific index
        grad_slice = error * self.xp.outer(left_vec, right_vec)
        # Ensure correct shape
        target_shape = gradient[:, idx, :].shape
        if grad_slice.shape == target_shape:
            gradient[:, idx, :] = grad_slice
        else:
            gradient[:, idx, :] = grad_slice.reshape(target_shape)
        
        return gradient
    
    def als_sweep(self, target_data: Dict[Tuple, float],
                  n_iterations: int = 10, verbose: bool = False) -> Dict[str, Any]:
        """
        Perform ALS (Alternating Least Squares) sweep through all cores.
        
        Args:
            target_data: Dictionary mapping indices to target values
            n_iterations: Number of full sweeps
            verbose: Print progress information
            
        Returns:
            Dictionary with update statistics
        """
        stats = {
            'iterations': n_iterations,
            'final_error': 0.0,
            'error_history': []
        }
        
        indices_list = list(target_data.keys())
        values_list = list(target_data.values())
        
        for iteration in range(n_iterations):
            # Left-to-right sweep
            for i in range(self.d):
                self.update_core(i, indices_list, values_list, learning_rate=0.1)
            
            # Right-to-left sweep
            for i in range(self.d - 1, -1, -1):
                self.update_core(i, indices_list, values_list, learning_rate=0.1)
            
            # Compute error
            error = self._compute_error(target_data)
            stats['error_history'].append(error)
            stats['final_error'] = error
            
            if verbose:
                print(f"Iteration {iteration + 1}/{n_iterations}: error = {error:.2e}")
        
        return stats
    
    def _compute_error(self, target_data: Dict[Tuple, float]) -> float:
        """Compute mean squared error on target data."""
        total_error = 0.0
        for indices, target_value in target_data.items():
            predicted = self._evaluate(indices)
            total_error += (predicted - target_value) ** 2
        
        if len(target_data) > 0:
            return total_error / len(target_data)
        return 0.0
    
    def rank_adaptation(self, core_idx: int, new_rank: int,
                       method: str = 'truncate'):
        """
        Adapt rank of TT decomposition at specified core boundary.
        
        Args:
            core_idx: Index of core whose right rank to change
            new_rank: New rank value
            method: 'truncate' or 'expand'
        """
        if not (0 <= core_idx < self.d - 1):
            raise ValueError(f"Core index must be in [0, {self.d-2}]")
        
        old_rank = self.ranks[core_idx + 1]
        
        if new_rank == old_rank:
            return  # No change needed
        
        if method == 'truncate' and new_rank < old_rank:
            # Truncate via SVD
            self._truncate_rank(core_idx, new_rank)
        elif method == 'expand' and new_rank > old_rank:
            # Expand by padding
            self._expand_rank(core_idx, new_rank)
        else:
            raise ValueError(f"Invalid method or rank change: {method}, {new_rank}")
        
        self.ranks[core_idx + 1] = new_rank
    
    def _truncate_rank(self, core_idx: int, new_rank: int):
        """Truncate rank using SVD."""
        # Reshape and perform SVD on concatenated cores
        core_left = self.cores[core_idx]
        core_right = self.cores[core_idx + 1]
        
        r_left, n_left, r_mid = core_left.shape
        r_mid_old, n_right, r_right = core_right.shape
        
        # Merge cores temporarily
        merged = self.xp.einsum('ijk,klm->ijlm', core_left, core_right)
        merged = merged.reshape(r_left * n_left, n_right * r_right)
        
        # SVD
        if HAS_SCIPY and not self.use_gpu:
            U, S, Vt = scipy_linalg.svd(merged, full_matrices=False)
        else:
            U, S, Vt = self.xp.linalg.svd(merged, full_matrices=False)
        
        # Truncate
        U = U[:, :new_rank]
        S = S[:new_rank]
        Vt = Vt[:new_rank, :]
        
        # Redistribute
        self.cores[core_idx] = (U @ self.xp.diag(S)).reshape(r_left, n_left, new_rank)
        self.cores[core_idx + 1] = Vt.reshape(new_rank, n_right, r_right)
    
    def _expand_rank(self, core_idx: int, new_rank: int):
        """Expand rank by padding with zeros."""
        core_left = self.cores[core_idx]
        core_right = self.cores[core_idx + 1]
        
        r_left, n_left, r_mid = core_left.shape
        r_mid_old, n_right, r_right = core_right.shape
        
        # Pad right dimension of left core
        new_core_left = self.xp.zeros((r_left, n_left, new_rank), dtype=np.float64)
        new_core_left[:, :, :r_mid] = core_left
        
        # Pad left dimension of right core
        new_core_right = self.xp.zeros((new_rank, n_right, r_right), dtype=np.float64)
        new_core_right[:r_mid, :, :] = core_right
        
        self.cores[core_idx] = new_core_left
        self.cores[core_idx + 1] = new_core_right
    
    def get_cores_numpy(self) -> List[np.ndarray]:
        """Get cores as numpy arrays (handles GPU case)."""
        if self.use_gpu and HAS_CUPY:
            return [cp.asnumpy(c) for c in self.cores]
        return self.cores
    
    def save_checkpoint(self, filepath: str, metadata: Optional[Dict] = None):
        """Save updated TT cores with RAFAELIA manifest."""
        checkpoint_data = {
            'shape': self.shape,
            'ranks': self.ranks,
            'cores': [c.tolist() for c in self.get_cores_numpy()],
            'metadata': metadata or {}
        }
        
        # Add RAFAELIA signature
        checkpoint_data['rafaelia'] = {
            'signature': 'RAFCODE-Φ-∆RafaelVerboΩ',
            'module': 'TT_UPDATE_FULL',
            'philosophy': 'VAZIO → VERBO → CHEIO → RETRO'
        }
        
        # Compute hashes
        data_str = json.dumps(checkpoint_data['cores'], sort_keys=True)
        checkpoint_data['hashes'] = {
            'sha256': hashlib.sha256(data_str.encode()).hexdigest()
        }
        
        if HAS_BLAKE3:
            checkpoint_data['hashes']['blake3'] = blake3.blake3(
                data_str.encode()
            ).hexdigest()
        
        with open(filepath, 'w') as f:
            json.dump(checkpoint_data, f, indent=2)


def demo_tt_update():
    """Demonstration of TT local update."""
    print("=" * 60)
    print("RAFAELIA TT-UPDATE FULL - Demonstration")
    print("=" * 60)
    print()
    
    # Create small TT cores
    shape = [3, 4, 5]
    ranks = [1, 2, 3, 1]
    
    cores = []
    for i in range(len(shape)):
        core = np.random.randn(ranks[i], shape[i], ranks[i+1]) * 0.1
        cores.append(core)
    
    print(f"Tensor shape: {shape}")
    print(f"TT ranks: {ranks}")
    print()
    
    # Initialize updater
    tt_update = TTLocalUpdate(cores, use_gpu=False)
    
    # Create synthetic target data
    target_data = {}
    for _ in range(10):
        indices = tuple(np.random.randint(0, s) for s in shape)
        target_data[indices] = np.random.randn() * 2.0
    
    print(f"Target data samples: {len(target_data)}")
    print()
    
    # Perform ALS updates
    print("Performing ALS updates...")
    stats = tt_update.als_sweep(target_data, n_iterations=5, verbose=True)
    print()
    
    print("Update Statistics:")
    print(f"  Final error: {stats['final_error']:.2e}")
    print(f"  Error reduction: {stats['error_history'][0] / (stats['final_error'] + 1e-10):.2f}x")
    print()
    
    # Save checkpoint
    checkpoint_path = "/tmp/tt_update_checkpoint.json"
    tt_update.save_checkpoint(checkpoint_path, metadata={'demo': True})
    print(f"Checkpoint saved: {checkpoint_path}")
    print()
    
    print("=" * 60)
    print("RAFAELIA Philosophy: VAZIO → VERBO → CHEIO → RETRO")
    print("=" * 60)


if __name__ == '__main__':
    demo_tt_update()
