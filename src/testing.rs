//! Testing module for quantum coherence
use crate::quantum::QuantumSystem;
use crate::noise::NoiseModel;
use crate::coherence::{CoherenceTestResults, CoherenceMeasurement, GHZMeasurement};
use anyhow::Context;
use std::time::Instant;

/// Main coherence testing function
pub fn test_superposition_preservation(
    system: &mut QuantumSystem,
    noise_models: Vec<NoiseModel>,
    num_tests: usize,
) -> anyhow::Result<CoherenceTestResults> {
    let start_time = Instant::now();

    // Set up noise models
    system.noise_models = noise_models;

    // Initialize results
    let mut results = CoherenceTestResults::new(system.n_qubits);
    
    // Run tests
    for test_idx in 0..num_tests {
        // Create superposition on each qubit
        for qubit in 0..system.n_qubits {
            system.create_superposition(qubit, std::f64::consts::PI/2.0, 0.0)
                .context("Failed to create superposition")?;

            let initial_fidelity = system.measure_superposition_fidelity(qubit)
                .context("Failed to measure initial fidelity")?;

            // Apply noise for various durations
            let mut fidelity_over_time = Vec::new();
            let mut purity_over_time = Vec::new();
            let mut entropy_over_time = Vec::new();

            for time_step in 0..10 {
                let duration = (time_step as f64) * system.t2_times[qubit] / 10.0;

                let mut test_system = system.clone();
                test_system.apply_noise(duration)
                    .context("Failed to apply noise")?;

                let fidelity = test_system.measure_superposition_fidelity(qubit)
                    .context("Failed to measure fidelity after noise")?;
                let purity = test_system.calculate_purity();
                let entropy = test_system.calculate_entropy();

                fidelity_over_time.push(fidelity);
                purity_over_time.push(purity);
                entropy_over_time.push(entropy);
            }

            let t2_estimate = fit_exponential_decay(&fidelity_over_time, system.t2_times[qubit]);

            results.add_measurement(
                qubit,
                test_idx,
                CoherenceMeasurement {
                    qubit_index: qubit,
                    test_index: test_idx,
                    initial_fidelity,
                    fidelity_over_time,
                    purity_over_time,
                    entropy_over_time,
                    t2_estimate,
                    timestamp: chrono::Utc::now(),
                },
            );
        }

        // Test GHZ state preservation
        if test_idx % 10 == 0 {
            test_ghz_preservation(system, &mut results, test_idx)?;
        }
    }

    // Calculate statistics
    results.calculate_statistics();
    results.duration = start_time.elapsed();

    Ok(results)
}

/// Fit exponential decay to fidelity data
fn fit_exponential_decay(fidelity_data: &[f64], initial_guess: f64) -> f64 {
    if fidelity_data.len() < 2 {
        return initial_guess;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;

    let n = fidelity_data.len() as f64;

    for (i, &fidelity) in fidelity_data.iter().enumerate() {
        let x = i as f64;
        let y = if fidelity > 1e-12 {
            fidelity.ln()
        } else {
            -30.0
        };

        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_xx += x * x;
    }

    let b = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
    
    if b < -1e-12 {
        -1.0 / b
    } else {
        initial_guess
    }
}

/// Test GHZ state preservation
fn test_ghz_preservation(
    system: &mut QuantumSystem,
    results: &mut CoherenceTestResults,
    test_idx: usize,
) -> anyhow::Result<()> {
    let original_state = system.density_matrix.clone();

    system.create_ghz_state()
        .context("Failed to create GHZ state")?;

    let ghz_fidelity = measure_ghz_fidelity(system)
        .context("Failed to measure GHZ fidelity")?;

    let mut fidelities = Vec::new();

    for time_step in 0..5 {
        let duration = (time_step as f64) * system.t2_times[0] / 5.0;

        let mut test_system = system.clone();
        test_system.apply_noise(duration)
            .context("Failed to apply noise to GHZ state")?;

        let fidelity = measure_ghz_fidelity(&test_system)
            .context("Failed to measure GHZ fidelity after noise")?;
        fidelities.push(fidelity);
    }

    results.ghz_measurements.push(GHZMeasurement {
        test_index: test_idx,
        initial_fidelity: ghz_fidelity,
        fidelity_over_time: fidelities,
        n_qubits: system.n_qubits,
        timestamp: chrono::Utc::now(),
    });

    system.density_matrix = original_state;
    
    Ok(())
}

/// Measure GHZ state fidelity
fn measure_ghz_fidelity(system: &QuantumSystem) -> anyhow::Result<f64> {
    let dim = 1 << system.n_qubits;

    let mut ideal_ghz = nalgebra::DMatrix::zeros(dim, dim);
    let amplitude = num_complex::Complex64::new(1.0 / (2.0_f64).sqrt(), 0.0);

    ideal_ghz[(0, 0)] = amplitude * amplitude.conj();
    ideal_ghz[(dim-1, dim-1)] = amplitude * amplitude.conj();
    ideal_ghz[(0, dim-1)] = amplitude * amplitude.conj();
    ideal_ghz[(dim-1, 0)] = amplitude * amplitude.conj();

    let product = &ideal_ghz * &system.density_matrix;
    let fidelity = product.trace().re;

    Ok(fidelity)
}
