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
RAFAELIA ENGINE FULLSTACK - Integrated TT Processing Engine

This module provides the main orchestration engine for RAFAELIA Tensor Train
processing, integrating cross-approximation, local updates, and adaptive algorithms.

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
import time
from typing import List, Tuple, Optional, Dict, Any, Callable
from pathlib import Path
import os

# Optional dependencies
try:
    import cupy as cp
    HAS_CUPY = True
except ImportError:
    HAS_CUPY = False

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

try:
    from flask import Flask, jsonify, request
    HAS_FLASK = True
except ImportError:
    HAS_FLASK = False


# Import RAFAELIA modules (relative imports work when in package)
try:
    from .RAFAELIA_TT_CROSS_FULL import TTCrossApproximation
    from .RAFAELIA_TT_UPDATE_FULL import TTLocalUpdate
except ImportError:
    # Fallback for standalone execution
    import sys
    sys.path.insert(0, os.path.dirname(__file__))
    from RAFAELIA_TT_CROSS_FULL import TTCrossApproximation
    from RAFAELIA_TT_UPDATE_FULL import TTLocalUpdate


class RAFAELIAEngine:
    """
    Fullstack TT Engine integrating cross-approximation and updates.
    
    Provides high-level interface for tensor operations with automatic
    rank adaptation, checkpointing, and RAFAELIA manifest generation.
    """
    
    def __init__(self, config: Optional[Dict[str, Any]] = None):
        """
        Initialize RAFAELIA Engine.
        
        Args:
            config: Configuration dictionary with options:
                - use_gpu: Enable GPU acceleration (bool)
                - checkpoint_dir: Directory for checkpoints (str)
                - auto_checkpoint: Auto-save after operations (bool)
                - compression: Use zstd compression (bool)
        """
        self.config = config or {}
        self.use_gpu = self.config.get('use_gpu', False) and HAS_CUPY
        self.checkpoint_dir = Path(self.config.get('checkpoint_dir', '/tmp'))
        self.auto_checkpoint = self.config.get('auto_checkpoint', True)
        self.compression = self.config.get('compression', True) and HAS_ZSTD
        
        # Engine state
        self.tt_cross = None
        self.tt_update = None
        self.metadata = {
            'created': time.time(),
            'operations': [],
            'rafaelia': {
                'signature': 'RAFCODE-Φ-∆RafaelVerboΩ',
                'module': 'ENGINE_FULLSTACK',
                'philosophy': 'VAZIO → VERBO → CHEIO → RETRO'
            }
        }
        
        # Ensure checkpoint directory exists
        self.checkpoint_dir.mkdir(parents=True, exist_ok=True)
    
    def approximate_tensor(self, func: Callable, shape: List[int],
                          ranks: List[int], **kwargs) -> Dict[str, Any]:
        """
        Approximate high-dimensional tensor using TT-cross.
        
        Args:
            func: Function to approximate (takes list of indices)
            shape: Tensor dimensions
            ranks: TT ranks
            **kwargs: Additional arguments for cross approximation
            
        Returns:
            Dictionary with approximation results
        """
        print(f"Starting TT-cross approximation...")
        print(f"  Shape: {shape}")
        print(f"  Ranks: {ranks}")
        
        start_time = time.time()
        
        # Create cross approximation
        self.tt_cross = TTCrossApproximation(
            shape=shape,
            ranks=ranks,
            use_gpu=self.use_gpu,
            epsilon=kwargs.get('epsilon', 1e-6)
        )
        
        # Perform approximation
        stats = self.tt_cross.cross_approximation(
            func=func,
            max_iter=kwargs.get('max_iter', 100),
            verbose=kwargs.get('verbose', False)
        )
        
        elapsed = time.time() - start_time
        stats['elapsed_time'] = elapsed
        
        # Record operation
        self.metadata['operations'].append({
            'type': 'cross_approximation',
            'timestamp': time.time(),
            'shape': shape,
            'ranks': ranks,
            'stats': stats
        })
        
        # Auto-checkpoint
        if self.auto_checkpoint:
            self._save_checkpoint('tt_cross_auto')
        
        print(f"Approximation complete in {elapsed:.2f}s")
        print(f"  Converged: {stats['converged']}")
        print(f"  Final error: {stats['error']:.2e}")
        
        return stats
    
    def update_tensor(self, target_data: Dict[Tuple, float],
                     **kwargs) -> Dict[str, Any]:
        """
        Update TT decomposition using local updates.
        
        Args:
            target_data: Dictionary mapping indices to target values
            **kwargs: Additional arguments for update
            
        Returns:
            Dictionary with update results
        """
        if self.tt_cross is None:
            raise RuntimeError("Must run approximate_tensor first")
        
        print(f"Starting TT local update...")
        print(f"  Target samples: {len(target_data)}")
        
        start_time = time.time()
        
        # Create updater from cross approximation cores
        self.tt_update = TTLocalUpdate(
            cores=self.tt_cross.cores,
            use_gpu=self.use_gpu
        )
        
        # Perform ALS updates
        stats = self.tt_update.als_sweep(
            target_data=target_data,
            n_iterations=kwargs.get('n_iterations', 10),
            verbose=kwargs.get('verbose', False)
        )
        
        elapsed = time.time() - start_time
        stats['elapsed_time'] = elapsed
        
        # Update cross approximation cores
        self.tt_cross.cores = self.tt_update.cores
        
        # Record operation
        self.metadata['operations'].append({
            'type': 'local_update',
            'timestamp': time.time(),
            'n_samples': len(target_data),
            'stats': stats
        })
        
        # Auto-checkpoint
        if self.auto_checkpoint:
            self._save_checkpoint('tt_update_auto')
        
        print(f"Update complete in {elapsed:.2f}s")
        print(f"  Final error: {stats['final_error']:.2e}")
        
        return stats
    
    def adapt_ranks(self, core_idx: int, new_rank: int,
                   method: str = 'truncate') -> Dict[str, Any]:
        """
        Adapt TT ranks at specified position.
        
        Args:
            core_idx: Core index where rank changes
            new_rank: New rank value
            method: 'truncate' or 'expand'
            
        Returns:
            Dictionary with adaptation results
        """
        if self.tt_update is None:
            if self.tt_cross is not None:
                self.tt_update = TTLocalUpdate(
                    cores=self.tt_cross.cores,
                    use_gpu=self.use_gpu
                )
            else:
                raise RuntimeError("No TT decomposition available")
        
        print(f"Adapting rank at position {core_idx} to {new_rank}...")
        
        old_rank = self.tt_update.ranks[core_idx + 1]
        start_time = time.time()
        
        self.tt_update.rank_adaptation(core_idx, new_rank, method)
        
        elapsed = time.time() - start_time
        
        # Update cross approximation if it exists
        if self.tt_cross is not None:
            self.tt_cross.cores = self.tt_update.cores
            self.tt_cross.ranks = self.tt_update.ranks
        
        result = {
            'old_rank': old_rank,
            'new_rank': new_rank,
            'method': method,
            'elapsed_time': elapsed
        }
        
        # Record operation
        self.metadata['operations'].append({
            'type': 'rank_adaptation',
            'timestamp': time.time(),
            'core_idx': core_idx,
            'result': result
        })
        
        print(f"Rank adaptation complete in {elapsed:.4f}s")
        
        return result
    
    def evaluate(self, indices: List[int]) -> float:
        """Evaluate TT at given indices."""
        if self.tt_cross is not None:
            return self.tt_cross.evaluate(indices)
        elif self.tt_update is not None:
            return self.tt_update._evaluate(tuple(indices))
        else:
            raise RuntimeError("No TT decomposition available")
    
    def _save_checkpoint(self, name: str):
        """Save checkpoint with RAFAELIA manifest."""
        timestamp = int(time.time())
        filepath = self.checkpoint_dir / f"{name}_{timestamp}.json"
        
        if self.tt_cross is not None:
            self.tt_cross.save_checkpoint(str(filepath), metadata=self.metadata)
        elif self.tt_update is not None:
            self.tt_update.save_checkpoint(str(filepath), metadata=self.metadata)
    
    def generate_manifest(self, output_path: Optional[str] = None) -> Dict[str, Any]:
        """
        Generate RAFAELIA manifest for current state.
        
        Args:
            output_path: Optional path to save manifest JSON
            
        Returns:
            Manifest dictionary
        """
        manifest = {
            'signature': 'RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ',
            'timestamp': time.time(),
            'module': 'ENGINE_FULLSTACK',
            'philosophy': 'VAZIO → VERBO → CHEIO → RETRO',
            'metadata': self.metadata,
            'config': {
                'use_gpu': self.use_gpu,
                'has_cupy': HAS_CUPY,
                'has_blake3': HAS_BLAKE3,
                'has_zstd': HAS_ZSTD,
                'has_flask': HAS_FLASK
            }
        }
        
        # Add TT state if available
        if self.tt_cross is not None:
            manifest['tt_state'] = {
                'shape': self.tt_cross.shape,
                'ranks': self.tt_cross.ranks,
                'epsilon': self.tt_cross.epsilon
            }
        
        # Compute manifest hash
        manifest_str = json.dumps(manifest['metadata'], sort_keys=True)
        manifest['hashes'] = {
            'sha256': hashlib.sha256(manifest_str.encode()).hexdigest()
        }
        
        if HAS_BLAKE3:
            manifest['hashes']['blake3'] = blake3.blake3(
                manifest_str.encode()
            ).hexdigest()
        
        if output_path:
            with open(output_path, 'w') as f:
                json.dump(manifest, f, indent=2)
            print(f"Manifest saved: {output_path}")
        
        return manifest


def demo_engine():
    """Demonstration of RAFAELIA Engine."""
    print("=" * 60)
    print("RAFAELIA ENGINE FULLSTACK - Demonstration")
    print("=" * 60)
    print()
    
    # Configuration
    config = {
        'use_gpu': False,
        'checkpoint_dir': '/tmp/rafaelia_checkpoints',
        'auto_checkpoint': True,
        'compression': HAS_ZSTD
    }
    
    print("Engine Configuration:")
    for key, value in config.items():
        print(f"  {key}: {value}")
    print()
    
    # Initialize engine
    engine = RAFAELIAEngine(config)
    
    # Define test function
    def test_function(indices):
        return sum(indices) * 0.5 + np.prod(indices) * 0.1
    
    # Approximate tensor
    shape = [4, 5, 6]
    ranks = [1, 2, 3, 1]
    
    approx_stats = engine.approximate_tensor(
        func=test_function,
        shape=shape,
        ranks=ranks,
        max_iter=5,
        verbose=True
    )
    print()
    
    # Create target data for update
    target_data = {}
    for _ in range(15):
        indices = tuple(np.random.randint(0, s) for s in shape)
        target_data[indices] = test_function(list(indices)) + np.random.randn() * 0.1
    
    # Update tensor
    update_stats = engine.update_tensor(
        target_data=target_data,
        n_iterations=3,
        verbose=True
    )
    print()
    
    # Test evaluation
    test_indices = [1, 2, 3]
    value = engine.evaluate(test_indices)
    true_value = test_function(test_indices)
    print(f"Evaluation at {test_indices}:")
    print(f"  Predicted: {value:.6f}")
    print(f"  True: {true_value:.6f}")
    print(f"  Error: {abs(value - true_value):.2e}")
    print()
    
    # Generate manifest
    manifest_path = "/tmp/rafaelia_manifest.json"
    manifest = engine.generate_manifest(manifest_path)
    print(f"\nManifest generated with {len(manifest['metadata']['operations'])} operations")
    print()
    
    print("=" * 60)
    print("RAFAELIA Philosophy: VAZIO → VERBO → CHEIO → RETRO")
    print("=" * 60)


if __name__ == '__main__':
    demo_engine()
