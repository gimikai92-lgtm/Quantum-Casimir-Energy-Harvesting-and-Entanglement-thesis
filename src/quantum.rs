//! Core quantum system implementations
use crate::coherence::CoherenceError;
use crate::noise::NoiseModel;
use nalgebra::DMatrix;
use ndarray::Array2;
use num_complex::Complex64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Normal, Uniform};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Reduced Planck constant (J·s)
pub const HBAR: f64 = 1.054571817e-34;
/// Boltzmann constant (J/K)
pub const K_B: f64 = 1.380649e-23;
/// Planck constant (J·s)
pub const PLANCK: f64 = 6.62607015e-34;
/// Elementary charge (C)
pub const E_CHARGE: f64 = 1.602176634e-19;
/// Vacuum permittivity (F/m)
pub const EPSILON_0: f64 = 8.854187817e-12;
/// Vacuum permeability (H/m)
pub const MU_0: f64 = 1.25663706212e-6;
/// Speed of light (m/s)
pub const C_LIGHT: f64 = 299792458.0;

/// Main quantum system structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSystem {
    /// Number of qubits in the system
    pub n_qubits: usize,
    /// System temperature in Kelvin
    pub temperature: f64,
    /// Qubit transition frequencies in GHz
    pub qubit_frequencies: Vec<f64>,
    /// Qubit anharmonicities in MHz
    pub anharmonicities: Vec<f64>,
    /// Coupling strengths between qubits in MHz
    pub couplings: Array2<f64>,
    /// Density matrix representing quantum state
    pub density_matrix: DMatrix<Complex64>,
    /// T1 relaxation times per qubit in microseconds
    pub t1_times: Vec<f64>,
    /// T2 dephasing times per qubit in microseconds
    pub t2_times: Vec<f64>,
    /// System Hamiltonian matrix
    pub hamiltonian: DMatrix<Complex64>,
    /// Active noise models for simulation
    pub noise_models: Vec<NoiseModel>,
}

impl QuantumSystem {
    /// Create a new quantum system
    pub fn new(n_qubits: usize, temperature: f64) -> Self {
        let mut rng = StdRng::from_entropy();

        let freq_dist = Uniform::new(4.0, 6.0);
        let qubit_frequencies: Vec<f64> = (0..n_qubits).map(|_| rng.sample(&freq_dist)).collect();

        let anharm_dist = Uniform::new(-350.0, -200.0);
        let anharmonicities: Vec<f64> = (0..n_qubits).map(|_| rng.sample(&anharm_dist)).collect();

        let mut couplings = Array2::zeros((n_qubits, n_qubits));
        let coupling_dist = Uniform::new(5.0, 50.0);

        for i in 0..n_qubits {
            for j in (i + 1)..n_qubits.min(i + 3) {
                let coupling = rng.sample(&coupling_dist);
                couplings[[i, j]] = coupling;
                couplings[[j, i]] = coupling;
            }
        }

        let base_t1 = 120.0;
        let base_t2 = 85.0;
        let temp_factor = (temperature * 1000.0).powf(-0.7);

        let t1_times: Vec<f64> = (0..n_qubits)
            .map(|_| base_t1 * temp_factor * rng.gen_range(0.8..1.2))
            .collect();

        let t2_times: Vec<f64> = (0..n_qubits)
            .map(|_| base_t2 * temp_factor * rng.gen_range(0.8..1.2))
            .collect();

        let dim = 1 << n_qubits;
        let mut density_matrix = DMatrix::zeros(dim, dim);
        density_matrix[(0, 0)] = Complex64::new(1.0, 0.0);

        let hamiltonian =
            Self::create_hamiltonian(&qubit_frequencies, &anharmonicities, &couplings, n_qubits);

        QuantumSystem {
            n_qubits,
            temperature,
            qubit_frequencies,
            anharmonicities,
            couplings,
            density_matrix,
            t1_times,
            t2_times,
            hamiltonian,
            noise_models: vec![NoiseModel::thermal(temperature)],
        }
    }

    /// Create system Hamiltonian
    fn create_hamiltonian(
        frequencies: &[f64],
        anharmonicities: &[f64],
        couplings: &Array2<f64>,
        n_qubits: usize,
    ) -> DMatrix<Complex64> {
        let dim = 1 << n_qubits;
        let mut hamiltonian = DMatrix::zeros(dim, dim);

        for i in 0..n_qubits {
            let freq = frequencies[i] * 1e9 * 2.0 * PI;
            let _anharm = anharmonicities[i] * 1e6 * 2.0 * PI;

            for state in 0..dim {
                let bit_i = (state >> i) & 1;
                if bit_i == 1 {
                    hamiltonian[(state, state)] += Complex64::new(freq, 0.0);
                }
            }
        }

        for i in 0..n_qubits {
            for j in (i + 1)..n_qubits {
                let coupling = couplings[[i, j]] * 1e6 * 2.0 * PI;

                if coupling.abs() > 1e-9 {
                    for state in 0..dim {
                        let _bit_i = (state >> i) & 1;
                        let _bit_j = (state >> j) & 1;
                        let new_state = state ^ (1 << i) ^ (1 << j);

                        hamiltonian[(state, new_state)] += Complex64::new(coupling / 2.0, 0.0);
                        hamiltonian[(new_state, state)] += Complex64::new(coupling / 2.0, 0.0);
                    }
                }
            }
        }

        hamiltonian
    }

    /// Create superposition state on specific qubit
    pub fn create_superposition(
        &mut self,
        qubit: usize,
        theta: f64,
        phi: f64,
    ) -> Result<(), CoherenceError> {
        if qubit >= self.n_qubits {
            return Err(CoherenceError::QuantumError(format!(
                "Qubit index {} out of bounds",
                qubit
            )));
        }

        let dim = 1 << self.n_qubits;
        let cos_half = (theta / 2.0).cos();
        let sin_half = (theta / 2.0).sin();
        let phase = Complex64::new(phi.cos(), phi.sin());

        let mut new_density = DMatrix::zeros(dim, dim);

        for state1 in 0..dim {
            for state2 in 0..dim {
                let mut amplitude = Complex64::new(0.0, 0.0);

                for basis1 in 0..2 {
                    for basis2 in 0..2 {
                        let coeff1 = if basis1 == 0 {
                            cos_half
                        } else {
                            sin_half * phase.re
                        };
                        let coeff1_imag = if basis1 == 0 {
                            0.0
                        } else {
                            sin_half * phase.im
                        };

                        let coeff2 = if basis2 == 0 {
                            cos_half
                        } else {
                            sin_half * phase.re
                        };
                        let coeff2_imag = if basis2 == 0 {
                            0.0
                        } else {
                            sin_half * phase.im
                        };

                        let basis_state1 = if basis1 == 0 {
                            state1 & !(1 << qubit)
                        } else {
                            state1 | (1 << qubit)
                        };

                        let basis_state2 = if basis2 == 0 {
                            state2 & !(1 << qubit)
                        } else {
                            state2 | (1 << qubit)
                        };

                        amplitude += Complex64::new(coeff1, coeff1_imag).conj()
                            * self.density_matrix[(basis_state1, basis_state2)]
                            * Complex64::new(coeff2, coeff2_imag);
                    }
                }

                new_density[(state1, state2)] = amplitude;
            }
        }

        self.density_matrix = new_density;
        Ok(())
    }

    /// Create GHZ (Greenberger-Horne-Zeilinger) state
    pub fn create_ghz_state(&mut self) -> Result<(), CoherenceError> {
        let dim = 1 << self.n_qubits;
        let mut new_density = DMatrix::zeros(dim, dim);
        let amplitude = Complex64::new(1.0 / (2.0 as f64).sqrt(), 0.0);

        new_density[(0, 0)] = amplitude * amplitude.conj();
        new_density[(dim - 1, dim - 1)] = amplitude * amplitude.conj();
        new_density[(0, dim - 1)] = amplitude * amplitude.conj();
        new_density[(dim - 1, 0)] = amplitude * amplitude.conj();

        self.density_matrix = new_density;
        Ok(())
    }

    /// Measure superposition fidelity
    pub fn measure_superposition_fidelity(&self, qubit: usize) -> Result<f64, CoherenceError> {
        if qubit >= self.n_qubits {
            return Err(CoherenceError::QuantumError(
                "Qubit index out of bounds".to_string(),
            ));
        }

        let dim = 1 << self.n_qubits;
        let mut p0 = 0.0;
        let mut p1 = 0.0;

        for state in 0..dim {
            let bit = (state >> qubit) & 1;
            let prob = self.density_matrix[(state, state)].re;

            if bit == 0 {
                p0 += prob;
            } else {
                p1 += prob;
            }
        }

        let ideal_p0 = 0.5;
        let ideal_p1 = 0.5;
        let fidelity = ((p0 * ideal_p0).sqrt() + (p1 * ideal_p1).sqrt()).powi(2);

        Ok(fidelity)
    }

    /// Apply noise to the system
    pub fn apply_noise(&mut self, duration: f64) -> Result<(), CoherenceError> {
        let total_noise_rate: f64 = self
            .noise_models
            .iter()
            .map(|model| model.total_rate())
            .sum();

        if total_noise_rate < 1e-12 {
            return Ok(());
        }

        let noise_models_to_apply = self.noise_models.clone();
        for noise_model in &noise_models_to_apply {
            self.apply_specific_noise(noise_model, duration)?;
        }

        self.renormalize();
        Ok(())
    }

    /// Apply specific noise model
    fn apply_specific_noise(
        &mut self,
        noise_model: &NoiseModel,
        duration: f64,
    ) -> Result<(), CoherenceError> {
        match noise_model {
            NoiseModel::Thermal { temperature, rates } => {
                self.apply_thermal_noise(*temperature, rates, duration)
            }
            NoiseModel::Dephasing { rates } => self.apply_dephasing_noise(rates, duration),
            NoiseModel::AmplitudeDamping { rates } => {
                self.apply_amplitude_damping_noise(rates, duration)
            }
            NoiseModel::OneOverF { strength, exponent } => {
                self.apply_one_over_f_noise(*strength, *exponent, duration)
            }
            NoiseModel::Composite { models } => {
                for model in models {
                    self.apply_specific_noise(model, duration)?;
                }
                Ok(())
            }
        }
    }

    /// Apply thermal (T1) noise
    fn apply_thermal_noise(
        &mut self,
        temperature: f64,
        rates: &[f64],
        duration: f64,
    ) -> Result<(), CoherenceError> {
        let dim = 1 << self.n_qubits;

        for qubit in 0..self.n_qubits {
            let rate = rates
                .get(qubit)
                .copied()
                .unwrap_or(1.0 / self.t1_times[qubit]);
            let omega = self.qubit_frequencies[qubit] * 1e9 * 2.0 * PI;
            let n_th = 1.0 / ((HBAR * omega) / (K_B * temperature)).exp() - 1.0;

            let p_excite = rate * n_th * duration;
            let p_relax = rate * (n_th + 1.0) * duration;

            for state in 0..dim {
                let bit = (state >> qubit) & 1;

                if bit == 0 {
                    let excited_state = state | (1 << qubit);
                    let transfer = p_excite.min(1.0);
                    let old_val = self.density_matrix[(state, state)];

                    self.density_matrix[(excited_state, excited_state)] += old_val * transfer;
                    self.density_matrix[(state, state)] *= 1.0 - transfer;
                } else {
                    let ground_state = state & !(1 << qubit);
                    let transfer = p_relax.min(1.0);
                    let old_val = self.density_matrix[(state, state)];

                    self.density_matrix[(ground_state, ground_state)] += old_val * transfer;
                    self.density_matrix[(state, state)] *= 1.0 - transfer;
                }
            }
        }

        Ok(())
    }

    /// Apply dephasing (T2) noise
    fn apply_dephasing_noise(
        &mut self,
        rates: &[f64],
        duration: f64,
    ) -> Result<(), CoherenceError> {
        let dim = 1 << self.n_qubits;

        for qubit in 0..self.n_qubits {
            let rate = rates
                .get(qubit)
                .copied()
                .unwrap_or(1.0 / self.t2_times[qubit] - 0.5 / self.t1_times[qubit]);

            let decay = (-rate * duration).exp();

            for i in 0..dim {
                for j in 0..dim {
                    if i != j {
                        let bit_i = (i >> qubit) & 1;
                        let bit_j = (j >> qubit) & 1;

                        if bit_i != bit_j {
                            self.density_matrix[(i, j)] *= decay;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Apply amplitude damping noise
    fn apply_amplitude_damping_noise(
        &mut self,
        rates: &[f64],
        duration: f64,
    ) -> Result<(), CoherenceError> {
        let dim = 1 << self.n_qubits;

        for qubit in 0..self.n_qubits {
            let rate = rates
                .get(qubit)
                .copied()
                .unwrap_or(1.0 / self.t1_times[qubit]);
            let p_decay = 1.0 - (-rate * duration).exp();

            for state in 0..dim {
                let bit = (state >> qubit) & 1;

                if bit == 1 {
                    let ground_state = state & !(1 << qubit);
                    let decay_amount = self.density_matrix[(state, state)].re * p_decay;

                    self.density_matrix[(ground_state, ground_state)] +=
                        Complex64::new(decay_amount, 0.0);
                    self.density_matrix[(state, state)] -= Complex64::new(decay_amount, 0.0);

                    for other in 0..dim {
                        if other != state {
                            let other_ground = other & !(1 << qubit);

                            if (other >> qubit) & 1 == 1 {
                                let decay_sqrt = p_decay.sqrt();
                                let old_val = self.density_matrix[(state, other)];
                                self.density_matrix[(ground_state, other_ground)] +=
                                    old_val * decay_sqrt;
                                self.density_matrix[(state, other)] *= 1.0 - decay_sqrt;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Apply 1/f noise
    fn apply_one_over_f_noise(
        &mut self,
        strength: f64,
        exponent: f64,
        duration: f64,
    ) -> Result<(), CoherenceError> {
        let mut rng = StdRng::from_entropy();
        let dim = 1 << self.n_qubits;

        for qubit in 0..self.n_qubits {
            let phase_variance = strength * duration.ln().max(1.0).powf(exponent);
            let phase_std = phase_variance.sqrt();
            let phase_dist = Normal::new(0.0, phase_std).unwrap();

            for i in 0..dim {
                for j in 0..dim {
                    if i != j {
                        let bit_i = (i >> qubit) & 1;
                        let bit_j = (j >> qubit) & 1;

                        if bit_i != bit_j {
                            let phase_shift: f64 = rng.sample(&phase_dist);
                            let phase_factor = Complex64::new(phase_shift.cos(), phase_shift.sin());
                            self.density_matrix[(i, j)] *= phase_factor;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Renormalize density matrix
    fn renormalize(&mut self) {
        let trace: f64 = self.density_matrix.diagonal().iter().map(|c| c.re).sum();

        if trace.abs() > 1e-12 {
            let norm = Complex64::new(1.0 / trace, 0.0);
            self.density_matrix *= norm;
        }
    }

    /// Calculate purity of the quantum state
    pub fn calculate_purity(&self) -> f64 {
        let product = &self.density_matrix * &self.density_matrix;
        product.trace().re
    }

    /// Calculate von Neumann entropy
    pub fn calculate_entropy(&self) -> f64 {
        let eigen = self.density_matrix.clone().symmetric_eigen();
        let mut entropy = 0.0;

        for &eigval in eigen.eigenvalues.iter() {
            if eigval > 1e-12 {
                entropy -= eigval * eigval.ln();
            }
        }

        entropy
    }

    /// Calculate quantum Fisher information for phase estimation
    pub fn calculate_quantum_fisher_information(&self) -> f64 {
        let eigen = self.density_matrix.clone().symmetric_eigen();
        let eigenvalues = eigen.eigenvalues;
        let eigenvectors = eigen.eigenvectors;

        let dim = eigenvalues.len();
        let mut fisher_info = 0.0;

        for i in 0..dim {
            for j in 0..dim {
                if eigenvalues[i] + eigenvalues[j] > 1e-12 {
                    let term = (eigenvalues[i] - eigenvalues[j]).powi(2)
                        / (eigenvalues[i] + eigenvalues[j]);

                    let matrix_element = eigenvectors.column(i).dot(&eigenvectors.column(j));
                    fisher_info += term * matrix_element.norm_sqr();
                }
            }
        }

        2.0 * fisher_info
    }
}
