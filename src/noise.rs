//! Noise models for quantum systems
use serde::{Deserialize, Serialize};

/// Types of quantum noise
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoiseModel {
    /// Thermal noise (T1 processes)
    Thermal {
        /// Temperature in Kelvin
        temperature: f64,
        /// Relaxation rates per qubit (Hz)
        rates: Vec<f64>,
    },
    /// Dephasing noise (T2 processes)
    Dephasing {
        /// Dephasing rates per qubit (Hz)
        rates: Vec<f64>,
    },
    /// Amplitude damping
    AmplitudeDamping {
        /// Damping rates per qubit (Hz)
        rates: Vec<f64>,
    },
    /// 1/f noise
    OneOverF {
        /// Noise strength coefficient
        strength: f64,
        /// Frequency exponent (typically 1.0)
        exponent: f64,
    },
    /// Composite noise model
    Composite {
        /// Collection of noise models to apply
        models: Vec<NoiseModel>,
    },
}

impl NoiseModel {
    /// Create thermal noise model
    pub fn thermal(temperature: f64) -> Self {
        NoiseModel::Thermal {
            temperature,
            rates: Vec::new(),
        }
    }

    /// Create thermal noise with specific rates
    pub fn thermal_with_rates(temperature: f64, rates: Vec<f64>) -> Self {
        NoiseModel::Thermal { temperature, rates }
    }

    /// Create dephasing noise model
    pub fn dephasing(rates: Vec<f64>) -> Self {
        NoiseModel::Dephasing { rates }
    }

    /// Create amplitude damping noise model
    pub fn amplitude_damping(rates: Vec<f64>) -> Self {
        NoiseModel::AmplitudeDamping { rates }
    }

    /// Create 1/f noise model
    pub fn one_over_f(strength: f64, exponent: f64) -> Self {
        NoiseModel::OneOverF { strength, exponent }
    }

    /// Create comprehensive noise model
    pub fn comprehensive() -> Self {
        NoiseModel::Composite {
            models: vec![
                NoiseModel::thermal(0.02),
                NoiseModel::dephasing(vec![]),
                NoiseModel::one_over_f(0.1, 1.0),
            ],
        }
    }

    /// Get total noise rate
    pub fn total_rate(&self) -> f64 {
        match self {
            NoiseModel::Thermal { rates, .. } => {
                if rates.is_empty() {
                    1.0
                } else {
                    rates.iter().sum::<f64>() / rates.len() as f64
                }
            }
            NoiseModel::Dephasing { rates } => {
                if rates.is_empty() {
                    0.5
                } else {
                    rates.iter().sum::<f64>() / rates.len() as f64
                }
            }
            NoiseModel::AmplitudeDamping { rates } => {
                if rates.is_empty() {
                    0.3
                } else {
                    rates.iter().sum::<f64>() / rates.len() as f64
                }
            }
            NoiseModel::OneOverF { strength, .. } => *strength,
            NoiseModel::Composite { models } => models.iter().map(|m| m.total_rate()).sum(),
        }
    }

    /// Scale noise rates by factor
    pub fn scale(&mut self, factor: f64) {
        match self {
            NoiseModel::Thermal { rates, .. } => {
                for rate in rates.iter_mut() {
                    *rate *= factor;
                }
            }
            NoiseModel::Dephasing { rates } => {
                for rate in rates.iter_mut() {
                    *rate *= factor;
                }
            }
            NoiseModel::AmplitudeDamping { rates } => {
                for rate in rates.iter_mut() {
                    *rate *= factor;
                }
            }
            NoiseModel::OneOverF { strength, .. } => {
                *strength *= factor;
            }
            NoiseModel::Composite { models } => {
                for model in models.iter_mut() {
                    model.scale(factor);
                }
            }
        }
    }
}
