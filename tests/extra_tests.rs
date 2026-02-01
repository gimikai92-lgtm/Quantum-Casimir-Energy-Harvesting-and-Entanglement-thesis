use quantum_coherence::QuantumSystem;

#[test]
fn test_density_matrix_trace_and_t2s() {
    let system = QuantumSystem::new(4, 0.02);
    // dimension should be 2^4 = 16
    assert_eq!(system.density_matrix.nrows(), 1 << 4);
    assert_eq!(system.density_matrix.ncols(), 1 << 4);
    // trace should be 1.0 for a valid density matrix
    let trace: f64 = system.density_matrix.diagonal().iter().map(|c| c.re).sum();
    assert!((trace - 1.0).abs() < 1e-12);
    // t2_times length matches qubits
    assert_eq!(system.t2_times.len(), 4);
}

#[test]
fn test_ghz_on_single_qubit_is_valid() {
    let mut system = QuantumSystem::new(1, 0.02);
    assert!(system.create_ghz_state().is_ok());
    // Purity of GHZ (pure state) should be ~1.0
    let purity = system.calculate_purity();
    assert!((purity - 1.0).abs() < 1e-8);
}
