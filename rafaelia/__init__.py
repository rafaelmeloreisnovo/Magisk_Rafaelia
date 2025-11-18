"""
RAFAELIA Tensor-Train Fullstack Suite
======================================

A comprehensive toolkit for high-order tensor decomposition using Tensor-Train
format with cross-approximation, local updates, and optimization.

Main modules:
- RAFAELIA_TT_CROSS_FULL: TT-cross builder with QTT support
- RAFAELIA_TT_UPDATE_FULL: Local patch updates and TT-rounding
- RAFAELIA_SPIRAL_FIBONACCI: Value generators for testing
- RAFAELIA_TT_ACCEL: TT-SVD utilities for small tensors
- RAFAELIA_ENGINE_FULLSTACK: Orchestrator with CLI and REST API

Quick start:
    from rafaelia.RAFAELIA_TT_CROSS_FULL import TTBuilder
    from rafaelia.RAFAELIA_SPIRAL_FIBONACCI import value_generator
    
    shape = (8, 8, 8)
    gen = value_generator("fibonacci_spiral", shape=shape)
    builder = TTBuilder(shape, gen, max_rank=20, tol=1e-6)
    cores = builder.build_tt_cross()
"""

__version__ = "0.1.0"
__author__ = "RAFAELIA Team"

# Import key classes and functions for convenience
from rafaelia.RAFAELIA_TT_CROSS_FULL import TTBuilder
from rafaelia.RAFAELIA_SPIRAL_FIBONACCI import value_generator
from rafaelia.RAFAELIA_ENGINE_FULLSTACK import RAFAELIAEngine
from rafaelia.RAFAELIA_TT_UPDATE_FULL import local_patch_update_tt, tt_round
from rafaelia.RAFAELIA_TT_ACCEL import tt_svd_from_full

__all__ = [
    "TTBuilder",
    "value_generator",
    "RAFAELIAEngine",
    "local_patch_update_tt",
    "tt_round",
    "tt_svd_from_full",
]
