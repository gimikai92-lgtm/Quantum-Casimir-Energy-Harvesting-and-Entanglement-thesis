//! Minimal quantum coherence crate for tests
use anyhow::Result;
use nalgebra::DMatrix;
use num_complex::Complex64;

/// Physics constants
pub const HBAR: f64 = 1.054571817e-34;
pub const K_B: f64 = 1.380649e-23;

/// Simple noise model enum
#[derive(Debug, Clone)]
pub enum NoiseModel {
    Thermal { temperature: f64, rates: Vec<f64> },
}
impl NoiseModel {
    pub fn thermal_with_rates(temperature: f64, rates: Vec<f64>) -> Self {
        NoiseModel::Thermal { temperature, rates }
    }
    pub fn total_rate(&self) -> f64 {
        match self {
            NoiseModel::Thermal { rates, .. } => {
                if rates.is_empty() {
                    1.0
                } else {
                    rates.iter().sum::<f64>() / rates.len() as f64
                }
            }
        }
    }
    pub fn scale(&mut self, factor: f64) {
        match self {
            NoiseModel::Thermal { rates, .. } => {
                for r in rates.iter_mut() {
                    *r *= factor;
                }
            }
        }
    }
}

/// Coherence measurement (small subset)
#[derive(Debug, Clone)]
pub struct CoherenceMeasurement {
    pub qubit_index: usize,
    pub test_index: usize,
    pub initial_fidelity: f64,
    pub fidelity_over_time: Vec<f64>,
    pub purity_over_time: Vec<f64>,
    pub entropy_over_time: Vec<f64>,
    pub t2_estimate: f64,
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
        }
    }
}

/// Simplified QuantumSystem for tests
#[derive(Clone, Debug)]
pub struct QuantumSystem {
    pub n_qubits: usize,
    pub temperature: f64,
    pub density_matrix: DMatrix<Complex64>,
    pub t2_times: Vec<f64>,
}

impl QuantumSystem {
    pub fn new(n_qubits: usize, temperature: f64) -> Self {
        let dim = 1usize << n_qubits;
        let mut density_matrix = DMatrix::from_element(dim, dim, Complex64::new(0.0, 0.0));
        density_matrix[(0, 0)] = Complex64::new(1.0, 0.0);
        let base_t2 = 85.0;
        let t2_times = vec![base_t2; n_qubits];
        QuantumSystem {
            n_qubits,
            temperature,
            density_matrix,
            t2_times,
        }
    }

    pub fn calculate_purity(&self) -> f64 {
        let prod = &self.density_matrix * &self.density_matrix;
        prod.trace().re
    }

    pub fn create_ghz_state(&mut self) -> Result<()> {
        let dim = 1usize << self.n_qubits;
        let mut rho = DMatrix::from_element(dim, dim, Complex64::new(0.0, 0.0));
        let amp = 1.0_f64 / (2.0_f64).sqrt();
        let val = Complex64::new(amp * amp, 0.0);
        rho[(0, 0)] = val;
        rho[(dim - 1, dim - 1)] = val;
        rho[(0, dim - 1)] = val;
        rho[(dim - 1, 0)] = val;
        self.density_matrix = rho;
        Ok(())
    }

    /// Simple T2 coherence model: coherence(t) = exp(-t / T2)
    pub fn coherence_at_time(&self, qubit: usize, t: f64) -> f64 {
        if qubit >= self.t2_times.len() {
            panic!("qubit index out of range");
        }
        let t2 = self.t2_times[qubit];
        (-(t) / t2).exp()
    }

    /// Derive the ratio of coherence between two times using the stored T2 for `qubit`.
    /// Returns exp(-(t2 - t1) / T2)
    pub fn coherence_ratio(&self, qubit: usize, t1: f64, t2: f64) -> f64 {
        let c1 = self.coherence_at_time(qubit, t1);
        let c2 = self.coherence_at_time(qubit, t2);
        c2 / c1
    }

    /// Return purity of a GHZ state created on the system.
    pub fn purity_of_ghz(&self) -> f64 {
        let mut tmp = self.clone();
        let _ = tmp.create_ghz_state();
        tmp.calculate_purity()
    }
}

// Re-export types at crate root
// types are already public; no additional re-exports needed

// Optional Python bindings (enabled via `--features python`)
#[cfg(feature = "python")]
mod python_bindings {
    use super::QuantumSystem;
    use pyo3::prelude::*;

    /// Return coherence at time t for a newly created system with `n_qubits` at `temperature`.
    #[pyfunction]
    fn coherence_at_time_py(
        n_qubits: usize,
        temperature: f64,
        qubit: usize,
        t: f64,
    ) -> PyResult<f64> {
        let sys = QuantumSystem::new(n_qubits, temperature);
        Ok(sys.coherence_at_time(qubit, t))
    }

    /// Return coherence ratio between two times t1 and t2 using the T2 for `qubit`.
    /// Equivalent to exp(-(t2 - t1) / T2).
    #[pyfunction]
    fn coherence_ratio_py(
        n_qubits: usize,
        temperature: f64,
        qubit: usize,
        t1: f64,
        t2: f64,
    ) -> PyResult<f64> {
        let sys = QuantumSystem::new(n_qubits, temperature);
        Ok(sys.coherence_ratio(qubit, t1, t2))
    }

    /// Create a GHZ on a fresh system and return its purity (should be ~1.0).
    #[pyfunction]
    fn purity_of_ghz_py(n_qubits: usize, temperature: f64) -> PyResult<f64> {
        let sys = QuantumSystem::new(n_qubits, temperature);
        Ok(sys.purity_of_ghz())
    }

    #[pymodule]
    fn quantum_coherence(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(coherence_at_time_py, m)?)?;
        m.add_function(wrap_pyfunction!(coherence_ratio_py, m)?)?;
        m.add_function(wrap_pyfunction!(purity_of_ghz_py, m)?)?;
        Ok(())
    }
}
