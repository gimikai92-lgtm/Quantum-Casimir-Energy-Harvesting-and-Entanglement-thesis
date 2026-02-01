//! Minimal quantum coherence crate for tests
use nalgebra::DMatrix;
use num_complex::Complex64;
use anyhow::Result;

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
}

// Re-export types at crate root
// types are already public; no additional re-exports needed
