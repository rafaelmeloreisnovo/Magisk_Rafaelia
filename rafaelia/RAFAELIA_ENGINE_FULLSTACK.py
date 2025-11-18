#!/usr/bin/env python3
"""
RAFAELIA ENGINE FULLSTACK - Integrated TT Processing Engine

====================================================================
RESUMO TÉCNICO / TECHNICAL SUMMARY
====================================================================

PORTUGUÊS:
----------
Este módulo implementa o motor principal de processamento Tensor Train (TT)
do framework RAFAELIA. Ele orquestra três componentes fundamentais:

1. **Aproximação TT-Cross**: Algoritmo adaptativo que constrói decomposições
   TT de tensores de alta dimensão usando apenas avaliações seletivas da
   função original. Reduz complexidade exponencial para linear.

2. **Atualizações Locais ALS**: Refinamento iterativo da decomposição usando
   Alternating Least Squares. Permite aprendizado incremental e adaptação
   a novos dados sem reconstruir toda a aproximação.

3. **Adaptação de Ranks**: Ajuste dinâmico da capacidade representacional
   do TT, permitindo trade-off entre precisão e eficiência computacional.

O motor fornece interface unificada com checkpointing automático, geração
de manifestos de auditoria e suporte opcional para aceleração GPU.

INTEGRAÇÃO: Conecta-se com RAFAELIA_TT_CROSS_FULL.py e 
RAFAELIA_TT_UPDATE_FULL.py para operações específicas.

ENGLISH:
--------
This module implements the main Tensor Train (TT) processing engine for the
RAFAELIA framework. It orchestrates three fundamental components:

1. **TT-Cross Approximation**: Adaptive algorithm that builds TT decompositions
   of high-dimensional tensors using only selective evaluations of the original
   function. Reduces exponential complexity to linear.

2. **ALS Local Updates**: Iterative refinement using Alternating Least Squares.
   Enables incremental learning and adaptation to new data without rebuilding
   the entire approximation.

3. **Rank Adaptation**: Dynamic adjustment of TT representational capacity,
   allowing trade-off between accuracy and computational efficiency.

The engine provides unified interface with automatic checkpointing, audit
manifest generation, and optional GPU acceleration support.

INTEGRATION: Connects to RAFAELIA_TT_CROSS_FULL.py and 
RAFAELIA_TT_UPDATE_FULL.py for specific operations.

====================================================================

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
    Motor Fullstack TT integrando aproximação cruzada e atualizações.
    Fullstack TT Engine integrating cross-approximation and updates.
    
    Fornece interface de alto nível para operações tensoriais com adaptação
    automática de ranks, checkpoints e geração de manifesto RAFAELIA.
    
    Provides high-level interface for tensor operations with automatic
    rank adaptation, checkpointing, and RAFAELIA manifest generation.
    """
    
    def __init__(self, config: Optional[Dict[str, Any]] = None):
        """
        Inicializa o Motor RAFAELIA / Initialize RAFAELIA Engine.
        
        Args:
            config: Dicionário de configuração com opções:
                - use_gpu: Habilita aceleração GPU (bool)
                - checkpoint_dir: Diretório para checkpoints (str)
                - auto_checkpoint: Salvamento automático após operações (bool)
                - compression: Usa compressão zstd (bool)
        """
        # Configuração básica / Basic configuration
        self.config = config or {}
        self.use_gpu = self.config.get('use_gpu', False) and HAS_CUPY
        self.checkpoint_dir = Path(self.config.get('checkpoint_dir', '/tmp'))
        self.auto_checkpoint = self.config.get('auto_checkpoint', True)
        self.compression = self.config.get('compression', True) and HAS_ZSTD
        
        # Estado do motor / Engine state
        # tt_cross: objeto de aproximação cruzada TT / TT cross approximation object
        # tt_update: objeto de atualização local / local update object
        self.tt_cross = None
        self.tt_update = None
        
        # Metadados para auditoria e rastreamento / Metadata for audit and tracking
        self.metadata = {
            'created': time.time(),
            'operations': [],  # histórico de operações / operation history
            'rafaelia': {
                'signature': 'RAFCODE-Φ-∆RafaelVerboΩ',
                'module': 'ENGINE_FULLSTACK',
                'philosophy': 'VAZIO → VERBO → CHEIO → RETRO'
            }
        }
        
        # Garante que diretório de checkpoint existe / Ensure checkpoint directory exists
        self.checkpoint_dir.mkdir(parents=True, exist_ok=True)
    
    def approximate_tensor(self, func: Callable, shape: List[int],
                          ranks: List[int], **kwargs) -> Dict[str, Any]:
        """
        Aproxima tensor de alta dimensão usando TT-cross.
        Approximate high-dimensional tensor using TT-cross.
        
        Este método implementa a aproximação cruzada de Tensor Train (TT-cross),
        uma técnica eficiente para representar tensores de alta dimensão com
        poucos parâmetros, usando decomposição de baixo rank.
        
        Args:
            func: Função a aproximar (recebe lista de índices e retorna valor escalar)
            shape: Dimensões do tensor (ex: [10, 20, 30] para tensor 3D)
            ranks: Ranks TT entre cada dimensão (ex: [1, 5, 7, 1])
            **kwargs: Argumentos adicionais:
                - epsilon: Tolerância de erro (padrão: 1e-6)
                - max_iter: Número máximo de iterações (padrão: 100)
                - verbose: Mostra progresso (padrão: False)
            
        Returns:
            Dicionário com resultados da aproximação contendo:
            - converged: Se convergiu
            - error: Erro final
            - iterations: Número de iterações
            - elapsed_time: Tempo decorrido
        """
        print(f"Starting TT-cross approximation...")
        print(f"  Shape: {shape}")
        print(f"  Ranks: {ranks}")
        
        start_time = time.time()
        
        # Cria objeto de aproximação cruzada / Create cross approximation object
        # O TT-cross é um algoritmo adaptativo que seleciona automaticamente
        # os índices mais importantes para a aproximação
        self.tt_cross = TTCrossApproximation(
            shape=shape,
            ranks=ranks,
            use_gpu=self.use_gpu,
            epsilon=kwargs.get('epsilon', 1e-6)
        )
        
        # Executa aproximação iterativa / Perform iterative approximation
        # O algoritmo refina os cores TT até convergência ou max_iter
        stats = self.tt_cross.cross_approximation(
            func=func,
            max_iter=kwargs.get('max_iter', 100),
            verbose=kwargs.get('verbose', False)
        )
        
        elapsed = time.time() - start_time
        stats['elapsed_time'] = elapsed
        
        # Registra operação no histórico / Record operation in history
        self.metadata['operations'].append({
            'type': 'cross_approximation',
            'timestamp': time.time(),
            'shape': shape,
            'ranks': ranks,
            'stats': stats
        })
        
        # Checkpoint automático se habilitado / Auto-checkpoint if enabled
        if self.auto_checkpoint:
            self._save_checkpoint('tt_cross_auto')
        
        print(f"Approximation complete in {elapsed:.2f}s")
        print(f"  Converged: {stats['converged']}")
        print(f"  Final error: {stats['error']:.2e}")
        
        return stats
    
    def update_tensor(self, target_data: Dict[Tuple, float],
                     **kwargs) -> Dict[str, Any]:
        """
        Atualiza decomposição TT usando atualizações locais (ALS).
        Update TT decomposition using local updates (ALS).
        
        Este método usa o algoritmo Alternating Least Squares (ALS) para
        ajustar a decomposição TT existente para melhor aproximar novos dados.
        É útil para refinamento adaptativo e aprendizado incremental.
        
        Args:
            target_data: Dicionário mapeando índices para valores alvo
                        Ex: {(0,1,2): 3.5, (1,2,3): 4.2}
            **kwargs: Argumentos adicionais:
                - n_iterations: Número de iterações ALS (padrão: 10)
                - verbose: Mostra progresso (padrão: False)
            
        Returns:
            Dicionário com resultados da atualização contendo:
            - final_error: Erro final médio
            - iteration_errors: Erros por iteração
            - elapsed_time: Tempo decorrido
        
        Raises:
            RuntimeError: Se approximate_tensor não foi executado primeiro
        """
        if self.tt_cross is None:
            raise RuntimeError("Must run approximate_tensor first")
        
        print(f"Starting TT local update...")
        print(f"  Target samples: {len(target_data)}")
        
        start_time = time.time()
        
        # Cria atualizador a partir dos cores da aproximação cruzada
        # Create updater from cross approximation cores
        self.tt_update = TTLocalUpdate(
            cores=self.tt_cross.cores,
            use_gpu=self.use_gpu
        )
        
        # Executa varreduras ALS (Alternating Least Squares)
        # Perform ALS sweeps
        # ALS otimiza cada core mantendo os outros fixos, alternadamente
        stats = self.tt_update.als_sweep(
            target_data=target_data,
            n_iterations=kwargs.get('n_iterations', 10),
            verbose=kwargs.get('verbose', False)
        )
        
        elapsed = time.time() - start_time
        stats['elapsed_time'] = elapsed
        
        # Atualiza cores da aproximação cruzada com resultados do ALS
        # Update cross approximation cores with ALS results
        self.tt_cross.cores = self.tt_update.cores
        
        # Registra operação / Record operation
        self.metadata['operations'].append({
            'type': 'local_update',
            'timestamp': time.time(),
            'n_samples': len(target_data),
            'stats': stats
        })
        
        # Checkpoint automático / Auto-checkpoint
        if self.auto_checkpoint:
            self._save_checkpoint('tt_update_auto')
        
        print(f"Update complete in {elapsed:.2f}s")
        print(f"  Final error: {stats['final_error']:.2e}")
        
        return stats
    
    def adapt_ranks(self, core_idx: int, new_rank: int,
                   method: str = 'truncate') -> Dict[str, Any]:
        """
        Adapta ranks TT em posição específica.
        Adapt TT ranks at specified position.
        
        Permite ajustar dinamicamente a capacidade representacional do TT
        aumentando (expand) ou reduzindo (truncate) os ranks.
        
        Args:
            core_idx: Índice do core onde o rank muda (0 a d-2, onde d é número de dimensões)
            new_rank: Novo valor do rank
            method: Método de adaptação:
                - 'truncate': Reduz rank (remove componentes menos importantes)
                - 'expand': Aumenta rank (adiciona novos componentes)
            
        Returns:
            Dicionário com resultados da adaptação contendo:
            - old_rank: Rank anterior
            - new_rank: Novo rank
            - method: Método usado
            - elapsed_time: Tempo decorrido
        
        Raises:
            RuntimeError: Se não há decomposição TT disponível
        """
        if self.tt_update is None:
            if self.tt_cross is not None:
                # Inicializa atualizador se só temos aproximação / Initialize updater if we only have approximation
                self.tt_update = TTLocalUpdate(
                    cores=self.tt_cross.cores,
                    use_gpu=self.use_gpu
                )
            else:
                raise RuntimeError("No TT decomposition available")
        
        print(f"Adapting rank at position {core_idx} to {new_rank}...")
        
        old_rank = self.tt_update.ranks[core_idx + 1]
        start_time = time.time()
        
        # Executa adaptação de rank / Perform rank adaptation
        self.tt_update.rank_adaptation(core_idx, new_rank, method)
        
        elapsed = time.time() - start_time
        
        # Sincroniza com aproximação cruzada se existir / Update cross approximation if it exists
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
        """
        Avalia TT em índices específicos / Evaluate TT at given indices.
        
        Args:
            indices: Lista de índices para avaliar (um por dimensão)
        
        Returns:
            Valor escalar da aproximação TT nesses índices
        
        Raises:
            RuntimeError: Se não há decomposição TT disponível
        """
        if self.tt_cross is not None:
            return self.tt_cross.evaluate(indices)
        elif self.tt_update is not None:
            return self.tt_update._evaluate(tuple(indices))
        else:
            raise RuntimeError("No TT decomposition available")
    
    def _save_checkpoint(self, name: str):
        """
        Salva checkpoint com manifesto RAFAELIA / Save checkpoint with RAFAELIA manifest.
        
        Args:
            name: Nome base do checkpoint (timestamp será adicionado)
        """
        timestamp = int(time.time())
        filepath = self.checkpoint_dir / f"{name}_{timestamp}.json"
        
        if self.tt_cross is not None:
            self.tt_cross.save_checkpoint(str(filepath), metadata=self.metadata)
        elif self.tt_update is not None:
            self.tt_update.save_checkpoint(str(filepath), metadata=self.metadata)
    
    def generate_manifest(self, output_path: Optional[str] = None) -> Dict[str, Any]:
        """
        Gera manifesto RAFAELIA para estado atual.
        Generate RAFAELIA manifest for current state.
        
        O manifesto contém metadados completos sobre a execução, incluindo
        configuração, operações realizadas, estado do TT e hashes de integridade.
        Essencial para auditoria e reprodutibilidade.
        
        Args:
            output_path: Caminho opcional para salvar manifesto JSON
            
        Returns:
            Dicionário com manifesto completo contendo:
            - signature: Assinatura RAFAELIA
            - timestamp: Momento de geração
            - module: Nome do módulo
            - philosophy: Filosofia VAZIO→VERBO→CHEIO→RETRO
            - metadata: Histórico de operações
            - config: Configuração do motor
            - tt_state: Estado da decomposição TT (se disponível)
            - hashes: Hashes de integridade (SHA256, Blake3)
        """
        # Estrutura base do manifesto / Base manifest structure
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
        
        # Adiciona estado TT se disponível / Add TT state if available
        if self.tt_cross is not None:
            manifest['tt_state'] = {
                'shape': self.tt_cross.shape,
                'ranks': self.tt_cross.ranks,
                'epsilon': self.tt_cross.epsilon
            }
        
        # Computa hashes do manifesto para integridade / Compute manifest hash for integrity
        manifest_str = json.dumps(manifest['metadata'], sort_keys=True)
        manifest['hashes'] = {
            'sha256': hashlib.sha256(manifest_str.encode()).hexdigest()
        }
        
        # Adiciona Blake3 se disponível (mais rápido que SHA256) / Add Blake3 if available
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
    """
    Demonstração do Motor RAFAELIA / Demonstration of RAFAELIA Engine.
    
    Executa exemplo completo mostrando:
    1. Aproximação TT-cross de uma função
    2. Atualização local com ALS
    3. Avaliação de pontos
    4. Geração de manifesto
    """
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
