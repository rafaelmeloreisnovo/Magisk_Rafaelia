#!/usr/bin/env python3
"""
RAFAELIA Core Module - Tensor Train Algorithms

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved

This module provides core TT-cross approximation and local update algorithms.

Module 1: Core Tensor Train Algorithms
- TT-cross approximation with maxvol index selection
- Local updates via ALS sweeps and rank adaptation
- Gradient-based core updates

Legal Status: Original works of authorship, fully copyrighted
Rights: All economic and moral rights reserved to Rafael Melo Reis
Best Practices: Clean code architecture, comprehensive documentation, safe fallbacks
Ethical Considerations: AI safety, transparency, responsible deployment

Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
Philosophy: VAZIO → VERBO → CHEIO → RETRO
"""

from rafaelia.core.tt_cross import TTCrossApproximation
from rafaelia.core.tt_update import TTLocalUpdate
from rafaelia.core.matrix_ops import (
    MatrixOperations,
    TensorTrainMatrix,
    AdaptiveMatrixOperations,
)
from rafaelia.core.authorship import (
    RafaeliaAuthorship,
    Author,
    Publication,
    LegalFramework,
)
from rafaelia.core.interop import (
    Version,
    VersionCompatibility,
    VersionRegistry,
    PlatformCapabilities,
    InteroperabilityLayer,
    ApplicabilityScore,
    ApplicabilityChecker,
    MitigationStrategy,
)
from rafaelia.core.cognitive import (
    ComputationPattern,
    ExecutionProfile,
    PatternRecognizer,
    OptimizationObjective,
    MultiObjectiveOptimizer,
    AdaptiveTuner,
    FractalOptimizer,
    ResourceLoad,
    CognitiveLoadBalancer,
)

__all__ = [
    # Original TT algorithms
    'TTCrossApproximation',
    'TTLocalUpdate',
    # Low-level matrix operations
    'MatrixOperations',
    'TensorTrainMatrix',
    'AdaptiveMatrixOperations',
    # Authorship and legal
    'RafaeliaAuthorship',
    'Author',
    'Publication',
    'LegalFramework',
    # Interoperability and versioning
    'Version',
    'VersionCompatibility',
    'VersionRegistry',
    'PlatformCapabilities',
    'InteroperabilityLayer',
    'ApplicabilityScore',
    'ApplicabilityChecker',
    'MitigationStrategy',
    # Cognitive optimization
    'ComputationPattern',
    'ExecutionProfile',
    'PatternRecognizer',
    'OptimizationObjective',
    'MultiObjectiveOptimizer',
    'AdaptiveTuner',
    'FractalOptimizer',
    'ResourceLoad',
    'CognitiveLoadBalancer',
]
