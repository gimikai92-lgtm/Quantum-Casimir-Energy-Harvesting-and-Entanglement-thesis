//! Coherence testing structures and functions
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

/// Coherence testing error types
#[derive(Error, Debug)]
pub enum CoherenceError {
    /// Quantum simulation error
    #[error("Quantum simulation error: {0}")]
    QuantumError(String),

    /// Coherence time calculation error
    #[error("Coherence time calculation error: {0}")]
    CoherenceCalcError(String),

    /// Noise model error
    #[error("Noise model error: {0}")]
    NoiseModelError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Results from coherence testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceTestResults {
    /// Number of qubits tested
    pub n_qubits: usize,
    /// Individual qubit measurements
    pub qubit_measurements: Vec<Vec<CoherenceMeasurement>>,
    /// GHZ state measurements
    pub ghz_measurements: Vec<GHZMeasurement>,
    /// Statistics per qubit
    pub statistics: HashMap<usize, QubitStatistics>,
    /// Overall statistics
    pub overall_stats: OverallStatistics,
    /// Test duration
    pub duration: Duration,
    /// Test configuration
    pub config: TestConfig,
}

impl CoherenceTestResults {
    /// Create new results structure
    pub fn new(n_qubits: usize) -> Self {
        CoherenceTestResults {
            n_qubits,
            qubit_measurements: vec![Vec::new(); n_qubits],
            ghz_measurements: Vec::new(),
            statistics: HashMap::new(),
            overall_stats: OverallStatistics::new(),
            duration: Duration::default(),
            config: TestConfig::default(),
        }
    }

    /// Add a measurement for a specific qubit
    pub fn add_measurement(
        &mut self,
        qubit: usize,
        test_idx: usize,
        measurement: CoherenceMeasurement,
    ) {
        if qubit < self.n_qubits {
            if self.qubit_measurements[qubit].len() <= test_idx {
                self.qubit_measurements[qubit]
                    .resize(test_idx + 1, CoherenceMeasurement::default());
            }
            self.qubit_measurements[qubit][test_idx] = measurement;
        }
    }

    /// Calculate statistics from measurements
    pub fn calculate_statistics(&mut self) {
        // Calculate per-qubit statistics
        for qubit in 0..self.n_qubits {
            let measurements = &self.qubit_measurements[qubit];
            if !measurements.is_empty() {
                let stats = calculate_qubit_statistics(measurements);
                self.statistics.insert(qubit, stats);
            }
        }

        // Calculate overall statistics
        self.overall_stats = calculate_overall_statistics(&self.statistics, &self.ghz_measurements);
    }

    /// Get average T2 time for a qubit
    pub fn get_average_t2(&self, qubit: usize) -> Option<f64> {
        self.statistics.get(&qubit).map(|stats| stats.average_t2)
    }

    /// Get worst-case T2 (minimum)
    pub fn get_worst_t2(&self) -> Option<(usize, f64)> {
        self.statistics
            .iter()
            .map(|(&qubit, stats)| (qubit, stats.min_t2))
            .min_by(|(_, t2_a), (_, t2_b)| t2_a.partial_cmp(t2_b).unwrap())
    }

    /// Get best-case T2 (maximum)
    pub fn get_best_t2(&self) -> Option<(usize, f64)> {
        self.statistics
            .iter()
            .map(|(&qubit, stats)| (qubit, stats.max_t2))
            .max_by(|(_, t2_a), (_, t2_b)| t2_a.partial_cmp(t2_b).unwrap())
    }

    /// Export results to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export results to CSV
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("qubit,test_index,initial_fidelity,t2_estimate\n");

        for (qubit, measurements) in self.qubit_measurements.iter().enumerate() {
            for (test_idx, measurement) in measurements.iter().enumerate() {
                csv.push_str(&format!(
                    "{},{},{:.6},{:.3}\n",
                    qubit, test_idx, measurement.initial_fidelity, measurement.t2_estimate
                ));
            }
        }
        csv
    }
}

/// Individual coherence measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceMeasurement {
    /// Qubit index
    pub qubit_index: usize,
    /// Test index
    pub test_index: usize,
    /// Initial fidelity
    pub initial_fidelity: f64,
    /// Fidelity over time
    pub fidelity_over_time: Vec<f64>,
    /// Purity over time
    pub purity_over_time: Vec<f64>,
    /// Entropy over time
    pub entropy_over_time: Vec<f64>,
    /// Estimated T2 from this measurement
    pub t2_estimate: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for CoherenceMeasurement {
    fn default() -> Self {
        CoherenceMeasurement {
            qubit_index: 0,
            test_index: 0,
            initial_fidelity: 0.0,
            fidelity_over_time: Vec::new(),
            purity_over_time: Vec::new(),
            entropy_over_time: Vec::new(),
            t2_estimate: 0.0,
            timestamp: Utc::now(),
        }
    }
}

/// GHZ state measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHZMeasurement {
    /// Test index
    pub test_index: usize,
    /// Initial GHZ fidelity
    pub initial_fidelity: f64,
    /// Fidelity over time
    pub fidelity_over_time: Vec<f64>,
    /// Number of qubits in GHZ state
    pub n_qubits: usize,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Statistics for a single qubit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QubitStatistics {
    /// Average T2 time
    pub average_t2: f64,
    /// Minimum T2 time
    pub min_t2: f64,
    /// Maximum T2 time
    pub max_t2: f64,
    /// Standard deviation of T2
    pub std_t2: f64,
    /// Average initial fidelity
    pub average_fidelity: f64,
    /// Minimum fidelity
    pub min_fidelity: f64,
    /// Maximum fidelity
    pub max_fidelity: f64,
    /// Number of measurements
    pub n_measurements: usize,
}

/// Overall statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallStatistics {
    /// Average T2 across all qubits
    pub average_t2: f64,
    /// System T2 (worst-case qubit)
    pub system_t2: f64,
    /// Average fidelity across all qubits
    pub average_fidelity: f64,
    /// Average GHZ fidelity
    pub average_ghz_fidelity: f64,
    /// Total number of measurements
    pub total_measurements: usize,
}

impl OverallStatistics {
    /// Create new overall statistics
    fn new() -> Self {
        OverallStatistics {
            average_t2: 0.0,
            system_t2: 0.0,
            average_fidelity: 0.0,
            average_ghz_fidelity: 0.0,
            total_measurements: 0,
        }
    }
}

/// Test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    /// Number of time points per test
    pub time_points: usize,
    /// Maximum test duration (multiple of T2)
    pub max_duration_factor: f64,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
}

impl Default for TestConfig {
    fn default() -> Self {
        TestConfig {
            time_points: 10,
            max_duration_factor: 3.0,
            random_seed: None,
        }
    }
}

/// Calculate statistics for a qubit
fn calculate_qubit_statistics(measurements: &[CoherenceMeasurement]) -> QubitStatistics {
    let n = measurements.len();

    if n == 0 {
        return QubitStatistics {
            average_t2: 0.0,
            min_t2: 0.0,
            max_t2: 0.0,
            std_t2: 0.0,
            average_fidelity: 0.0,
            min_fidelity: 0.0,
            max_fidelity: 0.0,
            n_measurements: 0,
        };
    }

    let t2_values: Vec<f64> = measurements.iter().map(|m| m.t2_estimate).collect();
    let fidelity_values: Vec<f64> = measurements.iter().map(|m| m.initial_fidelity).collect();

    let avg_t2 = t2_values.iter().sum::<f64>() / n as f64;
    let min_t2 = t2_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_t2 = t2_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let variance_t2 = t2_values.iter().map(|&x| (x - avg_t2).powi(2)).sum::<f64>() / n as f64;
    let std_t2 = variance_t2.sqrt();

    let avg_fidelity = fidelity_values.iter().sum::<f64>() / n as f64;
    let min_fidelity = fidelity_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_fidelity = fidelity_values
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    QubitStatistics {
        average_t2: avg_t2,
        min_t2,
        max_t2,
        std_t2,
        average_fidelity: avg_fidelity,
        min_fidelity,
        max_fidelity,
        n_measurements: n,
    }
}

/// Calculate overall statistics
fn calculate_overall_statistics(
    qubit_stats: &HashMap<usize, QubitStatistics>,
    ghz_measurements: &[GHZMeasurement],
) -> OverallStatistics {
    let n_qubits = qubit_stats.len();

    if n_qubits == 0 {
        return OverallStatistics::new();
    }

    let total_t2: f64 = qubit_stats.values().map(|stats| stats.average_t2).sum();
    let total_fidelity: f64 = qubit_stats
        .values()
        .map(|stats| stats.average_fidelity)
        .sum();

    let system_t2 = qubit_stats
        .values()
        .map(|stats| stats.min_t2)
        .fold(f64::INFINITY, |a, b| a.min(b));

    let total_measurements: usize = qubit_stats.values().map(|stats| stats.n_measurements).sum();

    let ghz_avg = if !ghz_measurements.is_empty() {
        ghz_measurements
            .iter()
            .map(|m| m.initial_fidelity)
            .sum::<f64>()
            / ghz_measurements.len() as f64
    } else {
        0.0
    };

    OverallStatistics {
        average_t2: total_t2 / n_qubits as f64,
        system_t2,
        average_fidelity: total_fidelity / n_qubits as f64,
        average_ghz_fidelity: ghz_avg,
        total_measurements,
    }
}
