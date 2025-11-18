# RAFAELIA Tensor-Train Fullstack Suite

A comprehensive toolkit for high-order tensor decomposition using Tensor-Train (TT) format with cross-approximation, local updates, and optimization.

## Overview

The RAFAELIA TT suite provides:

- **TT-Cross Builder**: Adaptive cross-approximation for memory-efficient TT construction
- **QTT Support**: Quantized Tensor-Train for binary-structured problems
- **Local Patch Updates**: Incremental modifications with ALS and SOC solvers
- **TT-Rounding**: Rank reduction with controlled truncation
- **Spiral/Fibonacci Generators**: Structured test functions for validation
- **TT-Accel**: Fast SVD-based compression for small tensors
- **Orchestrator Engine**: CLI and REST API for complete workflows

## Design Rationale

### ψχρΔΣΩ Principles

- **ψ (psi)**: Multi-dimensional tensor structure preservation
- **χ (chi)**: Cross-approximation sampling strategy
- **ρ (rho)**: Rank adaptation and control
- **Δ (delta)**: Differential/incremental update capability
- **Σ (sigma)**: Statistical accuracy measures
- **Ω (omega)**: Optimization convergence

### Φ_ethica: Responsible AI Constraints

- **Safe memory limits**: `max_memory_gb` prevents OOM
- **Controlled rank growth**: `max_rank` bounds complexity
- **Sample budget limits**: `max_samples` for resource control
- **Transparent checkpointing**: Bitraf64 format with compression
- **Content-addressable hashing**: HashVivo for reproducibility

### Bitraf64 Checkpoint Format

Checkpoint files use a custom format:
- Magic header: `BITRAF64`
- Compression flag (optional zstandard)
- Serialized TT-cores and metadata
- HashVivo content hash (blake3 or SHA3-256)

### HashVivo Metadata

Each TT decomposition includes:
- Hash of concatenated core data
- Algorithm identifier (blake3 preferred)
- Shape and rank information
- Sample count for reproducibility

## Installation

### Core Dependencies (Required)

```bash
pip install numpy
```

### Optional Dependencies

```bash
# For improved performance and features
pip install scipy          # Better maxvol pivot selection
pip install blake3         # Faster hashing than SHA3
pip install zstandard      # Checkpoint compression
pip install flask          # REST API server
pip install cupy           # GPU acceleration (future)
pip install numba          # JIT compilation (future)
```

**Note**: All optional dependencies are gracefully handled. The suite falls back to numpy-only implementations when optional packages are unavailable.

## Quick Start

### 1. Build TT from Generator

```python
from RAFAELIA_TT_CROSS_FULL import TTBuilder
from RAFAELIA_SPIRAL_FIBONACCI import value_generator

# Create value function
shape = (8, 8, 8)
value_func = value_generator("fibonacci_spiral", shape=shape)

# Build TT using cross-approximation
builder = TTBuilder(
    shape=shape,
    value_function=value_func,
    max_rank=20,
    tol=1e-6,
    max_samples=10000,
    verbose=True
)

cores = builder.build_tt_cross()

# Save checkpoint
builder.save_checkpoint("my_tt.bitraf64")
```

### 2. Load and Update TT

```python
from RAFAELIA_TT_CROSS_FULL import TTBuilder
from RAFAELIA_TT_UPDATE_FULL import local_patch_update_tt
import numpy as np

# Load checkpoint
builder = TTBuilder.load_checkpoint("my_tt.bitraf64")

# Define patch region and new data
patch_indices = (slice(2, 5), slice(2, 5), slice(2, 5))
patch_data = np.random.rand(3, 3, 3)

# Update TT to fit patch
updated_cores = local_patch_update_tt(
    builder.cores,
    patch_data,
    patch_indices,
    method="als",
    max_iter=10
)

builder.cores = updated_cores
builder.save_checkpoint("my_tt_updated.bitraf64")
```

### 3. Use the Orchestrator Engine

#### Demo Mode

```bash
cd rafaelia
python RAFAELIA_ENGINE_FULLSTACK.py demo
```

#### Build TT from CLI

```bash
python RAFAELIA_ENGINE_FULLSTACK.py build \
    --shape 10,10,10 \
    --generator fibonacci_spiral \
    --max-rank 15 \
    --checkpoint my_build
```

#### Apply Patch Update

```bash
python RAFAELIA_ENGINE_FULLSTACK.py patch \
    --checkpoint my_build
```

#### REST API Server

```bash
python RAFAELIA_ENGINE_FULLSTACK.py serve --port 5000
```

Then use curl:

```bash
# Check status
curl http://localhost:5000/status

# Build TT
curl -X POST http://localhost:5000/build \
    -H "Content-Type: application/json" \
    -d '{"shape": [8,8,8], "generator": "gaussian_blobs"}'

# Load checkpoint
curl -X POST http://localhost:5000/load \
    -H "Content-Type: application/json" \
    -d '{"checkpoint": "my_build"}'
```

## Recommended Parameters

### Small Tensors (< 10^6 elements)

- `max_rank`: 10-20
- `tol`: 1e-4 to 1e-6
- `max_samples`: 1000-5000
- Use TT-SVD (RAFAELIA_TT_ACCEL) if full tensor fits in memory

### Medium Tensors (10^6 - 10^9 elements)

- `max_rank`: 20-50
- `tol`: 1e-6 to 1e-8
- `max_samples`: 10000-50000
- TT-cross recommended

### Large Tensors (> 10^9 elements)

- `max_rank`: 30-100
- `tol`: 1e-8 to 1e-10
- `max_samples`: 50000-200000
- Enable QTT if dimensions are powers of 2
- Consider GPU acceleration (future)

### Patch Update Parameters

- `method`: "als" (stable), "soc" (faster), "direct" (simple)
- `max_iter`: 5-20 for ALS
- Patch size: 10-30% of each dimension
- Re-round after multiple patches: `tol=1e-4, max_rank=...`

## Module Reference

### RAFAELIA_TT_CROSS_FULL.py

**TTBuilder class**:
- `build_tt_cross()`: Adaptive TT-cross construction
- `save_checkpoint(path)`: Save to Bitraf64 format
- `load_checkpoint(path)`: Load from checkpoint

**Features**:
- Maxvol pivot selection
- QTT mode for binary-structured tensors
- HashVivo content addressing
- Optional zstandard compression

### RAFAELIA_TT_UPDATE_FULL.py

**Functions**:
- `local_patch_update_tt(cores, patch_data, indices, ...)`: Update TT with new patch
- `tt_round(cores, tol, max_rank)`: Rank truncation
- `tt_reconstruct_slice(cores, indices)`: Extract tensor slice
- `compute_patch_error(cores, patch_data, indices)`: Error estimation

**Update Methods**:
- `"als"`: Alternating Least Squares (stable, iterative)
- `"soc"`: Second-Order Correction (faster convergence)
- `"direct"`: Direct substitution (simple, may increase ranks)

### RAFAELIA_SPIRAL_FIBONACCI.py

**Generators**:
- `fibonacci_spiral`: Smooth spiral patterns
- `gaussian_blobs`: Random Gaussian fields
- `polynomial`: Polynomial test functions
- `sinusoidal`: Multi-frequency oscillations

**Functions**:
- `value_generator(type, **kwargs)`: Factory for value functions
- `generate_fibonacci_spiral(n_points, n_turns, ...)`: 2D spiral coordinates
- `embed_2d_to_nd(coords, target_dim, method)`: Extend to higher dimensions

### RAFAELIA_TT_ACCEL.py

**Functions**:
- `tt_svd_from_full(tensor, tol, max_rank)`: TT-SVD compression
- `tt_reconstruct_full(cores)`: Full tensor reconstruction (use with caution!)
- `tt_reconstruct_element(cores, idx)`: Single element
- `estimate_compression_ratio(shape, ranks)`: Theoretical compression
- `estimate_tt_accuracy(cores, reference, n_samples)`: Sampling-based error

### RAFAELIA_ENGINE_FULLSTACK.py

**RAFAELIAEngine class**:
- `build_tt(shape, generator, ...)`: Build TT from generator
- `load_checkpoint(name)`: Load TT from checkpoint
- `apply_patch(patch_data, indices, ...)`: Apply patch update
- `round_tt(tol, max_rank)`: Round current TT
- `get_status()`: Get engine state
- `demo_workflow()`: Run complete demo

**CLI Modes**:
- `demo`: Full workflow demonstration
- `build`: Build TT and save checkpoint
- `patch`: Load and apply patch update
- `serve`: Start REST API server

## Advanced Topics

### QTT (Quantized Tensor-Train)

For tensors with power-of-2 dimensions, QTT provides better compression by representing each dimension in binary:

```python
# Standard TT for (32, 32, 32) tensor
builder = TTBuilder(shape=(32, 32, 32), ..., use_qtt=False)
# Ranks might be: [1, 15, 15, 1]

# QTT mode (32 = 2^5)
builder = TTBuilder(shape=(32, 32, 32), ..., use_qtt=True)
# Effective shape: (2,2,2,2,2, 2,2,2,2,2, 2,2,2,2,2) = 15 dimensions
# Ranks might be: [1, 2, 4, 4, 4, 2, 2, 4, 4, 4, 2, 2, 4, 4, 4, 1]
# Often achieves better compression for structured data
```

### Mixed Precision

Future enhancement: support for mixed-precision computation (FP16/FP32/FP64) to trade accuracy for memory/speed.

### Checkpoint Migration

```python
# Load old checkpoint
builder = TTBuilder.load_checkpoint("old.bitraf64")

# Re-round with new tolerance
cores = tt_round(builder.cores, tol=1e-4, max_rank=10)
builder.cores = cores

# Save with new parameters
builder.save_checkpoint("new_compressed.bitraf64")
```

## Safety and Φ_ethica

### Resource Limits

Always set appropriate limits:

```python
builder = TTBuilder(
    ...,
    max_rank=50,          # Prevent rank explosion
    max_samples=100000,   # Limit computation
    max_memory_gb=8.0     # Prevent OOM
)
```

### Checkpoint Security

⚠️ **Important**: Checkpoints use pickle serialization. Only load checkpoints from trusted sources.

- Checkpoints may contain arbitrary Python objects
- HashVivo provides integrity checking but not authentication
- Store checkpoints securely (encrypted filesystem if sensitive)
- Do not share checkpoints containing proprietary data

### Token Protection

When using REST API:

- Do not hardcode credentials in code
- Use environment variables or secure vaults
- Enable HTTPS for production deployments
- Implement authentication middleware (not included in stub)

### Audit Logging

The engine maintains an audit log:

```python
engine = RAFAELIAEngine(...)
# ... perform operations ...

# Review audit trail
for entry in engine.audit_log:
    print(entry)
```

## Testing

Run smoke tests:

```bash
cd rafaelia/tests
python test_smoke.py
```

Individual module demos:

```bash
python RAFAELIA_TT_CROSS_FULL.py
python RAFAELIA_TT_UPDATE_FULL.py
python RAFAELIA_SPIRAL_FIBONACCI.py
python RAFAELIA_TT_ACCEL.py
python RAFAELIA_ENGINE_FULLSTACK.py demo
```

## Troubleshooting

### Memory Issues

- Reduce `max_rank`
- Increase `tol` (less accurate but faster)
- Use QTT mode for power-of-2 dimensions
- Process in smaller patches

### Poor Accuracy

- Decrease `tol`
- Increase `max_rank`
- Increase `max_samples` for TT-cross
- Choose smoother generator for testing

### Slow Performance

- Install scipy for better maxvol
- Install blake3 for faster hashing
- Reduce `max_samples`
- Use TT-SVD for small tensors instead of TT-cross

### Import Errors

The suite is designed to work from the rafaelia/ directory:

```bash
cd /path/to/repo/rafaelia
python RAFAELIA_ENGINE_FULLSTACK.py demo
```

Or install as a package (create setup.py):

```python
from setuptools import setup
setup(
    name="rafaelia",
    packages=["rafaelia"],
    install_requires=["numpy"]
)
```

## Contributing

This is an experimental research suite. Contributions welcome:

- Improved ALS/SOC solvers
- GPU acceleration with CuPy
- Better maxvol algorithms
- Parallel sampling strategies
- Adaptive rank selection
- More sophisticated generators

## License

See repository LICENSE file.

## References

- Oseledets, I. V. (2011). "Tensor-train decomposition"
- Dolgov, S., & Savostyanov, D. (2014). "Alternating minimal energy methods for linear systems in higher dimensions"
- Savostyanov, D. V. (2011). "QTT-rank-one vectors"

## Contact

For issues and questions, please use the repository issue tracker.
