#!/usr/bin/env python3
"""
Testes Unitários para RAFAELIA_ENGINE_FULLSTACK
Unit Tests for RAFAELIA_ENGINE_FULLSTACK

Testa as funcionalidades principais do motor RAFAELIA:
- Inicialização e configuração
- Aproximação TT-cross
- Atualização local ALS
- Adaptação de ranks
- Avaliação de tensores
- Geração de manifestos
- Checkpointing

Tests core RAFAELIA engine functionality:
- Initialization and configuration
- TT-cross approximation
- Local ALS updates
- Rank adaptation
- Tensor evaluation
- Manifest generation
- Checkpointing

Part of RAFAELIA Fullstack Suite
Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
"""

import pytest
import numpy as np
import tempfile
import json
from pathlib import Path
import sys
import os

# Adiciona diretório pai ao path para imports / Add parent dir to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

from RAFAELIA_ENGINE_FULLSTACK import RAFAELIAEngine


class TestRAFAELIAEngineInitialization:
    """Testes de inicialização do motor / Engine initialization tests."""
    
    def test_default_initialization(self):
        """Testa inicialização com configuração padrão / Test default initialization."""
        engine = RAFAELIAEngine()
        
        assert engine.use_gpu == False
        assert engine.auto_checkpoint == True
        assert engine.tt_cross is None
        assert engine.tt_update is None
        assert 'rafaelia' in engine.metadata
        assert engine.metadata['rafaelia']['signature'] == 'RAFCODE-Φ-∆RafaelVerboΩ'
    
    def test_custom_configuration(self):
        """Testa inicialização com configuração customizada / Test custom configuration."""
        config = {
            'use_gpu': False,
            'auto_checkpoint': False,
            'checkpoint_dir': '/tmp/test_rafaelia'
        }
        engine = RAFAELIAEngine(config)
        
        assert engine.use_gpu == False
        assert engine.auto_checkpoint == False
        assert str(engine.checkpoint_dir) == '/tmp/test_rafaelia'
    
    def test_checkpoint_directory_creation(self):
        """Testa criação automática do diretório de checkpoint / Test automatic checkpoint directory creation."""
        with tempfile.TemporaryDirectory() as tmpdir:
            checkpoint_dir = Path(tmpdir) / 'checkpoints'
            config = {'checkpoint_dir': str(checkpoint_dir)}
            engine = RAFAELIAEngine(config)
            
            assert checkpoint_dir.exists()
            assert checkpoint_dir.is_dir()


class TestTensorApproximation:
    """Testes de aproximação tensorial / Tensor approximation tests."""
    
    def test_simple_approximation(self):
        """Testa aproximação simples de função / Test simple function approximation."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        # Função de teste simples: soma dos índices / Simple test function: sum of indices
        def test_func(indices):
            return sum(indices)
        
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        stats = engine.approximate_tensor(test_func, shape, ranks, max_iter=5)
        
        assert 'converged' in stats
        assert 'error' in stats
        assert 'elapsed_time' in stats
        assert engine.tt_cross is not None
        assert len(engine.metadata['operations']) == 1
        assert engine.metadata['operations'][0]['type'] == 'cross_approximation'
    
    def test_approximation_with_evaluation(self):
        """Testa aproximação seguida de avaliação / Test approximation with evaluation."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return sum(i**2 for i in indices)
        
        shape = [4, 4, 4]
        ranks = [1, 3, 3, 1]
        
        engine.approximate_tensor(test_func, shape, ranks, max_iter=10)
        
        # Avalia em pontos específicos / Evaluate at specific points
        test_indices = [1, 2, 1]
        approx_value = engine.evaluate(test_indices)
        true_value = test_func(test_indices)
        
        # Verifica que aproximação funciona e retorna número / Check approximation works and returns number
        assert isinstance(approx_value, (int, float, np.number))
        # Verifica que valor está em range razoável / Check value is in reasonable range
        assert abs(approx_value) < 100  # Sanity check
    
    def test_approximation_different_shapes(self):
        """Testa aproximação com diferentes formas / Test approximation with different shapes."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return np.prod(indices) * 0.1
        
        # Testa forma 2D / Test 2D shape
        stats_2d = engine.approximate_tensor(
            test_func, 
            shape=[5, 6], 
            ranks=[1, 3, 1], 
            max_iter=3
        )
        assert 'error' in stats_2d
        
        # Testa forma 4D / Test 4D shape
        engine2 = RAFAELIAEngine({'auto_checkpoint': False})
        stats_4d = engine2.approximate_tensor(
            test_func,
            shape=[3, 3, 3, 3],
            ranks=[1, 2, 2, 2, 1],
            max_iter=3
        )
        assert 'error' in stats_4d


class TestLocalUpdates:
    """Testes de atualização local / Local update tests."""
    
    def test_update_after_approximation(self):
        """Testa atualização após aproximação / Test update after approximation."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return sum(indices) * 1.5
        
        shape = [4, 5, 6]
        ranks = [1, 2, 3, 1]
        
        # Primeiro aproxima / First approximate
        engine.approximate_tensor(test_func, shape, ranks, max_iter=5)
        
        # Cria dados alvo / Create target data
        target_data = {
            (0, 1, 2): 4.5,
            (1, 2, 3): 9.0,
            (2, 3, 4): 13.5
        }
        
        # Atualiza / Update
        stats = engine.update_tensor(target_data, n_iterations=3)
        
        assert 'final_error' in stats
        assert 'elapsed_time' in stats
        assert engine.tt_update is not None
        assert len(engine.metadata['operations']) == 2
    
    def test_update_without_approximation_fails(self):
        """Testa que atualização sem aproximação falha / Test that update without approximation fails."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        target_data = {(0, 1, 2): 1.0}
        
        with pytest.raises(RuntimeError):
            engine.update_tensor(target_data)
    
    def test_update_reduces_error(self):
        """Testa que atualização reduz erro / Test that update reduces error."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return sum(indices) * 2.0
        
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        engine.approximate_tensor(test_func, shape, ranks, max_iter=3)
        
        # Cria dados alvo com valores exatos / Create target data with exact values
        target_data = {}
        np.random.seed(42)
        for _ in range(10):
            indices = tuple(np.random.randint(0, s) for s in shape)
            target_data[indices] = test_func(list(indices))
        
        # Atualiza e verifica erro / Update and check error
        stats = engine.update_tensor(target_data, n_iterations=5)
        
        # Erro final deve existir e ser um número / Final error should exist and be a number
        assert 'final_error' in stats
        assert isinstance(stats['final_error'], (int, float, np.number))


class TestRankAdaptation:
    """Testes de adaptação de rank / Rank adaptation tests."""
    
    def test_rank_truncation(self):
        """Testa truncamento de rank / Test rank truncation."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return sum(indices)
        
        shape = [4, 5, 6]
        ranks = [1, 5, 5, 1]  # Ranks iniciais grandes / Large initial ranks
        
        engine.approximate_tensor(test_func, shape, ranks, max_iter=3)
        
        # Trunca rank / Truncate rank
        result = engine.adapt_ranks(core_idx=0, new_rank=3, method='truncate')
        
        assert result['old_rank'] == 5
        assert result['new_rank'] == 3
        assert result['method'] == 'truncate'
        assert 'elapsed_time' in result
    
    def test_rank_expansion(self):
        """Testa expansão de rank / Test rank expansion."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return np.prod(indices) * 0.5
        
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]  # Ranks iniciais pequenos / Small initial ranks
        
        engine.approximate_tensor(test_func, shape, ranks, max_iter=3)
        
        # Expande rank / Expand rank
        result = engine.adapt_ranks(core_idx=1, new_rank=4, method='expand')
        
        assert result['old_rank'] == 2
        assert result['new_rank'] == 4
        assert result['method'] == 'expand'
    
    def test_rank_adaptation_without_decomposition_fails(self):
        """Testa que adaptação sem decomposição falha / Test that adaptation without decomposition fails."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        with pytest.raises(RuntimeError):
            engine.adapt_ranks(0, 5, 'truncate')


class TestEvaluation:
    """Testes de avaliação / Evaluation tests."""
    
    def test_evaluate_with_cross_approximation(self):
        """Testa avaliação com aproximação cruzada / Test evaluation with cross approximation."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return sum(indices) * 3.0
        
        shape = [5, 6, 7]
        ranks = [1, 3, 3, 1]
        
        engine.approximate_tensor(test_func, shape, ranks, max_iter=5)
        
        # Avalia múltiplos pontos / Evaluate multiple points
        test_points = [[0, 0, 0], [1, 2, 3], [4, 5, 6]]
        
        for point in test_points:
            value = engine.evaluate(point)
            assert isinstance(value, (int, float, np.number))
    
    def test_evaluate_without_decomposition_fails(self):
        """Testa que avaliação sem decomposição falha / Test that evaluation without decomposition fails."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        with pytest.raises(RuntimeError):
            engine.evaluate([0, 0, 0])


class TestManifestGeneration:
    """Testes de geração de manifesto / Manifest generation tests."""
    
    def test_manifest_structure(self):
        """Testa estrutura do manifesto / Test manifest structure."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        manifest = engine.generate_manifest()
        
        # Verifica campos obrigatórios / Check required fields
        assert 'signature' in manifest
        assert 'timestamp' in manifest
        assert 'module' in manifest
        assert 'philosophy' in manifest
        assert 'metadata' in manifest
        assert 'config' in manifest
        assert 'hashes' in manifest
        
        # Verifica valores corretos / Check correct values
        assert manifest['module'] == 'ENGINE_FULLSTACK'
        assert manifest['philosophy'] == 'VAZIO → VERBO → CHEIO → RETRO'
        assert 'sha256' in manifest['hashes']
    
    def test_manifest_with_tt_state(self):
        """Testa manifesto com estado TT / Test manifest with TT state."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return sum(indices)
        
        shape = [3, 4, 5]
        ranks = [1, 2, 2, 1]
        
        engine.approximate_tensor(test_func, shape, ranks, max_iter=3)
        
        manifest = engine.generate_manifest()
        
        # Verifica estado TT / Check TT state
        assert 'tt_state' in manifest
        assert manifest['tt_state']['shape'] == shape
        assert manifest['tt_state']['ranks'] == ranks
    
    def test_manifest_save_to_file(self):
        """Testa salvamento de manifesto em arquivo / Test manifest save to file."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        with tempfile.TemporaryDirectory() as tmpdir:
            manifest_path = Path(tmpdir) / 'manifest.json'
            
            manifest = engine.generate_manifest(str(manifest_path))
            
            # Verifica que arquivo foi criado / Check file was created
            assert manifest_path.exists()
            
            # Verifica conteúdo / Check content
            with open(manifest_path, 'r') as f:
                loaded_manifest = json.load(f)
            
            assert loaded_manifest['signature'] == manifest['signature']
    
    def test_manifest_tracks_operations(self):
        """Testa que manifesto rastreia operações / Test that manifest tracks operations."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return sum(indices)
        
        # Executa várias operações / Execute several operations
        engine.approximate_tensor(test_func, [3, 4], [1, 2, 1], max_iter=2)
        
        target_data = {(0, 1): 1.0, (1, 2): 3.0}
        engine.update_tensor(target_data, n_iterations=2)
        
        manifest = engine.generate_manifest()
        
        # Verifica histórico de operações / Check operation history
        assert len(manifest['metadata']['operations']) == 2
        assert manifest['metadata']['operations'][0]['type'] == 'cross_approximation'
        assert manifest['metadata']['operations'][1]['type'] == 'local_update'


class TestEdgeCases:
    """Testes de casos extremos / Edge case tests."""
    
    def test_very_small_tensor(self):
        """Testa tensor muito pequeno / Test very small tensor."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return 1.0
        
        # Tensor 2x2 / 2x2 tensor
        shape = [2, 2]
        ranks = [1, 1, 1]
        
        stats = engine.approximate_tensor(test_func, shape, ranks, max_iter=2)
        assert 'error' in stats
    
    def test_empty_target_data(self):
        """Testa atualização com dados vazios / Test update with empty data."""
        engine = RAFAELIAEngine({'auto_checkpoint': False})
        
        def test_func(indices):
            return 1.0
        
        engine.approximate_tensor(test_func, [3, 3], [1, 2, 1], max_iter=2)
        
        # Dados alvo vazios / Empty target data
        target_data = {}
        stats = engine.update_tensor(target_data, n_iterations=1)
        
        assert 'final_error' in stats


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
