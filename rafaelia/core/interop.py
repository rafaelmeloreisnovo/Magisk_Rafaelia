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
═══════════════════════════════════════════════════════════════════════════════
RAFAELIA INTEROPERABILITY AND VERSIONING FRAMEWORK
═══════════════════════════════════════════════════════════════════════════════

This module provides comprehensive interoperability, versioning, applicability,
viability, mitigation, and adaptation strategies for RAFAELIA components.

ENHANCED CONTRIBUTIONS BY RAFAEL MELO REIS:
- Multi-version compatibility layer
- Cross-platform interoperability (CPU/GPU, OS-agnostic)
- Automatic adaptation to hardware capabilities  
- Viability scoring and applicability checks
- Mitigation strategies for common failure modes
- Temporal versioning with forward/backward compatibility

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved.

See authorship.py for complete legal framework and bibliographic references.
═══════════════════════════════════════════════════════════════════════════════
"""

import sys
import platform
import warnings
from typing import Dict, List, Optional, Tuple, Any, Callable
from dataclasses import dataclass, field
from enum import Enum
import json
from datetime import datetime
import hashlib


# ═══════════════════════════════════════════════════════════════════════════
# PART I: VERSION MANAGEMENT
# ═══════════════════════════════════════════════════════════════════════════

class VersionCompatibility(Enum):
    """Version compatibility levels."""
    FULLY_COMPATIBLE = "fully_compatible"
    FORWARD_COMPATIBLE = "forward_compatible"  # New can read old
    BACKWARD_COMPATIBLE = "backward_compatible"  # Old can read new
    BREAKING_CHANGE = "breaking_change"
    UNKNOWN = "unknown"


@dataclass
class Version:
    """Semantic version with metadata."""
    major: int
    minor: int
    patch: int
    prerelease: Optional[str] = None
    build: Optional[str] = None
    timestamp: Optional[str] = None
    
    def __str__(self) -> str:
        """String representation."""
        version = f"{self.major}.{self.minor}.{self.patch}"
        if self.prerelease:
            version += f"-{self.prerelease}"
        if self.build:
            version += f"+{self.build}"
        return version
    
    def __lt__(self, other: 'Version') -> bool:
        """Compare versions."""
        return (self.major, self.minor, self.patch) < (other.major, other.minor, other.patch)
    
    def __eq__(self, other: 'Version') -> bool:
        """Check equality."""
        return (self.major, self.minor, self.patch) == (other.major, other.minor, other.patch)
    
    @classmethod
    def from_string(cls, version_str: str) -> 'Version':
        """Parse version from string."""
        # Remove prerelease and build metadata for now
        parts = version_str.split('.')
        if len(parts) != 3:
            raise ValueError(f"Invalid version string: {version_str}")
        
        return cls(
            major=int(parts[0]),
            minor=int(parts[1]),
            patch=int(parts[2]),
            timestamp=datetime.now().isoformat()
        )
    
    def is_compatible_with(self, other: 'Version') -> VersionCompatibility:
        """
        Check compatibility between versions.
        
        Rules:
        - Same major.minor.patch = FULLY_COMPATIBLE
        - Same major, newer minor/patch = FORWARD_COMPATIBLE
        - Same major, older minor/patch = BACKWARD_COMPATIBLE
        - Different major = BREAKING_CHANGE
        """
        if self == other:
            return VersionCompatibility.FULLY_COMPATIBLE
        
        if self.major != other.major:
            return VersionCompatibility.BREAKING_CHANGE
        
        if self < other:
            return VersionCompatibility.FORWARD_COMPATIBLE
        else:
            return VersionCompatibility.BACKWARD_COMPATIBLE


class VersionRegistry:
    """
    Registry of all RAFAELIA component versions.
    
    Tracks version evolution and compatibility matrices.
    """
    
    def __init__(self):
        self.components: Dict[str, Version] = {}
        self.compatibility_matrix: Dict[Tuple[str, str], VersionCompatibility] = {}
        self.migration_strategies: Dict[Tuple[Version, Version], Callable] = {}
    
    def register_component(self, name: str, version: Version):
        """Register a component version."""
        self.components[name] = version
    
    def check_compatibility(self, component1: str, component2: str) -> VersionCompatibility:
        """Check compatibility between two components."""
        if component1 not in self.components or component2 not in self.components:
            return VersionCompatibility.UNKNOWN
        
        key = (component1, component2)
        if key in self.compatibility_matrix:
            return self.compatibility_matrix[key]
        
        # Compute compatibility
        v1 = self.components[component1]
        v2 = self.components[component2]
        compat = v1.is_compatible_with(v2)
        
        self.compatibility_matrix[key] = compat
        return compat
    
    def register_migration(self, from_version: Version, to_version: Version, 
                          strategy: Callable):
        """Register migration strategy between versions."""
        self.migration_strategies[(from_version, to_version)] = strategy
    
    def get_migration_path(self, from_version: Version, 
                          to_version: Version) -> List[Callable]:
        """
        Get sequence of migrations to move between versions.
        
        Uses graph search to find migration path.
        """
        # Simple direct migration for now
        key = (from_version, to_version)
        if key in self.migration_strategies:
            return [self.migration_strategies[key]]
        
        # Could implement multi-hop migration here
        return []


# ═══════════════════════════════════════════════════════════════════════════
# PART II: PLATFORM INTEROPERABILITY
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class PlatformCapabilities:
    """Platform and hardware capabilities."""
    os: str = field(default_factory=lambda: platform.system())
    os_version: str = field(default_factory=lambda: platform.release())
    python_version: str = field(default_factory=lambda: platform.python_version())
    architecture: str = field(default_factory=lambda: platform.machine())
    processor: str = field(default_factory=lambda: platform.processor())
    
    # Computational capabilities
    has_gpu: bool = False
    gpu_count: int = 0
    gpu_memory_mb: Optional[int] = None
    cpu_count: int = field(default_factory=lambda: __import__('os').cpu_count() or 1)
    
    # Library availability
    has_cupy: bool = False
    has_numba: bool = False
    has_scipy: bool = False
    has_jax: bool = False
    
    # Memory
    total_memory_mb: Optional[int] = None
    available_memory_mb: Optional[int] = None
    
    def __post_init__(self):
        """Detect capabilities."""
        self._detect_libraries()
        self._detect_gpu()
        self._detect_memory()
    
    def _detect_libraries(self):
        """Detect available libraries."""
        try:
            import cupy
            self.has_cupy = True
        except ImportError:
            pass
        
        try:
            import numba
            self.has_numba = True
        except ImportError:
            pass
        
        try:
            import scipy
            self.has_scipy = True
        except ImportError:
            pass
        
        try:
            import jax
            self.has_jax = True
        except ImportError:
            pass
    
    def _detect_gpu(self):
        """Detect GPU availability."""
        if self.has_cupy:
            try:
                import cupy as cp
                self.has_gpu = True
                self.gpu_count = cp.cuda.runtime.getDeviceCount()
                if self.gpu_count > 0:
                    props = cp.cuda.runtime.getDeviceProperties(0)
                    self.gpu_memory_mb = props['totalGlobalMem'] // (1024 ** 2)
            except Exception:
                pass
    
    def _detect_memory(self):
        """Detect system memory."""
        try:
            import psutil
            mem = psutil.virtual_memory()
            self.total_memory_mb = mem.total // (1024 ** 2)
            self.available_memory_mb = mem.available // (1024 ** 2)
        except ImportError:
            pass
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary."""
        return {
            'os': self.os,
            'os_version': self.os_version,
            'python_version': self.python_version,
            'architecture': self.architecture,
            'processor': self.processor,
            'has_gpu': self.has_gpu,
            'gpu_count': self.gpu_count,
            'gpu_memory_mb': self.gpu_memory_mb,
            'cpu_count': self.cpu_count,
            'has_cupy': self.has_cupy,
            'has_numba': self.has_numba,
            'has_scipy': self.has_scipy,
            'has_jax': self.has_jax,
            'total_memory_mb': self.total_memory_mb,
            'available_memory_mb': self.available_memory_mb,
        }


class InteroperabilityLayer:
    """
    Provides unified interface across platforms and libraries.
    
    Automatically adapts to available hardware and software capabilities.
    """
    
    def __init__(self):
        self.capabilities = PlatformCapabilities()
        self.preferred_backend = self._select_backend()
        self.fallback_backends = self._get_fallback_chain()
    
    def _select_backend(self) -> str:
        """Select optimal backend based on capabilities."""
        if self.capabilities.has_cupy and self.capabilities.has_gpu:
            return 'cupy'
        elif self.capabilities.has_jax:
            return 'jax'
        elif self.capabilities.has_scipy:
            return 'scipy+numpy'
        else:
            return 'numpy'
    
    def _get_fallback_chain(self) -> List[str]:
        """Get fallback chain for backend failures."""
        chain = []
        
        backends = ['cupy', 'jax', 'scipy+numpy', 'numpy']
        for backend in backends:
            if backend != self.preferred_backend:
                chain.append(backend)
        
        return chain
    
    def get_array_module(self):
        """
        Get appropriate array module (numpy or cupy).
        
        Returns:
            Module for array operations
        """
        if self.preferred_backend == 'cupy':
            import cupy as cp
            return cp
        elif self.preferred_backend == 'jax':
            import jax.numpy as jnp
            return jnp
        else:
            import numpy as np
            return np
    
    def to_cpu(self, array) -> 'numpy.ndarray':
        """Convert array to CPU (numpy)."""
        import numpy as np
        
        if hasattr(array, 'get'):  # CuPy array
            return array.get()
        elif hasattr(array, '__array__'):  # JAX or other
            return np.asarray(array)
        else:
            return array
    
    def to_gpu(self, array):
        """Convert array to GPU if available."""
        if not self.capabilities.has_gpu:
            return array
        
        if self.preferred_backend == 'cupy':
            import cupy as cp
            if not isinstance(array, cp.ndarray):
                return cp.asarray(array)
        
        return array
    
    def execute_with_fallback(self, func: Callable, *args, **kwargs) -> Any:
        """
        Execute function with automatic fallback on failure.
        
        Tries backends in order: preferred -> fallback1 -> fallback2 -> ...
        """
        backends_to_try = [self.preferred_backend] + self.fallback_backends
        
        last_error = None
        for backend in backends_to_try:
            try:
                # Set current backend
                old_backend = self.preferred_backend
                self.preferred_backend = backend
                
                result = func(*args, **kwargs)
                
                # Restore backend
                self.preferred_backend = old_backend
                return result
                
            except Exception as e:
                last_error = e
                warnings.warn(f"Backend {backend} failed: {e}. Trying fallback...",
                            RuntimeWarning)
                continue
        
        # All backends failed
        raise RuntimeError(f"All backends failed. Last error: {last_error}")


# ═══════════════════════════════════════════════════════════════════════════
# PART III: APPLICABILITY AND VIABILITY
# ═══════════════════════════════════════════════════════════════════════════

@dataclass
class ApplicabilityScore:
    """Score indicating how applicable an algorithm is to given inputs."""
    score: float  # 0.0 to 1.0
    confidence: float  # 0.0 to 1.0
    reasons: List[str] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    recommendations: List[str] = field(default_factory=list)
    
    def is_applicable(self, threshold: float = 0.5) -> bool:
        """Check if algorithm is applicable."""
        return self.score >= threshold
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary."""
        return {
            'score': self.score,
            'confidence': self.confidence,
            'reasons': self.reasons,
            'warnings': self.warnings,
            'recommendations': self.recommendations,
        }


class ApplicabilityChecker:
    """
    Checks if algorithms are applicable to given inputs.
    
    Considers:
    - Input dimensions and size
    - Memory requirements vs available memory
    - Numerical properties (condition number, sparsity)
    - Platform capabilities
    """
    
    def __init__(self, capabilities: Optional[PlatformCapabilities] = None):
        self.capabilities = capabilities or PlatformCapabilities()
    
    def check_tensor_approximation(self, tensor_shape: Tuple[int, ...],
                                   ranks: List[int]) -> ApplicabilityScore:
        """
        Check if TT approximation is applicable.
        
        Args:
            tensor_shape: Shape of tensor to approximate
            ranks: TT ranks
        
        Returns:
            Applicability score
        """
        score = 1.0
        confidence = 1.0
        reasons = []
        warnings_list = []
        recommendations = []
        
        # Check dimension
        ndim = len(tensor_shape)
        if ndim < 3:
            score *= 0.5
            warnings_list.append(f"Low dimensionality (d={ndim}). TT most effective for d≥3")
            recommendations.append("Consider standard matrix factorization for 2D")
        else:
            reasons.append(f"Good dimensionality (d={ndim})")
        
        # Check tensor size
        full_size = 1
        for dim in tensor_shape:
            full_size *= dim
        
        if full_size > 1e9:  # >1B elements
            reasons.append(f"Large tensor ({full_size:.2e} elements) - TT decomposition beneficial")
            score *= 1.0
        elif full_size < 1e6:  # <1M elements
            score *= 0.7
            warnings_list.append(f"Small tensor ({full_size:.2e} elements). Direct computation may be faster")
        
        # Check memory requirements
        tt_memory = self._estimate_tt_memory(tensor_shape, ranks)
        if self.capabilities.available_memory_mb:
            available_mb = self.capabilities.available_memory_mb
            required_mb = tt_memory / (1024 ** 2)
            
            if required_mb > available_mb * 0.8:  # Using >80% memory
                score *= 0.3
                confidence *= 0.7
                warnings_list.append(f"High memory usage ({required_mb:.0f}MB / {available_mb:.0f}MB available)")
                recommendations.append("Consider reducing ranks or using disk-based computation")
            else:
                reasons.append(f"Sufficient memory ({required_mb:.0f}MB / {available_mb:.0f}MB)")
        
        # Check ranks
        max_rank = max(ranks[1:-1]) if len(ranks) > 2 else 1
        avg_dim = sum(tensor_shape) / len(tensor_shape)
        
        if max_rank > avg_dim * 0.5:
            score *= 0.8
            warnings_list.append(f"High ranks (max={max_rank}) relative to dimensions (avg={avg_dim:.1f})")
            recommendations.append("Try lower ranks for better compression")
        
        return ApplicabilityScore(
            score=score,
            confidence=confidence,
            reasons=reasons,
            warnings=warnings_list,
            recommendations=recommendations
        )
    
    def _estimate_tt_memory(self, shape: Tuple[int, ...], ranks: List[int]) -> int:
        """
        Estimate memory requirement for TT representation.
        
        Returns:
            Memory in bytes
        """
        memory = 0
        for i in range(len(shape)):
            r_left = ranks[i]
            n = shape[i]
            r_right = ranks[i + 1]
            memory += r_left * n * r_right * 8  # 8 bytes per float64
        return memory


# ═══════════════════════════════════════════════════════════════════════════
# PART IV: MITIGATION STRATEGIES
# ═══════════════════════════════════════════════════════════════════════════

class MitigationStrategy:
    """
    Mitigation strategies for common failure modes.
    
    Handles:
    - Numerical instability
    - Memory overflow
    - Convergence failures
    - Hardware failures
    - Performance degradation
    """
    
    @staticmethod
    def mitigate_numerical_instability(condition_number: float, 
                                      matrix_size: Tuple[int, int]) -> Dict[str, Any]:
        """
        Suggest mitigations for numerical instability.
        
        Args:
            condition_number: Matrix condition number
            matrix_size: Shape of matrix
        
        Returns:
            Dictionary of mitigation suggestions
        """
        mitigations = {
            'use_iterative_refinement': False,
            'use_preconditioning': False,
            'use_higher_precision': False,
            'use_regularization': False,
            'recommended_actions': []
        }
        
        if condition_number > 1e12:
            mitigations['use_higher_precision'] = True
            mitigations['use_regularization'] = True
            mitigations['recommended_actions'].append(
                "Matrix is ill-conditioned. Use float128 or regularization."
            )
        elif condition_number > 1e8:
            mitigations['use_iterative_refinement'] = True
            mitigations['use_preconditioning'] = True
            mitigations['recommended_actions'].append(
                "Matrix is poorly conditioned. Use iterative refinement."
            )
        
        if matrix_size[0] * matrix_size[1] > 1e8:  # Large matrix
            mitigations['use_preconditioning'] = True
            mitigations['recommended_actions'].append(
                "Large matrix. Consider preconditioning for stability."
            )
        
        return mitigations
    
    @staticmethod
    def mitigate_memory_overflow(required_bytes: int, 
                                available_bytes: int) -> Dict[str, Any]:
        """
        Suggest mitigations for memory overflow.
        
        Args:
            required_bytes: Memory required
            available_bytes: Memory available
        
        Returns:
            Dictionary of mitigation suggestions
        """
        mitigations = {
            'use_disk_storage': False,
            'use_lower_precision': False,
            'use_compression': False,
            'split_computation': False,
            'recommended_actions': []
        }
        
        ratio = required_bytes / available_bytes
        
        if ratio > 2.0:
            mitigations['use_disk_storage'] = True
            mitigations['split_computation'] = True
            mitigations['recommended_actions'].append(
                f"Required memory ({required_bytes/(1024**3):.2f}GB) >> available "
                f"({available_bytes/(1024**3):.2f}GB). Use disk-based computation."
            )
        elif ratio > 1.2:
            mitigations['use_compression'] = True
            mitigations['use_lower_precision'] = True
            mitigations['recommended_actions'].append(
                f"Tight memory ({ratio:.1f}x required). Use compression or float32."
            )
        
        return mitigations
    
    @staticmethod
    def mitigate_convergence_failure(iteration: int, max_iter: int,
                                    error: float, prev_error: float) -> Dict[str, Any]:
        """
        Suggest mitigations for convergence failures.
        
        Args:
            iteration: Current iteration
            max_iter: Maximum iterations
            error: Current error
            prev_error: Previous error
        
        Returns:
            Dictionary of mitigation suggestions
        """
        mitigations = {
            'increase_max_iter': False,
            'adjust_learning_rate': False,
            'use_better_initialization': False,
            'change_algorithm': False,
            'recommended_actions': []
        }
        
        # Check if making progress
        if error >= prev_error * 0.99:  # <1% improvement
            mitigations['adjust_learning_rate'] = True
            mitigations['recommended_actions'].append(
                "Slow convergence. Try adjusting learning rate or better initialization."
            )
        
        # Check if reached max iterations
        if iteration >= max_iter * 0.9:
            mitigations['increase_max_iter'] = True
            mitigations['recommended_actions'].append(
                f"Approaching max iterations ({iteration}/{max_iter}). "
                "Consider increasing max_iter."
            )
        
        # Check if error is very high
        if error > 1.0:
            mitigations['use_better_initialization'] = True
            mitigations['change_algorithm'] = True
            mitigations['recommended_actions'].append(
                "High error. Try better initialization or different algorithm."
            )
        
        return mitigations


# Export public interface
__all__ = [
    'Version',
    'VersionCompatibility',
    'VersionRegistry',
    'PlatformCapabilities',
    'InteroperabilityLayer',
    'ApplicabilityScore',
    'ApplicabilityChecker',
    'MitigationStrategy',
]
