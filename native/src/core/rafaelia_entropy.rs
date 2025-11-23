/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
 *
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */

use std::collections::HashMap;
use std::f64;

/// Entropy analysis result
#[derive(Debug, Clone)]
pub struct EntropyAnalysis {
    /// Shannon entropy in bits
    pub shannon_entropy: f64,
    /// Normalized entropy (0 to 1)
    pub normalized_entropy: f64,
    /// Data size in bytes
    pub data_size: usize,
    /// Number of unique values
    pub unique_values: usize,
    /// Compression potential (0 to 1, higher = more compressible)
    pub compression_potential: f64,
}

/// Invariant validation result
#[derive(Debug, Clone)]
pub struct InvariantCheck {
    /// Whether all invariants hold
    pub valid: bool,
    /// Number of invariants checked
    pub total_checks: usize,
    /// Number of failed checks
    pub failed_checks: usize,
    /// Descriptions of failed invariants
    pub failures: Vec<String>,
}

/// Coherence metrics for data structure
#[derive(Debug, Clone)]
pub struct CoherenceMetrics {
    /// Temporal coherence (0 to 1)
    pub temporal: f64,
    /// Spatial coherence (0 to 1)
    pub spatial: f64,
    /// Logical coherence (0 to 1)
    pub logical: f64,
    /// Overall coherence score (0 to 1)
    pub overall: f64,
}

/// Calculate Shannon entropy of byte array.
///
/// H(X) = -Σ p(x) log₂ p(x)
///
/// # Arguments
/// * `data` - Input data buffer
///
/// # Returns
/// * Entropy in bits per byte
pub fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    // Count frequency of each byte value
    let mut freq: HashMap<u8, usize> = HashMap::new();
    for &byte in data {
        *freq.entry(byte).or_insert(0) += 1;
    }

    // Calculate entropy
    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &count in freq.values() {
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }

    entropy
}

/// Perform comprehensive entropy analysis.
///
/// # Arguments
/// * `data` - Input data buffer
///
/// # Returns
/// * EntropyAnalysis structure with metrics
pub fn analyze_entropy(data: &[u8]) -> EntropyAnalysis {
    if data.is_empty() {
        return EntropyAnalysis {
            shannon_entropy: 0.0,
            normalized_entropy: 0.0,
            data_size: 0,
            unique_values: 0,
            compression_potential: 1.0,
        };
    }

    let entropy = shannon_entropy(data);
    let max_entropy = 8.0; // Maximum entropy for byte (log₂(256))
    let normalized = entropy / max_entropy;

    // Count unique values
    let mut unique: HashMap<u8, ()> = HashMap::new();
    for &byte in data {
        unique.insert(byte, ());
    }
    let unique_count = unique.len();

    // Compression potential (lower entropy = higher potential)
    let compression = 1.0 - normalized;

    EntropyAnalysis {
        shannon_entropy: entropy,
        normalized_entropy: normalized,
        data_size: data.len(),
        unique_values: unique_count,
        compression_potential: compression,
    }
}

/// Calculate approximate entropy for time series data.
///
/// ApEn(m, r, N) = φ(m) - φ(m+1)
/// where φ(m) = (N-m+1)⁻¹ Σ log(Cᵢᵐ(r))
///
/// # Arguments
/// * `data` - Time series data
/// * `m` - Pattern length
/// * `r` - Tolerance (as fraction of std dev)
///
/// # Returns
/// * Approximate entropy value
pub fn approximate_entropy(data: &[f64], m: usize, r: f64) -> f64 {
    if data.len() < m + 1 {
        return 0.0;
    }

    let n = data.len();

    // Calculate standard deviation
    let mean: f64 = data.iter().sum::<f64>() / n as f64;
    let variance: f64 = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n as f64;
    let std_dev = variance.sqrt();
    let tolerance = r * std_dev;

    // Calculate phi(m) and phi(m+1)
    let phi_m = calculate_phi(data, m, tolerance, n);
    let phi_m1 = calculate_phi(data, m + 1, tolerance, n);

    phi_m - phi_m1
}

/// Helper function to calculate phi for approximate entropy.
fn calculate_phi(data: &[f64], m: usize, r: f64, n: usize) -> f64 {
    let mut patterns: Vec<f64> = Vec::new();

    for i in 0..=(n - m) {
        let mut count = 0;

        for j in 0..=(n - m) {
            // Check if patterns match within tolerance
            let mut matches = true;
            for k in 0..m {
                if (data[i + k] - data[j + k]).abs() > r {
                    matches = false;
                    break;
                }
            }
            if matches {
                count += 1;
            }
        }

        let c = count as f64 / (n - m + 1) as f64;
        if c > 0.0 {
            patterns.push(c.ln());
        }
    }

    if patterns.is_empty() {
        return 0.0;
    }

    patterns.iter().sum::<f64>() / patterns.len() as f64
}

/// Validate structural invariants of data.
///
/// Checks common invariants like:
/// - Size constraints
/// - Checksum validity
/// - Ordering properties
/// - Consistency rules
///
/// # Arguments
/// * `data` - Data to validate
/// * `invariants` - Vector of invariant check functions
///
/// # Returns
/// * InvariantCheck result
pub fn validate_invariants<F>(data: &[u8], invariants: &[F]) -> InvariantCheck
where
    F: Fn(&[u8]) -> Result<(), String>,
{
    let mut failures = Vec::new();
    let total = invariants.len();

    for (i, check) in invariants.iter().enumerate() {
        if let Err(msg) = check(data) {
            failures.push(format!("Invariant {}: {}", i, msg));
        }
    }

    let failed = failures.len();

    InvariantCheck {
        valid: failed == 0,
        total_checks: total,
        failed_checks: failed,
        failures,
    }
}

/// Calculate coherence metrics for data structure.
///
/// Measures different aspects of coherence:
/// - Temporal: Consistency over time/sequence
/// - Spatial: Locality and structure
/// - Logical: Semantic consistency
///
/// # Arguments
/// * `data` - Input data
///
/// # Returns
/// * CoherenceMetrics structure
pub fn calculate_coherence(data: &[u8]) -> CoherenceMetrics {
    if data.is_empty() {
        return CoherenceMetrics {
            temporal: 0.0,
            spatial: 0.0,
            logical: 0.0,
            overall: 0.0,
        };
    }

    // Temporal coherence: smoothness of transitions
    let temporal = calculate_temporal_coherence(data);

    // Spatial coherence: locality of similar values
    let spatial = calculate_spatial_coherence(data);

    // Logical coherence: structural consistency
    let logical = calculate_logical_coherence(data);

    // Overall coherence (weighted average)
    let overall = (temporal * 0.3 + spatial * 0.4 + logical * 0.3);

    CoherenceMetrics {
        temporal,
        spatial,
        logical,
        overall,
    }
}

/// Calculate temporal coherence (smoothness).
fn calculate_temporal_coherence(data: &[u8]) -> f64 {
    if data.len() < 2 {
        return 1.0;
    }

    let mut differences: Vec<i32> = Vec::new();

    for i in 1..data.len() {
        let diff = (data[i] as i32 - data[i - 1] as i32).abs();
        differences.push(diff);
    }

    // Lower average difference = higher coherence
    let avg_diff = differences.iter().sum::<i32>() as f64 / differences.len() as f64;
    let max_diff = 255.0;

    1.0 - (avg_diff / max_diff).min(1.0)
}

/// Calculate spatial coherence (locality).
fn calculate_spatial_coherence(data: &[u8]) -> f64 {
    if data.len() < 4 {
        return 1.0;
    }

    // Check locality in windows
    let window_size = 16.min(data.len() / 4);
    let mut coherence_sum = 0.0;
    let mut num_windows = 0;

    for i in (0..data.len()).step_by(window_size) {
        let end = (i + window_size).min(data.len());
        let window = &data[i..end];

        // Calculate variance within window
        let mean: f64 = window.iter().map(|&x| x as f64).sum::<f64>() / window.len() as f64;
        let variance: f64 = window
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>()
            / window.len() as f64;

        // Lower variance = higher spatial coherence
        let std_dev = variance.sqrt();
        let coherence = 1.0 - (std_dev / 128.0).min(1.0);

        coherence_sum += coherence;
        num_windows += 1;
    }

    coherence_sum / num_windows as f64
}

/// Calculate logical coherence (structural consistency).
fn calculate_logical_coherence(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 1.0;
    }

    // Check for repeating patterns (indicates structure)
    let pattern_score = detect_patterns(data);

    // Check for balanced byte distribution
    let distribution_score = check_distribution(data);

    // Combine scores
    (pattern_score + distribution_score) / 2.0
}

/// Detect repeating patterns in data.
fn detect_patterns(data: &[u8]) -> f64 {
    if data.len() < 8 {
        return 0.5;
    }

    let mut pattern_count = 0;
    let pattern_len = 4;

    for i in 0..(data.len() - 2 * pattern_len) {
        let pattern = &data[i..i + pattern_len];

        // Search for repetition
        for j in (i + pattern_len)..(data.len() - pattern_len) {
            if &data[j..j + pattern_len] == pattern {
                pattern_count += 1;
                break;
            }
        }
    }

    // More patterns = more structure = higher coherence
    let max_patterns = (data.len() / pattern_len) as f64;
    (pattern_count as f64 / max_patterns).min(1.0)
}

/// Check byte value distribution balance.
fn check_distribution(data: &[u8]) -> f64 {
    let entropy = shannon_entropy(data);
    let max_entropy = 8.0;

    // Balanced distribution has high entropy
    entropy / max_entropy
}

/// Detect anomalies based on entropy deviation.
///
/// # Arguments
/// * `data` - Input data
/// * `expected_entropy` - Expected entropy value
/// * `tolerance` - Acceptable deviation (0 to 1)
///
/// # Returns
/// * true if anomaly detected
pub fn detect_entropy_anomaly(data: &[u8], expected_entropy: f64, tolerance: f64) -> bool {
    let entropy = shannon_entropy(data);
    let deviation = (entropy - expected_entropy).abs() / 8.0; // Normalize by max entropy

    deviation > tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shannon_entropy_uniform() {
        // Uniform distribution should have maximum entropy
        let data: Vec<u8> = (0..=255).collect();
        let entropy = shannon_entropy(&data);
        assert!(entropy > 7.9); // Close to 8.0 bits
    }

    #[test]
    fn test_shannon_entropy_constant() {
        // Constant data should have zero entropy
        let data = vec![42u8; 100];
        let entropy = shannon_entropy(&data);
        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn test_analyze_entropy() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let analysis = analyze_entropy(&data);

        assert!(analysis.shannon_entropy > 0.0);
        assert!(analysis.normalized_entropy > 0.0);
        assert_eq!(analysis.data_size, 8);
        assert_eq!(analysis.unique_values, 8);
    }

    #[test]
    fn test_validate_invariants() {
        let data = vec![1, 2, 3, 4, 5];

        let checks: Vec<Box<dyn Fn(&[u8]) -> Result<(), String>>> = vec![
            Box::new(|d| {
                if d.len() == 5 {
                    Ok(())
                } else {
                    Err("Length must be 5".to_string())
                }
            }),
            Box::new(|d| {
                if d[0] < d[d.len() - 1] {
                    Ok(())
                } else {
                    Err("First must be less than last".to_string())
                }
            }),
        ];

        let result = validate_invariants(&data, &checks);
        assert!(result.valid);
        assert_eq!(result.total_checks, 2);
        assert_eq!(result.failed_checks, 0);
    }

    #[test]
    fn test_coherence_metrics() {
        let data = vec![10, 11, 12, 13, 14, 15, 16, 17]; // Smooth sequence
        let metrics = calculate_coherence(&data);

        assert!(metrics.temporal > 0.8); // High temporal coherence
        assert!(metrics.overall > 0.0);
    }
}
