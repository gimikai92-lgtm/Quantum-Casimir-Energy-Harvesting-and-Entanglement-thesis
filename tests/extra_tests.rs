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

#[test]
fn test_t2_t1_coherence_decay() {
    // For an exponential T2 decay model: coherence(t) = exp(-t / T2)
    let system = QuantumSystem::new(2, 0.02);
    let t2_time = system.t2_times[0];
    let t1 = 5.0;
    let t2 = 20.0;

    let coh1 = (-(t1) / t2_time).exp();
    let coh2 = (-(t2) / t2_time).exp();

    // coherence should decrease with time
    assert!(coh2 < coh1);

    // the ratio coh2/coh1 should equal exp(-(t2 - t1) / T2)
    let ratio = coh2 / coh1;
    let expected_ratio = (-(t2 - t1) / t2_time).exp();
    let tol = 1e-12;
    assert!(
        (ratio - expected_ratio).abs() < tol,
        "ratio {} != expected {}",
        ratio,
        expected_ratio
    );
}

#[test]
fn test_coherence_at_time_method_matches_formula() {
    let system = QuantumSystem::new(3, 0.01);
    let t = 7.5;
    let qubit = 1;
    let t2 = system.t2_times[qubit];

    let expected = (-(t) / t2).exp();
    let got = system.coherence_at_time(qubit, t);
    let tol = 1e-12;
    assert!(
        (got - expected).abs() < tol,
        "got {} expected {}",
        got,
        expected
    );
}
