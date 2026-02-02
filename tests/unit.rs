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

#[test]
fn test_coherence_decay_exponential() {
    // Test exponential decay of coherence: C(t) = exp(-t / T2)
    let system = QuantumSystem::new(2, 0.02);
    
    // T2 should be approximately 85 microseconds
    let t2 = system.t2_times[0];
    assert!((t2 - 85.0).abs() < 0.1);
    
    // At t=0, coherence should be 1.0
    let c_0 = system.coherence_at_time(0, 0.0);
    assert!((c_0 - 1.0).abs() < 1e-12);
    
    // At t=T2, coherence should be exp(-1) ≈ 0.368
    let c_t2 = system.coherence_at_time(0, t2);
    assert!((c_t2 - (-1.0_f64).exp()).abs() < 1e-10);
    
    // At t=5*T2, coherence should be exp(-5) ≈ 0.0067
    let c_5t2 = system.coherence_at_time(0, 5.0 * t2);
    assert!((c_5t2 - (-5.0_f64).exp()).abs() < 1e-10);
}

#[test]
fn test_coherence_collapse_complete() {
    // Test that coherence eventually decays to near-zero
    let system = QuantumSystem::new(1, 0.02);
    let t2 = system.t2_times[0];
    
    // Very long time (100 * T2)
    let long_time = 100.0 * t2;
    let c_long = system.coherence_at_time(0, long_time);
    
    // Should be effectively zero
    assert!(c_long < 1e-40);
    assert!(c_long > 0.0); // Should still be positive (exponential never truly reaches zero)
}

#[test]
fn test_coherence_ratio_between_times() {
    // Test relative coherence loss between two time points
    let system = QuantumSystem::new(2, 0.02);
    
    let t1 = 0.0;
    let t2_time = 10.0;
    let t3 = 20.0;
    
    // Ratio from t1 to t2
    let ratio_1_to_2 = system.coherence_ratio(0, t1, t2_time);
    let c_t2 = system.coherence_at_time(0, t2_time);
    assert!((ratio_1_to_2 - c_t2).abs() < 1e-12);
    
    // Ratio from t2 to t3
    let ratio_2_to_3 = system.coherence_ratio(0, t2_time, t3);
    let expected_ratio = (-(t3 - t2_time) / system.t2_times[0]).exp();
    assert!((ratio_2_to_3 - expected_ratio).abs() < 1e-12);
}

#[test]
fn test_multi_qubit_coherence_independence() {
    // Test that different qubits can have independent coherence
    let mut system = QuantumSystem::new(3, 0.02);
    
    // Modify T2 times for different qubits
    system.t2_times[0] = 50.0;
    system.t2_times[1] = 85.0;
    system.t2_times[2] = 150.0;
    
    let t = 25.0;
    
    // Qubit 0 should have faster decay
    let c0 = system.coherence_at_time(0, t);
    // Qubit 1 should have medium decay
    let c1 = system.coherence_at_time(1, t);
    // Qubit 2 should have slower decay
    let c2 = system.coherence_at_time(2, t);
    
    // Longer T2 means less decay, so: c2 > c1 > c0
    assert!(c2 > c1);
    assert!(c1 > c0);
}
