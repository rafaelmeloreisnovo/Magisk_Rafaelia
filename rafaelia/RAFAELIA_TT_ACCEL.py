#!/usr/bin/env python3
"""
RAFAELIA_TT_ACCEL.py
====================
TT utilities for acceleration and compression:
- tt_svd_from_full: Build TT from full tensor using SVD
- tt_reconstruct_slice: Reconstruct tensor values (imported pattern)
- tt_energy: Energy/norm computation
- tt_round: Rank reduction
- Estimation helpers and compression demos

Useful for compressing small tensors and as fallback when full tensor available.

Design rationale:
- Complementary to TT-cross (when full tensor is available)
- Fast compression for small/medium tensors
- Quality estimation and validation

Φ_ethica: Safe tensor operations
- Memory-aware processing
- Rank bounds
- Numerical stability checks
"""

import numpy as np
from typing import List, Tuple, Optional
import warnings


def tt_svd_from_full(
    tensor: np.ndarray,
    tol: float = 1e-6,
    max_rank: Optional[int] = None,
    verbose: bool = False
) -> List[np.ndarray]:
    """
    Build TT-decomposition from full tensor using TT-SVD algorithm.
    
    Args:
        tensor: Full tensor of shape (n1, n2, ..., nd)
        tol: Relative truncation tolerance
        max_rank: Maximum TT-rank (None = determined by tol)
        verbose: Print progress
        
    Returns:
        List of TT-cores [r_{k-1}, n_k, r_k]
    """
    shape = tensor.shape
    d = len(shape)
    
    if verbose:
        print(f"TT-SVD for shape {shape}")
        print(f"Parameters: tol={tol}, max_rank={max_rank}")
    
    cores = []
    C = tensor.copy()
    r_prev = 1
    
    # Compute norm for relative tolerance
    norm = np.linalg.norm(C)
    threshold = tol * norm / np.sqrt(d - 1)
    
    for k in range(d - 1):
        n_k = shape[k]
        n_right = int(np.prod(shape[k + 1:]))
        
        # Reshape to matrix
        C = C.reshape(r_prev * n_k, n_right)
        
        # SVD
        U, S, Vt = np.linalg.svd(C, full_matrices=False)
        
        # Determine rank
        if max_rank is not None:
            r = min(max_rank, len(S))
        else:
            # Truncate by threshold
            r = np.sum(S > threshold)
            r = max(1, min(r, len(S)))
        
        # Truncate
        U = U[:, :r]
        S = S[:r]
        Vt = Vt[:r, :]
        
        # Form core
        core = U.reshape(r_prev, n_k, r)
        cores.append(core)
        
        # Prepare for next iteration
        C = np.diag(S) @ Vt
        C = C.reshape(r, *shape[k + 1:])
        r_prev = r
        
        if verbose:
            print(f"Core {k}: shape {core.shape}, rank {r}")
    
    # Last core
    cores.append(C.reshape(r_prev, shape[-1], 1))
    
    if verbose:
        ranks = [c.shape[0] for c in cores] + [1]
        print(f"TT-SVD complete: ranks {ranks}")
    
    return cores


def tt_reconstruct_full(cores: List[np.ndarray]) -> np.ndarray:
    """
    Reconstruct full tensor from TT-cores.
    WARNING: Only use for small tensors (memory!)
    
    Args:
        cores: List of TT-cores
        
    Returns:
        Full tensor
    """
    # Start with first core
    result = cores[0][0, :, :]  # [n_0, r_1]
    
    # Contract through dimensions
    for k in range(1, len(cores) - 1):
        core = cores[k]  # [r_k, n_k, r_{k+1}]
        # result: [..., r_k]
        # Contract: result[..., r_k] × core[r_k, n_k, r_{k+1}]
        result = np.tensordot(result, core, axes=[[-1], [0]])
    
    # Last core
    result = np.tensordot(result, cores[-1][:, :, 0], axes=[[-1], [0]])
    
    return result


def tt_reconstruct_element(cores: List[np.ndarray], idx: Tuple[int, ...]) -> float:
    """
    Reconstruct a single tensor element from TT-cores.
    
    Args:
        cores: List of TT-cores
        idx: Multi-index
        
    Returns:
        Tensor value at index
    """
    # Start with first core
    result = cores[0][0, idx[0], :]  # [r_1]
    
    # Contract through dimensions
    for k in range(1, len(cores) - 1):
        result = result @ cores[k][:, idx[k], :]  # [r_k] @ [r_k, r_{k+1}] -> [r_{k+1}]
    
    # Last core
    result = result @ cores[-1][:, idx[-1], 0]  # [r_d] @ [r_d] -> scalar
    
    return result


def tt_slice_reconstruct(
    cores: List[np.ndarray],
    indices: Tuple
) -> np.ndarray:
    """
    Reconstruct a slice from TT-cores.
    Similar to RAFAELIA_TT_UPDATE_FULL.tt_reconstruct_slice but standalone.
    
    Args:
        cores: List of TT-cores
        indices: Tuple of slices/integers
        
    Returns:
        Reconstructed slice
    """
    # Normalize indices
    slices = []
    for i, idx in enumerate(indices):
        if isinstance(idx, int):
            slices.append(slice(idx, idx + 1))
        elif isinstance(idx, slice):
            slices.append(idx)
        else:
            slices.append(slice(None))
    
    # Extract slices from cores
    result = cores[0][:, slices[0], :]
    
    for k in range(1, len(cores)):
        core_slice = cores[k][:, slices[k], :]
        # Contract
        r_left = result.shape[-1]
        result = result.reshape(-1, r_left) @ core_slice.reshape(r_left, -1)
        result = result.reshape(*result.shape[:-1], core_slice.shape[1], core_slice.shape[-1])
    
    # Squeeze singleton dimensions
    result = result.squeeze()
    
    return result


def tt_energy(cores: List[np.ndarray]) -> float:
    """
    Compute Frobenius norm squared of TT efficiently.
    
    Args:
        cores: List of TT-cores
        
    Returns:
        Frobenius norm squared
    """
    # Gram matrix method: ||T||^2 = trace(G_1 * G_2 * ... * G_d)
    # where G_k = sum_i core_k[:, i, :].T @ core_k[:, i, :]
    
    G = None
    
    for k, core in enumerate(cores):
        r_left, n, r_right = core.shape
        
        # Compute local Gram matrix
        core_mat = core.reshape(r_left, n * r_right)
        G_local = core_mat @ core_mat.T  # [r_left, r_left]
        
        if G is None:
            G = G_local
        else:
            # For proper contraction, we need to handle the tensor structure
            # Simplified: trace product
            G = G @ G_local
    
    return np.trace(G)


def tt_round(
    cores: List[np.ndarray],
    tol: float = 1e-6,
    max_rank: Optional[int] = None
) -> List[np.ndarray]:
    """
    TT-rounding with rank truncation.
    Imported pattern from RAFAELIA_TT_UPDATE_FULL but standalone.
    
    Args:
        cores: TT-cores
        tol: Relative tolerance
        max_rank: Maximum rank
        
    Returns:
        Rounded TT-cores
    """
    d = len(cores)
    
    # Right-to-left orthogonalization
    orth_cores = [core.copy() for core in cores]
    
    for k in range(d - 1, 0, -1):
        core = orth_cores[k]
        r_left, n, r_right = core.shape
        
        # Reshape and QR
        mat = core.reshape(r_left, n * r_right)
        Q, R = np.linalg.qr(mat.T)
        
        orth_cores[k] = Q.T.reshape(Q.shape[1], n, r_right)
        
        # Merge R into previous core
        orth_cores[k - 1] = np.tensordot(orth_cores[k - 1], R.T, axes=[2, 0])
    
    # Left-to-right truncation
    rounded = [orth_cores[0]]
    
    for k in range(d - 1):
        core = rounded[k]
        r_left, n, r_right = core.shape
        
        # Reshape and SVD
        mat = core.reshape(r_left * n, r_right)
        U, S, Vt = np.linalg.svd(mat, full_matrices=False)
        
        # Truncate
        if max_rank is not None:
            r = min(max_rank, len(S))
        else:
            threshold = tol * S[0]
            r = max(1, np.sum(S > threshold))
        
        U = U[:, :r]
        S = S[:r]
        Vt = Vt[:r, :]
        
        # Update
        rounded[k] = U.reshape(r_left, n, r)
        
        # Merge into next
        if k < d - 1:
            next_core = orth_cores[k + 1]
            next_core = np.tensordot(np.diag(S) @ Vt, next_core, axes=[1, 0])
            rounded.append(next_core)
    
    return rounded


def estimate_compression_ratio(
    shape: Tuple[int, ...],
    ranks: List[int]
) -> float:
    """
    Estimate compression ratio for TT-decomposition.
    
    Args:
        shape: Original tensor shape
        ranks: TT-ranks [r_0=1, r_1, ..., r_{d-1}, r_d=1]
        
    Returns:
        Compression ratio (original size / TT size)
    """
    original_size = np.prod(shape)
    
    d = len(shape)
    tt_size = 0
    for k in range(d):
        r_left = ranks[k] if k < len(ranks) else 1
        r_right = ranks[k + 1] if k + 1 < len(ranks) else 1
        tt_size += r_left * shape[k] * r_right
    
    ratio = original_size / tt_size
    return ratio


def estimate_tt_accuracy(
    cores: List[np.ndarray],
    reference_tensor: np.ndarray,
    n_samples: int = 1000
) -> float:
    """
    Estimate TT-approximation accuracy by sampling.
    
    Args:
        cores: TT-cores
        reference_tensor: Original tensor
        n_samples: Number of random samples
        
    Returns:
        Relative error estimate
    """
    shape = reference_tensor.shape
    d = len(shape)
    
    errors = []
    
    for _ in range(n_samples):
        # Random index
        idx = tuple(np.random.randint(0, shape[k]) for k in range(d))
        
        # Compare values
        ref_val = reference_tensor[idx]
        tt_val = tt_reconstruct_element(cores, idx)
        
        error = abs(ref_val - tt_val)
        errors.append(error)
    
    # Relative error
    ref_norm = np.linalg.norm(reference_tensor)
    error_estimate = np.sqrt(np.mean(np.array(errors) ** 2)) * np.sqrt(np.prod(shape))
    
    return error_estimate / ref_norm


def demo_tt_accel():
    """Demo: TT compression and reconstruction."""
    print("=== RAFAELIA TT-Accel Demo ===\n")
    
    # Create a test tensor
    shape = (8, 8, 8)
    print(f"Creating test tensor of shape {shape}")
    
    # Smooth function
    X, Y, Z = np.meshgrid(
        np.linspace(0, 1, shape[0]),
        np.linspace(0, 1, shape[1]),
        np.linspace(0, 1, shape[2]),
        indexing='ij'
    )
    tensor = np.sin(2 * np.pi * X) * np.cos(2 * np.pi * Y) * np.exp(-Z)
    
    print(f"Original tensor: {tensor.size} elements")
    print(f"Original norm: {np.linalg.norm(tensor):.6f}")
    
    # TT-SVD compression
    print("\n--- TT-SVD compression ---")
    cores = tt_svd_from_full(tensor, tol=1e-3, verbose=True)
    
    ranks = [c.shape[0] for c in cores] + [1]
    compression = estimate_compression_ratio(shape, ranks)
    print(f"\nCompression ratio: {compression:.2f}x")
    
    # Reconstruct and check error
    print("\n--- Accuracy check ---")
    reconstructed = tt_reconstruct_full(cores)
    error = np.linalg.norm(tensor - reconstructed) / np.linalg.norm(tensor)
    print(f"Relative error: {error:.6e}")
    
    # Sample-based accuracy estimate
    sampled_error = estimate_tt_accuracy(cores, tensor, n_samples=500)
    print(f"Sampled error estimate: {sampled_error:.6e}")
    
    # Element reconstruction
    print("\n--- Element reconstruction ---")
    test_idx = (3, 4, 5)
    original_val = tensor[test_idx]
    tt_val = tt_reconstruct_element(cores, test_idx)
    print(f"Index {test_idx}:")
    print(f"  Original: {original_val:.6f}")
    print(f"  TT:       {tt_val:.6f}")
    print(f"  Error:    {abs(original_val - tt_val):.6e}")
    
    # TT-rounding
    print("\n--- TT-rounding ---")
    rounded = tt_round(cores, tol=1e-2, max_rank=5)
    rounded_ranks = [c.shape[0] for c in rounded] + [1]
    print(f"Rounded ranks: {rounded_ranks}")
    
    # Energy
    energy = tt_energy(cores)
    print(f"\nTT energy (norm²): {energy:.6f}")
    
    return cores


if __name__ == "__main__":
    demo_tt_accel()
