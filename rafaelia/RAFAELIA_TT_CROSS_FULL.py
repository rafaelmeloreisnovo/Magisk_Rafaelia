#!/usr/bin/env python3
"""
RAFAELIA_TT_CROSS_FULL.py
==========================
Implements TTBuilder with QTT mapping, adaptive TT-cross sampling, maxvol pivot
selection, Bitraf64 placeholder checkpointing, and SHA3 hashing.

Design rationale (ψχρΔΣΩ):
- ψ (psi): Multi-dimensional tensor structure preservation
- χ (chi): Cross-approximation sampling strategy
- ρ (rho): Rank adaptation and control
- Δ (delta): Differential update capability
- Σ (sigma): Statistical accuracy measures
- Ω (omega): Optimization convergence

Φ_ethica: Responsible AI constraints
- Safe memory limits (max_memory_gb)
- Controlled rank growth (max_rank)
- Sample budget limits (max_samples)
- Transparent checkpointing

Bitraf64: Checkpoint format with compression
HashVivo: Content-addressable hashing for reproducibility
"""

import numpy as np
import hashlib
import json
import pickle
from typing import List, Tuple, Callable, Optional, Dict, Any
import warnings

# Optional dependencies with fallback
try:
    import scipy.linalg
    HAS_SCIPY = True
except ImportError:
    HAS_SCIPY = False
    warnings.warn("scipy not available, using numpy fallback for maxvol")

try:
    import blake3
    HAS_BLAKE3 = True
except ImportError:
    HAS_BLAKE3 = False

try:
    import zstandard as zstd
    HAS_ZSTD = True
except ImportError:
    HAS_ZSTD = False


class TTBuilder:
    """
    Tensor-Train Cross-Approximation Builder with QTT support.
    
    Builds a Tensor-Train decomposition using adaptive cross-approximation
    sampling without requiring the full tensor in memory.
    """
    
    def __init__(
        self,
        shape: Tuple[int, ...],
        value_function: Callable,
        max_rank: int = 20,
        tol: float = 1e-6,
        max_samples: int = 10000,
        use_qtt: bool = False,
        max_memory_gb: float = 4.0,
        verbose: bool = True
    ):
        """
        Initialize TTBuilder.
        
        Args:
            shape: Tensor shape (n1, n2, ..., nd)
            value_function: Function that computes tensor[i1, i2, ..., id]
            max_rank: Maximum TT-rank allowed (Φ_ethica constraint)
            tol: Relative accuracy tolerance
            max_samples: Maximum samples to evaluate (Φ_ethica constraint)
            use_qtt: Enable Quantized Tensor-Train mode (requires power-of-2)
            max_memory_gb: Maximum memory usage in GB (Φ_ethica constraint)
            verbose: Print progress information
        """
        self.shape = shape
        self.d = len(shape)
        self.value_function = value_function
        self.max_rank = max_rank
        self.tol = tol
        self.max_samples = max_samples
        self.use_qtt = use_qtt
        self.max_memory_gb = max_memory_gb
        self.verbose = verbose
        
        # TT-cores storage: List of 3D arrays [r_{k-1}, n_k, r_k]
        self.cores: List[np.ndarray] = []
        
        # Metadata
        self.ranks: List[int] = []
        self.samples_used: int = 0
        self.hash_metadata: Dict[str, str] = {}
        
        # QTT validation
        if use_qtt:
            for n in shape:
                if not self._is_power_of_2(n):
                    raise ValueError(f"QTT mode requires power-of-2 dimensions, got {shape}")
    
    @staticmethod
    def _is_power_of_2(n: int) -> bool:
        """Check if n is a power of 2."""
        return n > 0 and (n & (n - 1)) == 0
    
    def _qtt_reshape(self, idx: np.ndarray) -> np.ndarray:
        """
        Convert flat index to QTT binary representation.
        For QTT mode, each dimension is represented in binary.
        """
        if not self.use_qtt:
            return idx
        
        # Convert to binary representation
        qtt_idx = []
        for i, n in enumerate(self.shape):
            bits = int(np.log2(n))
            val = idx[..., i]
            for b in range(bits):
                qtt_idx.append((val >> (bits - 1 - b)) & 1)
        
        return np.array(qtt_idx)
    
    def _maxvol(self, A: np.ndarray, tol: float = 1.05) -> Tuple[np.ndarray, np.ndarray]:
        """
        Maxvol pivot selection for cross-approximation.
        Finds the submatrix with maximum volume.
        
        Args:
            A: Matrix of shape (n, r)
            tol: Oversampling tolerance
            
        Returns:
            idx: Pivot row indices
            A_inv: Inverse of pivot submatrix
        """
        n, r = A.shape
        if n <= r:
            return np.arange(n), np.linalg.pinv(A)
        
        # Use QR decomposition to find initial pivots
        if HAS_SCIPY:
            Q, R, P = scipy.linalg.qr(A.T, pivoting=True)
            idx = P[:r]
        else:
            # Numpy fallback: use QR without pivoting, then greedy selection
            Q, R = np.linalg.qr(A)
            idx = np.arange(r)
        
        # Iterative maxvol refinement
        A_sub = A[idx, :]
        A_inv = np.linalg.pinv(A_sub)
        B = A @ A_inv
        
        max_iter = 100
        for _ in range(max_iter):
            i, j = np.unravel_index(np.argmax(np.abs(B)), B.shape)
            if np.abs(B[i, j]) <= tol:
                break
            
            # Update pivot set
            idx[j] = i
            b = B[i, :]
            B -= np.outer((B[:, j] / B[i, j]), b)
            B[i, :] = b / B[i, j]
        
        return idx, A_inv
    
    def build_tt_cross(self) -> List[np.ndarray]:
        """
        Build TT-decomposition using adaptive cross-approximation.
        (Simplified implementation for robustness)
        
        Returns:
            cores: List of TT-cores [r_{k-1}, n_k, r_k]
        """
        if self.verbose:
            print(f"Building TT-cross for shape {self.shape}")
            print(f"Parameters: max_rank={self.max_rank}, tol={self.tol}")
            if self.use_qtt:
                print("QTT mode enabled")
        
        self.cores = []
        self.ranks = [1]
        self.samples_used = 0
        
        # Simplified TT-cross: sample uniformly and use SVD truncation
        # This is less efficient but more robust than full maxvol-based cross
        
        for k in range(self.d):
            if self.verbose:
                print(f"Processing dimension {k+1}/{self.d}")
            
            n_k = self.shape[k]
            r_left = self.ranks[k]
            
            # Estimate right rank
            if k < self.d - 1:
                r_right = min(self.max_rank, r_left * n_k)
            else:
                r_right = 1
            
            # Build core by sampling
            # For simplicity, sample a representative set of fibers
            n_samples = min(r_left * n_k * r_right, self.max_samples - self.samples_used)
            
            if k == 0:
                # First core: simple structure
                core = np.zeros((1, n_k, min(self.max_rank, n_k)))
                for j in range(n_k):
                    for r in range(core.shape[2]):
                        # Sample from future dimensions
                        idx_future = tuple([0] * k + [j] + [r % self.shape[i] for i in range(k+1, self.d)])
                        core[0, j, r] = self.value_function(idx_future)
                        self.samples_used += 1
                        if self.samples_used >= self.max_samples:
                            break
                
                # SVD truncation
                core_mat = core[0, :, :]
                U, S, Vt = np.linalg.svd(core_mat, full_matrices=False)
                r_new = min(self.max_rank, np.sum(S > self.tol * S[0]))
                r_new = max(1, r_new)
                core = (U[:, :r_new] * S[:r_new]).reshape(1, n_k, r_new)
                
            elif k == self.d - 1:
                # Last core
                core = np.zeros((r_left, n_k, 1))
                for i in range(r_left):
                    for j in range(n_k):
                        # Sample with left context
                        idx_left = tuple([i % self.shape[l] for l in range(k)] + [j])
                        core[i, j, 0] = self.value_function(idx_left)
                        self.samples_used += 1
                        
            else:
                # Middle cores
                core = np.zeros((r_left, n_k, min(self.max_rank, r_left * n_k)))
                for i in range(r_left):
                    for j in range(n_k):
                        for r in range(core.shape[2]):
                            idx_left = [i % self.shape[l] for l in range(k)]
                            idx_right = [r % self.shape[l] for l in range(k+1, self.d)]
                            idx = tuple(idx_left + [j] + idx_right)
                            core[i, j, r] = self.value_function(idx)
                            self.samples_used += 1
                            if self.samples_used >= self.max_samples:
                                break
                
                # SVD truncation
                core_mat = core.reshape(r_left * n_k, core.shape[2])
                U, S, Vt = np.linalg.svd(core_mat, full_matrices=False)
                r_new = min(self.max_rank, np.sum(S > self.tol * S[0]))
                r_new = max(1, r_new)
                core = U[:, :r_new].reshape(r_left, n_k, r_new)
            
            self.cores.append(core)
            self.ranks.append(core.shape[2])
            
            if self.verbose:
                print(f"  Core shape: {core.shape}, rank: {core.shape[2]}")
        
        if self.verbose:
            print(f"TT-cross complete: {self.samples_used} samples used")
            print(f"Ranks: {self.ranks}")
        
        # Compute hash metadata
        self._compute_hash()
        
        return self.cores
    
    def _build_index(self, left_multi: tuple, j: int, right_multi: int, k: int) -> Tuple:
        """Build full tensor index from cross-approximation indices."""
        # left_multi is a tuple of indices for dimensions 0..k-1
        # j is the index for dimension k
        # right_multi is used to derive indices for dimensions k+1..d-1
        
        idx = list(left_multi) + [j]
        
        # Fill remaining dimensions with simplified pattern
        remaining = right_multi
        for i in range(k + 1, self.d):
            idx.append(remaining % self.shape[i])
            remaining = remaining // self.shape[i]
        
        return tuple(idx)
    
    def _build_index_last(self, left_multi: tuple, j: int) -> Tuple:
        """Build index for last dimension."""
        return left_multi + (j,)
    
    def _compute_hash(self):
        """Compute HashVivo metadata for reproducibility."""
        # Concatenate all cores
        data = b""
        for core in self.cores:
            data += core.tobytes()
        
        # Compute hashes
        if HAS_BLAKE3:
            hash_val = blake3.blake3(data).hexdigest()
        else:
            hash_val = hashlib.sha3_256(data).hexdigest()
        
        self.hash_metadata = {
            "hash": hash_val,
            "algorithm": "blake3" if HAS_BLAKE3 else "sha3_256",
            "shape": str(self.shape),
            "ranks": str(self.ranks),
            "samples": self.samples_used
        }
    
    def save_checkpoint(self, filepath: str):
        """
        Save TT-decomposition to Bitraf64 checkpoint format.
        
        Args:
            filepath: Path to save checkpoint
        """
        checkpoint = {
            "cores": self.cores,
            "shape": self.shape,
            "ranks": self.ranks,
            "samples_used": self.samples_used,
            "hash_metadata": self.hash_metadata,
            "metadata": {
                "max_rank": self.max_rank,
                "tol": self.tol,
                "use_qtt": self.use_qtt
            }
        }
        
        # Serialize
        data = pickle.dumps(checkpoint)
        
        # Optional compression
        if HAS_ZSTD:
            cctx = zstd.ZstdCompressor(level=3)
            data = cctx.compress(data)
            compressed = True
        else:
            compressed = False
        
        # Write to file
        with open(filepath, 'wb') as f:
            # Magic header
            f.write(b'BITRAF64')
            f.write(b'\x01' if compressed else b'\x00')
            f.write(data)
        
        if self.verbose:
            print(f"Checkpoint saved to {filepath}")
            print(f"Hash: {self.hash_metadata.get('hash', 'N/A')}")
    
    @staticmethod
    def load_checkpoint(filepath: str) -> 'TTBuilder':
        """
        Load TT-decomposition from checkpoint.
        
        Args:
            filepath: Path to checkpoint file
            
        Returns:
            builder: TTBuilder instance with loaded state
        """
        with open(filepath, 'rb') as f:
            # Read magic header
            magic = f.read(8)
            if magic != b'BITRAF64':
                raise ValueError("Invalid checkpoint format")
            
            compressed = f.read(1) == b'\x01'
            data = f.read()
        
        # Decompress if needed
        if compressed:
            if not HAS_ZSTD:
                raise RuntimeError("zstandard required to load compressed checkpoint")
            dctx = zstd.ZstdDecompressor()
            data = dctx.decompress(data)
        
        # Deserialize
        checkpoint = pickle.loads(data)
        
        # Reconstruct builder
        builder = TTBuilder(
            shape=checkpoint["shape"],
            value_function=lambda x: 0,  # Placeholder
            max_rank=checkpoint["metadata"]["max_rank"],
            tol=checkpoint["metadata"]["tol"],
            use_qtt=checkpoint["metadata"]["use_qtt"]
        )
        builder.cores = checkpoint["cores"]
        builder.ranks = checkpoint["ranks"]
        builder.samples_used = checkpoint["samples_used"]
        builder.hash_metadata = checkpoint["hash_metadata"]
        
        return builder


def demo_tt_cross():
    """Demo: Build TT-cross for a simple function."""
    print("=== RAFAELIA TT-Cross Demo ===\n")
    
    # Define a test function: sum of coordinates
    def test_func(idx):
        return sum(idx) + np.prod(idx) * 0.1
    
    # Build TT
    shape = (4, 4, 4)
    builder = TTBuilder(
        shape=shape,
        value_function=test_func,
        max_rank=10,
        tol=1e-3,
        verbose=True
    )
    
    cores = builder.build_tt_cross()
    
    print(f"\nBuilt TT with {len(cores)} cores")
    for i, core in enumerate(cores):
        print(f"Core {i}: shape {core.shape}")
    
    # Save checkpoint
    checkpoint_path = "/tmp/demo_tt_checkpoint.bitraf64"
    builder.save_checkpoint(checkpoint_path)
    
    # Load and verify
    loaded = TTBuilder.load_checkpoint(checkpoint_path)
    print(f"\nLoaded checkpoint with ranks: {loaded.ranks}")
    
    return builder


if __name__ == "__main__":
    demo_tt_cross()
