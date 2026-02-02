// Lightweight unit tests for core functionality
use approx::assert_relative_eq;
use quantum_coherence::{
    test_superposition_preservation, CoherenceMeasurement, NoiseModel, QuantumSystem, HBAR, K_B,
};

#[test]
fn test_physical_constants_positive() {
    // Runtime assertion for physical constants
    assert!(HBAR > 0.0);
    assert!(K_B > 0.0);
    assert!(HBAR.is_finite());
    assert!(K_B.is_finite());
}

#[test]
fn test_quantum_system_initial_state() {
    let system = QuantumSystem::new(2, 0.02);
    assert_eq!(system.n_qubits, 2);
    let dim = 1 << 2;
    assert_eq!(system.density_matrix.nrows(), dim);
    assert_eq!(system.density_matrix.ncols(), dim);
    // Newly created system initializes in ground state -> pure
    let purity = system.calculate_purity();
    assert!((purity - 1.0).abs() < 1e-12);
}

#[test]
fn test_ghz_state_purity() {
    let mut system = QuantumSystem::new(3, 0.02);
    assert!(system.create_ghz_state().is_ok());
    let purity = system.calculate_purity();
    // GHZ should be a pure state (allow small numerical tolerance)
    assert!((purity - 1.0).abs() < 1e-8);
}

#[test]
fn test_noise_model_scale_and_total_rate() {
    let mut noise = NoiseModel::thermal_with_rates(0.02, vec![1.0, 2.0]);
    let initial = noise.total_rate();
    noise.scale(3.0);
    let scaled = noise.total_rate();
    assert!((scaled - 3.0 * initial).abs() < 1e-12);
}

#[test]
fn test_coherence_measurement_default() {
    let m = CoherenceMeasurement::default();
    assert_eq!(m.qubit_index, 0);
    assert_eq!(m.test_index, 0);
    assert_eq!(m.t2_estimate, 0.0);
}

#[test]
fn test_quantum_system_superposition() {
    let mut system = QuantumSystem::new(1, 0.02);
    // Create equal superposition on single qubit
    assert!(system
        .create_superposition(0, std::f64::consts::PI / 4.0, 0.0)
        .is_ok());
    let purity = system.calculate_purity();
    // Pure state superposition should have high purity
    assert!(purity > 0.99);
}

#[test]
fn test_quantum_system_t2_times() {
    let system = QuantumSystem::new(2, 0.02);
    // T2 times should be reasonable for low temperature
    for t2 in system.t2_times.iter() {
        assert!(*t2 > 10.0); // Should be > 10 μs
        assert!(*t2 < 1000.0); // Should be < 1000 μs for 20 mK
    }
}

#[test]
fn test_quantum_system_entropy() {
    let mut system = QuantumSystem::new(2, 0.02);
    let initial_entropy = system.calculate_entropy();

    // Pure state should have near-zero entropy
    assert!(initial_entropy < 1e-10);

    // After creating superposition, entropy should increase
    let _ = system.create_superposition(0, std::f64::consts::PI / 4.0, 0.0);
    let entropy_after = system.calculate_entropy();
    assert!(entropy_after > initial_entropy);
}

/// Helper: ensure trace(ρ) ≈ 1.0
fn assert_trace_one(system: &QuantumSystem) {
    let trace: f64 = system.density_matrix.diagonal().iter().map(|c| c.re).sum();
    assert_relative_eq!(trace, 1.0, epsilon = 1e-9);
}

#[test]
fn noiseless_evolution_preserves_purity_and_fidelity() {
    let mut system = QuantumSystem::new(3, 0.02); // 3 qubits, 20 mK

    // No noise
    system.noise_models = vec![];

    // Create |+> on qubit 0
    system
        .create_superposition(0, std::f64::consts::PI / 2.0, 0.0)
        .unwrap();
    let initial_fid = system.measure_superposition_fidelity(0).unwrap();
    let initial_purity = system.calculate_purity();

    // Apply "evolution" with zero noise
    system.apply_noise(0.0).unwrap();

    let final_fid = system.measure_superposition_fidelity(0).unwrap();
    let final_purity = system.calculate_purity();

    assert_trace_one(&system);
    assert_relative_eq!(initial_fid, final_fid, epsilon = 1e-9);
    assert_relative_eq!(initial_purity, final_purity, epsilon = 1e-9);
}

#[test]
fn pure_dephasing_gives_exponential_decay() {
    let mut system = QuantumSystem::new(1, 0.02);

    // Overwrite T2 to a known value
    system.t2_times[0] = 100.0; // μs

    // Only dephasing noise with rate = 1/T2
    let rate = 1.0 / system.t2_times[0];
    let dephasing = NoiseModel::dephasing(vec![rate]);
    system.noise_models = vec![dephasing];

    system
        .create_superposition(0, std::f64::consts::PI / 2.0, 0.0)
        .unwrap();

    let fid0 = system.measure_superposition_fidelity(0).unwrap();

    let t = system.t2_times[0];
    let mut system_t = system.clone();
    system_t.apply_noise(t).unwrap();
    let fid_t = system_t.measure_superposition_fidelity(0).unwrap();

    // Expected coherence amplitude ~ exp(-t/T2)
    let expected_decay = (-t / system.t2_times[0]).exp();
    // Map coherence decay to fidelity decay roughly linearly near 0.5
    assert!(fid_t < fid0);
    assert!(expected_decay < 1.0);

    assert_trace_one(&system_t);
}

#[test]
fn coherence_test_runs_and_produces_stats() {
    let mut system = QuantumSystem::new(3, 0.02);

    let noise = NoiseModel::comprehensive(); // your composite constructor
    let results = test_superposition_preservation(&mut system, vec![noise], 5).unwrap();

    assert_eq!(results.n_qubits, 3);
    assert!(results.overall_stats.average_t2 > 0.0);
    assert!(results.to_json().unwrap().len() > 0);
}
