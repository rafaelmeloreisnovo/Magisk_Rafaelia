#!/usr/bin/env python3
"""
RAFAELIA_SPIRAL_FIBONACCI.py
=============================
Spiral and Fibonacci generators for value functions in TT-cross sampling.

Provides geometric spiral patterns, Fibonacci sequences, inverse spirals,
and Gaussian blob field generators for testing and visualization.

Design rationale:
- Provides smooth, structured test functions
- Multi-dimensional embedding via coordinate mapping
- Adjustable complexity for benchmarking

Φ_ethica: Safe value generation
- Bounded output ranges
- No divergent sequences
- Deterministic reproducibility
"""

import numpy as np
from typing import Tuple, Callable, Optional
import math


def fibonacci_sequence(n: int) -> np.ndarray:
    """
    Generate Fibonacci sequence up to n terms.
    
    Args:
        n: Number of terms
        
    Returns:
        Array of Fibonacci numbers
    """
    if n <= 0:
        return np.array([])
    if n == 1:
        return np.array([1])
    
    fib = np.zeros(n, dtype=np.int64)
    fib[0] = 1
    fib[1] = 1
    
    for i in range(2, n):
        fib[i] = fib[i - 1] + fib[i - 2]
    
    return fib


def golden_ratio_spiral(t: np.ndarray, a: float = 1.0, b: float = 0.1) -> Tuple[np.ndarray, np.ndarray]:
    """
    Generate 2D golden ratio (Fibonacci) spiral coordinates.
    
    Uses the logarithmic spiral: r = a * exp(b * θ)
    with θ scaled by golden ratio.
    
    Args:
        t: Parameter values (0 to 2π * n_turns typically)
        a: Scale factor
        b: Growth rate
        
    Returns:
        (x, y): Spiral coordinates
    """
    phi = (1 + np.sqrt(5)) / 2  # Golden ratio
    
    # Scale angle by golden ratio for Fibonacci property
    theta = t * phi
    r = a * np.exp(b * theta)
    
    x = r * np.cos(theta)
    y = r * np.sin(theta)
    
    return x, y


def generate_fibonacci_spiral(
    n_points: int = 100,
    n_turns: float = 3.0,
    spiral_type: str = "golden",
    normalize: bool = True
) -> np.ndarray:
    """
    Generate Fibonacci spiral coordinates.
    
    Args:
        n_points: Number of points on spiral
        n_turns: Number of spiral turns
        spiral_type: "golden", "archimedean", or "fermat"
        normalize: Normalize coordinates to [0, 1]
        
    Returns:
        Array of shape (n_points, 2) with (x, y) coordinates
    """
    t = np.linspace(0, 2 * np.pi * n_turns, n_points)
    
    if spiral_type == "golden":
        x, y = golden_ratio_spiral(t, a=1.0, b=0.15)
    
    elif spiral_type == "archimedean":
        # Linear growth: r = a + b * θ
        theta = t
        r = 0.1 + 0.3 * theta
        x = r * np.cos(theta)
        y = r * np.sin(theta)
    
    elif spiral_type == "fermat":
        # Fermat's spiral (parabolic): r² = a² * θ
        theta = t
        r = np.sqrt(0.5 * theta)
        x = r * np.cos(theta)
        y = r * np.sin(theta)
    
    else:
        raise ValueError(f"Unknown spiral type: {spiral_type}")
    
    coords = np.column_stack([x, y])
    
    if normalize:
        # Normalize to [0, 1]
        coords_min = coords.min(axis=0)
        coords_max = coords.max(axis=0)
        coords = (coords - coords_min) / (coords_max - coords_min + 1e-10)
    
    return coords


def inverse_spiral_distance(
    idx: Tuple[int, ...],
    shape: Tuple[int, ...],
    n_turns: float = 2.0
) -> float:
    """
    Compute distance along inverse spiral for multi-dimensional index.
    
    Maps high-dimensional index to a spiral path, computing distance from origin.
    Useful for creating smoothly varying test functions.
    
    Args:
        idx: Multi-dimensional index
        shape: Tensor shape
        n_turns: Number of spiral turns
        
    Returns:
        Distance value in [0, 1]
    """
    d = len(idx)
    
    # Map index to [0, 1]^d
    normalized = np.array([idx[i] / max(1, shape[i] - 1) for i in range(d)])
    
    # Project to 2D using first two dimensions
    if d >= 2:
        x, y = normalized[0], normalized[1]
    else:
        x = normalized[0]
        y = 0.5
    
    # Add contribution from higher dimensions as radial offset
    if d > 2:
        z_contrib = np.mean(normalized[2:])
        r_offset = 0.2 * z_contrib
    else:
        r_offset = 0.0
    
    # Compute spiral angle
    phi = (1 + np.sqrt(5)) / 2
    theta = math.atan2(y - 0.5, x - 0.5)
    r = math.sqrt((x - 0.5) ** 2 + (y - 0.5) ** 2) + r_offset
    
    # Inverse spiral: map (r, θ) back to parameter t
    if r > 1e-6:
        # Approximate inverse of r = exp(b*θ)
        b = 0.15
        t = (math.log(r + 0.1) / b) % (2 * math.pi * n_turns)
    else:
        t = 0.0
    
    # Normalize to [0, 1]
    distance = t / (2 * math.pi * n_turns)
    
    return distance


def gaussian_blob_field(
    idx: Tuple[int, ...],
    shape: Tuple[int, ...],
    n_blobs: int = 5,
    sigma: float = 0.15,
    seed: int = 42
) -> float:
    """
    Generate a field of Gaussian blobs at random locations.
    
    Args:
        idx: Multi-dimensional index
        shape: Tensor shape
        n_blobs: Number of Gaussian blobs
        sigma: Blob width
        seed: Random seed for reproducibility
        
    Returns:
        Field value at index
    """
    np.random.seed(seed)
    
    d = len(idx)
    
    # Generate blob centers
    centers = np.random.rand(n_blobs, d)
    
    # Normalize index to [0, 1]
    normalized = np.array([idx[i] / max(1, shape[i] - 1) for i in range(d)])
    
    # Sum Gaussian contributions
    value = 0.0
    for i in range(n_blobs):
        center = centers[i]
        dist_sq = np.sum((normalized - center) ** 2)
        value += np.exp(-dist_sq / (2 * sigma ** 2))
    
    return value


def value_generator(
    generator_type: str = "fibonacci_spiral",
    **kwargs
) -> Callable:
    """
    Factory function to create value generators for TT-cross.
    
    Args:
        generator_type: Type of generator
            - "fibonacci_spiral": Fibonacci spiral distance
            - "gaussian_blobs": Gaussian blob field
            - "polynomial": Simple polynomial
            - "sinusoidal": Multi-frequency sinusoid
        **kwargs: Additional parameters for specific generators
        
    Returns:
        Value function: idx -> float
    """
    if generator_type == "fibonacci_spiral":
        n_turns = kwargs.get("n_turns", 2.0)
        
        def func(idx):
            if not isinstance(idx, tuple):
                idx = tuple(idx)
            shape = kwargs.get("shape", tuple([10] * len(idx)))
            return inverse_spiral_distance(idx, shape, n_turns)
        
        return func
    
    elif generator_type == "gaussian_blobs":
        n_blobs = kwargs.get("n_blobs", 5)
        sigma = kwargs.get("sigma", 0.15)
        seed = kwargs.get("seed", 42)
        
        def func(idx):
            if not isinstance(idx, tuple):
                idx = tuple(idx)
            shape = kwargs.get("shape", tuple([10] * len(idx)))
            return gaussian_blob_field(idx, shape, n_blobs, sigma, seed)
        
        return func
    
    elif generator_type == "polynomial":
        degree = kwargs.get("degree", 2)
        
        def func(idx):
            if not isinstance(idx, tuple):
                idx = tuple(idx)
            # Polynomial of index components
            value = sum((i + 1) ** degree for i in idx) / len(idx)
            return value / 100.0  # Normalize
        
        return func
    
    elif generator_type == "sinusoidal":
        frequencies = kwargs.get("frequencies", [1.0, 2.0, 3.0])
        
        def func(idx):
            if not isinstance(idx, tuple):
                idx = tuple(idx)
            value = 0.0
            for i, freq in enumerate(frequencies[:len(idx)]):
                value += np.sin(2 * np.pi * freq * idx[i] / 10.0)
            return value / len(idx)
        
        return func
    
    else:
        raise ValueError(f"Unknown generator type: {generator_type}")


def embed_2d_to_nd(
    coords_2d: np.ndarray,
    target_dim: int,
    method: str = "repeat"
) -> np.ndarray:
    """
    Embed 2D coordinates into N-dimensional space.
    
    Args:
        coords_2d: Array of shape (n_points, 2)
        target_dim: Target dimensionality
        method: Embedding method
            - "repeat": Repeat 2D pattern
            - "helical": Helical extension
            - "random": Add random dimensions
            
    Returns:
        Array of shape (n_points, target_dim)
    """
    n_points = coords_2d.shape[0]
    
    if target_dim <= 2:
        return coords_2d[:, :target_dim]
    
    coords_nd = np.zeros((n_points, target_dim))
    coords_nd[:, :2] = coords_2d
    
    if method == "repeat":
        # Repeat pattern in higher dimensions
        for d in range(2, target_dim):
            coords_nd[:, d] = coords_2d[:, d % 2]
    
    elif method == "helical":
        # Helical extension
        t = np.linspace(0, 2 * np.pi, n_points)
        for d in range(2, target_dim):
            coords_nd[:, d] = 0.5 + 0.3 * np.sin(t * (d - 1))
    
    elif method == "random":
        # Random projection
        np.random.seed(42)
        coords_nd[:, 2:] = np.random.rand(n_points, target_dim - 2)
    
    return coords_nd


def demo_generators():
    """Demo: Test various value generators."""
    print("=== RAFAELIA Spiral/Fibonacci Generators Demo ===\n")
    
    # Test Fibonacci sequence
    fib = fibonacci_sequence(10)
    print(f"Fibonacci sequence (10 terms): {fib}")
    
    # Test spiral generation
    spiral = generate_fibonacci_spiral(n_points=50, n_turns=2.5, spiral_type="golden")
    print(f"\nGenerated golden spiral: {spiral.shape[0]} points")
    print(f"  Range X: [{spiral[:, 0].min():.3f}, {spiral[:, 0].max():.3f}]")
    print(f"  Range Y: [{spiral[:, 1].min():.3f}, {spiral[:, 1].max():.3f}]")
    
    # Test value generator
    print("\n--- Testing value generators ---")
    
    shape = (8, 8, 8)
    test_indices = [(0, 0, 0), (4, 4, 4), (7, 7, 7)]
    
    for gen_type in ["fibonacci_spiral", "gaussian_blobs", "polynomial", "sinusoidal"]:
        gen = value_generator(gen_type, shape=shape)
        print(f"\n{gen_type}:")
        for idx in test_indices:
            val = gen(idx)
            print(f"  {idx}: {val:.6f}")
    
    # Test embedding
    print("\n--- Testing embedding ---")
    spiral_2d = generate_fibonacci_spiral(n_points=20, normalize=True)
    spiral_5d = embed_2d_to_nd(spiral_2d, target_dim=5, method="helical")
    print(f"Embedded 2D spiral into 5D: {spiral_5d.shape}")
    
    return spiral


if __name__ == "__main__":
    demo_generators()
