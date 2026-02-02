// Lightweight unit tests for core functionality
use quantum_coherence::{CoherenceMeasurement, NoiseModel, QuantumSystem, HBAR, K_B};

#[test]
fn test_physical_constants_positive() {
    // Use const block to verify at compile-time (clippy: assertions_on_constants)
    const _: () = {
        assert!(HBAR > 0.0);
        assert!(K_B > 0.0);
    };
    // Runtime assertion kept for coverage
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
