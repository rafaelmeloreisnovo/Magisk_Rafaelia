# Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
# Instituto Rafael - CientiEspiritual Philosophy
#
# This file is part of Magisk_Rafaelia.
#
# Magisk_Rafaelia is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.

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
