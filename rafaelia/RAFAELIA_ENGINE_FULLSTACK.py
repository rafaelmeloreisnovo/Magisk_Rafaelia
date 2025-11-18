#!/usr/bin/env python3
"""
RAFAELIA_ENGINE_FULLSTACK.py
=============================
Orchestrator engine that ties TTBuilder and TT-update modules into a runnable engine.

Provides CLI modes:
- demo: Run demonstration with test data
- build: Build TT-decomposition from generator
- patch: Apply local patch updates
- serve: Flask REST API stub (optional)

Handles checkpoints, HashVivo metadata, and complete TT workflows.

Design rationale (ψχρΔΣΩ):
- Unified interface for all TT operations
- Checkpoint management and recovery
- Extensible architecture

Φ_ethica: Responsible orchestration
- Resource monitoring
- Safe defaults
- Audit logging
- Token/credential protection
"""

import sys
import os
import argparse
import json
from typing import Optional, Dict, Any
import warnings
import logging
# Import RAFAELIA modules
try:
    from RAFAELIA_TT_CROSS_FULL import TTBuilder
    from RAFAELIA_TT_UPDATE_FULL import (
        local_patch_update_tt, tt_round, compute_patch_error, tt_energy
    )
    from RAFAELIA_SPIRAL_FIBONACCI import value_generator
    from RAFAELIA_TT_ACCEL import tt_svd_from_full, estimate_compression_ratio
except ImportError:
    # Try relative import
    try:
        from .RAFAELIA_TT_CROSS_FULL import TTBuilder
        from .RAFAELIA_TT_UPDATE_FULL import (
            local_patch_update_tt, tt_round, compute_patch_error, tt_energy
        )
        from .RAFAELIA_SPIRAL_FIBONACCI import value_generator
        from .RAFAELIA_TT_ACCEL import tt_svd_from_full, estimate_compression_ratio
    except ImportError:
        warnings.warn("Could not import RAFAELIA modules. Some features may not work.")

import numpy as np

# Optional Flask for REST API
try:
    from flask import Flask, request, jsonify
    HAS_FLASK = True
except ImportError:
    HAS_FLASK = False


class RAFAELIAEngine:
    """
    Main orchestrator for TT-decomposition workflows.
    """
    
    def __init__(
        self,
        config: Optional[Dict[str, Any]] = None,
        verbose: bool = True
    ):
        """
        Initialize engine.
        
        Args:
            config: Configuration dictionary
            verbose: Print progress information
        """
        self.config = config or {}
        self.verbose = verbose
        
        # Default configuration
        self.default_config = {
            "max_rank": 20,
            "tol": 1e-6,
            "max_samples": 10000,
            "use_qtt": False,
            "max_memory_gb": 4.0,
            "checkpoint_dir": "/tmp/rafaelia_checkpoints",
            "generator_type": "fibonacci_spiral",
            "shape": (8, 8, 8)
        }
        
        # Merge with provided config
        for key, value in self.default_config.items():
            if key not in self.config:
                self.config[key] = value
        
        # State
        self.builder: Optional[TTBuilder] = None
        self.audit_log = []
        
        # Ensure checkpoint directory exists
        os.makedirs(self.config["checkpoint_dir"], exist_ok=True)
    
    def log_action(self, action: str, details: Dict[str, Any]):
        """Log action for audit trail."""
        entry = {
            "action": action,
            "details": details
        }
        self.audit_log.append(entry)
        
        if self.verbose:
            print(f"[LOG] {action}: {details}")
    
    def build_tt(
        self,
        shape: Optional[tuple] = None,
        generator_type: Optional[str] = None,
        checkpoint_name: str = "tt_build"
    ) -> TTBuilder:
        """
        Build TT-decomposition using cross-approximation.
        
        Args:
            shape: Tensor shape (uses config if None)
            generator_type: Value generator type (uses config if None)
            checkpoint_name: Name for checkpoint file
            
        Returns:
            TTBuilder instance
        """
        shape = shape or self.config["shape"]
        generator_type = generator_type or self.config["generator_type"]
        
        print(f"\n=== Building TT for shape {shape} ===")
        print(f"Generator: {generator_type}")
        
        # Create value function
        value_func = value_generator(generator_type, shape=shape)
        
        # Build TT
        self.builder = TTBuilder(
            shape=shape,
            value_function=value_func,
            max_rank=self.config["max_rank"],
            tol=self.config["tol"],
            max_samples=self.config["max_samples"],
            use_qtt=self.config["use_qtt"],
            max_memory_gb=self.config["max_memory_gb"],
            verbose=self.verbose
        )
        
        cores = self.builder.build_tt_cross()
        
        # Log action
        self.log_action("build_tt", {
            "shape": shape,
            "ranks": self.builder.ranks,
            "samples": self.builder.samples_used,
            "hash": self.builder.hash_metadata.get("hash", "N/A")
        })
        
        # Save checkpoint
        checkpoint_path = os.path.join(
            self.config["checkpoint_dir"],
            f"{checkpoint_name}.bitraf64"
        )
        self.builder.save_checkpoint(checkpoint_path)
        print(f"\nCheckpoint saved: {checkpoint_path}")
        
        return self.builder
    
    def load_checkpoint(self, checkpoint_name: str) -> TTBuilder:
        """
        Load TT from checkpoint.
        
        Args:
            checkpoint_name: Checkpoint filename (without path)
            
        Returns:
            Loaded TTBuilder
        """
        checkpoint_path = os.path.join(
            self.config["checkpoint_dir"],
            checkpoint_name if checkpoint_name.endswith(".bitraf64") else f"{checkpoint_name}.bitraf64"
        )
        
        print(f"\n=== Loading checkpoint: {checkpoint_path} ===")
        self.builder = TTBuilder.load_checkpoint(checkpoint_path)
        
        self.log_action("load_checkpoint", {
            "path": checkpoint_path,
            "ranks": self.builder.ranks,
            "hash": self.builder.hash_metadata.get("hash", "N/A")
        })
        
        return self.builder
    
    def apply_patch(
        self,
        patch_data: np.ndarray,
        patch_indices: tuple,
        method: str = "als",
        checkpoint_name: str = "tt_patched"
    ):
        """
        Apply local patch update to TT.
        
        Args:
            patch_data: New data for patch region
            patch_indices: Patch location (tuple of slices)
            method: Update method
            checkpoint_name: Name for updated checkpoint
        """
        if self.builder is None:
            raise RuntimeError("No TT loaded. Build or load checkpoint first.")
        
        print(f"\n=== Applying patch update ===")
        print(f"Patch indices: {patch_indices}")
        print(f"Method: {method}")
        
        # Compute initial error
        initial_error = compute_patch_error(
            self.builder.cores, patch_data, patch_indices
        )
        print(f"Initial patch error: {initial_error:.6e}")
        
        # Update
        updated_cores = local_patch_update_tt(
            self.builder.cores,
            patch_data,
            patch_indices,
            method=method,
            max_rank=self.config["max_rank"]
        )
        
        # Compute final error
        final_error = compute_patch_error(
            updated_cores, patch_data, patch_indices
        )
        print(f"Final patch error: {final_error:.6e}")
        
        # Update builder
        self.builder.cores = updated_cores
        self.builder._compute_hash()
        
        # Log action
        self.log_action("apply_patch", {
            "method": method,
            "initial_error": float(initial_error),
            "final_error": float(final_error),
            "new_hash": self.builder.hash_metadata.get("hash", "N/A")
        })
        
        # Save checkpoint
        checkpoint_path = os.path.join(
            self.config["checkpoint_dir"],
            f"{checkpoint_name}.bitraf64"
        )
        self.builder.save_checkpoint(checkpoint_path)
        print(f"\nUpdated checkpoint saved: {checkpoint_path}")
    
    def round_tt(self, tol: Optional[float] = None, max_rank: Optional[int] = None):
        """
        Apply TT-rounding to current TT.
        
        Args:
            tol: Tolerance (uses config if None)
            max_rank: Maximum rank (uses config if None)
        """
        if self.builder is None:
            raise RuntimeError("No TT loaded. Build or load checkpoint first.")
        
        tol = tol or self.config["tol"]
        max_rank = max_rank or self.config["max_rank"]
        
        print(f"\n=== Rounding TT ===")
        print(f"Tolerance: {tol}, Max rank: {max_rank}")
        
        old_ranks = [c.shape[0] for c in self.builder.cores] + [1]
        print(f"Old ranks: {old_ranks}")
        
        # Round
        self.builder.cores = tt_round(self.builder.cores, tol=tol, max_rank=max_rank)
        self.builder.ranks = [c.shape[0] for c in self.builder.cores] + [1]
        
        print(f"New ranks: {self.builder.ranks}")
        
        # Update hash
        self.builder._compute_hash()
        
        self.log_action("round_tt", {
            "old_ranks": old_ranks,
            "new_ranks": self.builder.ranks,
            "tol": tol,
            "max_rank": max_rank
        })
    
    def get_status(self) -> Dict[str, Any]:
        """
        Get engine status.
        
        Returns:
            Status dictionary
        """
        if self.builder is None:
            return {"loaded": False, "message": "No TT loaded"}
        
        energy = tt_energy(self.builder.cores)
        
        return {
            "loaded": True,
            "shape": self.builder.shape,
            "ranks": self.builder.ranks,
            "samples_used": self.builder.samples_used,
            "hash": self.builder.hash_metadata.get("hash", "N/A"),
            "energy": float(energy),
            "audit_log_entries": len(self.audit_log)
        }
    
    def demo_workflow(self):
        """Run complete demo workflow."""
        print("\n" + "=" * 60)
        print("RAFAELIA ENGINE FULLSTACK DEMO")
        print("=" * 60)
        
        # Step 1: Build TT
        print("\n[Step 1] Building TT from Fibonacci spiral generator...")
        self.build_tt(
            shape=(6, 6, 6),
            generator_type="fibonacci_spiral",
            checkpoint_name="demo_build"
        )
        
        # Step 2: Create patch
        print("\n[Step 2] Creating test patch...")
        patch_indices = (slice(2, 4), slice(2, 4), slice(2, 4))
        patch_data = np.random.rand(2, 2, 2) * 0.5
        
        # Step 3: Apply patch
        print("\n[Step 3] Applying patch update...")
        self.apply_patch(
            patch_data,
            patch_indices,
            method="als",
            checkpoint_name="demo_patched"
        )
        
        # Step 4: Round
        print("\n[Step 4] Rounding TT...")
        self.round_tt(tol=1e-3)
        
        # Step 5: Status
        print("\n[Step 5] Final status:")
        status = self.get_status()
        print(json.dumps(status, indent=2))
        
        print("\n" + "=" * 60)
        print("DEMO COMPLETE")
        print("=" * 60)


def create_flask_app(engine: RAFAELIAEngine) -> 'Flask':
    """
    Create Flask REST API for RAFAELIA engine.
    
    Args:
        engine: RAFAELIAEngine instance
        
    Returns:
        Flask app
    """
    if not HAS_FLASK:
        raise RuntimeError("Flask not available. Install with: pip install flask")
    
    app = Flask(__name__)
    
    @app.route('/status', methods=['GET'])
    def status():
        """Get engine status."""
        return jsonify(engine.get_status())
    
    @app.route('/build', methods=['POST'])
    def build():
        """Build TT from parameters."""
        data = request.json
        shape = tuple(data.get('shape', [8, 8, 8]))
        generator = data.get('generator', 'fibonacci_spiral')
        
        try:
            engine.build_tt(shape=shape, generator_type=generator)
            return jsonify({"success": True, "status": engine.get_status()})
        except Exception as e:
            return jsonify({"success": False, "error": str(e)}), 500
    
    @app.route('/load', methods=['POST'])
    def load():
        """Load checkpoint."""
        data = request.json
        checkpoint_name = data.get('checkpoint')
        
        try:
            engine.load_checkpoint(checkpoint_name)
            return jsonify({"success": True, "status": engine.get_status()})
        except Exception as e:
            logging.exception("Exception occurred in /load endpoint.")
            return jsonify({"success": False, "error": "An internal error occurred."}), 500
    return app


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="RAFAELIA Engine - TT-decomposition orchestrator"
    )
    
    parser.add_argument(
        'mode',
        choices=['demo', 'build', 'patch', 'serve'],
        help='Operation mode'
    )
    
    parser.add_argument(
        '--shape',
        type=str,
        default='8,8,8',
        help='Tensor shape (comma-separated)'
    )
    
    parser.add_argument(
        '--generator',
        type=str,
        default='fibonacci_spiral',
        choices=['fibonacci_spiral', 'gaussian_blobs', 'polynomial', 'sinusoidal'],
        help='Value generator type'
    )
    
    parser.add_argument(
        '--max-rank',
        type=int,
        default=20,
        help='Maximum TT-rank'
    )
    
    parser.add_argument(
        '--tol',
        type=float,
        default=1e-6,
        help='Tolerance'
    )
    
    parser.add_argument(
        '--checkpoint',
        type=str,
        default='tt_checkpoint',
        help='Checkpoint name'
    )
    
    parser.add_argument(
        '--port',
        type=int,
        default=5000,
        help='Port for serve mode'
    )
    
    parser.add_argument(
        '--quiet',
        action='store_true',
        help='Suppress verbose output'
    )
    
    args = parser.parse_args()
    
    # Parse shape
    shape = tuple(int(x) for x in args.shape.split(','))
    
    # Create config
    config = {
        "shape": shape,
        "generator_type": args.generator,
        "max_rank": args.max_rank,
        "tol": args.tol
    }
    
    # Create engine
    engine = RAFAELIAEngine(config=config, verbose=not args.quiet)
    
    # Execute mode
    if args.mode == 'demo':
        engine.demo_workflow()
    
    elif args.mode == 'build':
        engine.build_tt(checkpoint_name=args.checkpoint)
        print("\n" + json.dumps(engine.get_status(), indent=2))
    
    elif args.mode == 'patch':
        # Load checkpoint
        engine.load_checkpoint(args.checkpoint)
        
        # Create demo patch
        patch_indices = (slice(1, 3), slice(1, 3), slice(1, 3))
        patch_data = np.random.rand(2, 2, 2)
        
        engine.apply_patch(
            patch_data,
            patch_indices,
            checkpoint_name=f"{args.checkpoint}_patched"
        )
    
    elif args.mode == 'serve':
        if not HAS_FLASK:
            print("ERROR: Flask not installed. Install with: pip install flask")
            sys.exit(1)
        
        app = create_flask_app(engine)
        print(f"\nStarting REST API server on port {args.port}...")
        print(f"Endpoints: /status, /build, /load")
        app.run(port=args.port, debug=False)


if __name__ == "__main__":
    main()
