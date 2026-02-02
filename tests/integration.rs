//! Integration tests for quantum coherence framework
use quantum_coherence::{QuantumSystem, NoiseModel, test_superposition_preservation};
use std::time::Duration;

#[test]
fn test_superposition_creation() {
    let mut system = QuantumSystem::new(3, 0.02);

    // Create superposition on qubit 0
    assert!(system.create_superposition(0, std::f64::consts::PI/2.0, 0.0).is_ok());

    // Measure fidelity
    let fidelity = system.measure_superposition_fidelity(0);
    assert!(fidelity.is_ok());
    let fidelity_value = fidelity.unwrap();

    // Initial fidelity should be close to 1.0
    assert!(fidelity_value > 0.99);
}

#[test]
fn test_noise_application() {
    let mut system = QuantumSystem::new(2, 0.02);

    // Create superposition
    assert!(system.create_superposition(0, std::f64::consts::PI/2.0, 0.0).is_ok());

    let initial_fidelity = system.measure_superposition_fidelity(0).unwrap();

    // Apply noise
    system.noise_models = vec![NoiseModel::thermal(0.02)];
    assert!(system.apply_noise(10.0).is_ok()); // 10 μs

    let final_fidelity = system.measure_superposition_fidelity(0).unwrap();

    // Fidelity should decrease after noise
    assert!(final_fidelity < initial_fidelity);
    // But not drop to zero immediately
    assert!(final_fidelity > 0.5);
}

#[test]
fn test_coherence_test_function() {
    let mut system = QuantumSystem::new(3, 0.02);
    let noise_models = vec![NoiseModel::comprehensive()];

    let results = test_superposition_preservation(&mut system, noise_models, 10);
    assert!(results.is_ok());

    let results = results.unwrap();
    assert_eq!(results.n_qubits, 3);
    assert!(!results.qubit_measurements.is_empty());

    // Check that statistics were calculated
    assert!(!results.statistics.is_empty());
    assert!(results.overall_stats.total_measurements > 0);
}

#[test]
fn test_results_serialization() {
    let mut system = QuantumSystem::new(2, 0.02);
    let noise_models = vec![NoiseModel::thermal(0.02)];

    let results = test_superposition_preservation(&mut system, noise_models, 5).unwrap();

    // Test JSON serialization
    let json_result = results.to_json();
    assert!(json_result.is_ok());

    let json_str = json_result.unwrap();
    assert!(!json_str.is_empty());
    assert!(json_str.contains("n_qubits"));
    assert!(json_str.contains("qubit_measurements"));

    // Test CSV serialization
    let csv_result = results.to_csv();
    assert!(!csv_result.is_empty());
    assert!(csv_result.contains("qubit,test_index,initial_fidelity,t2_estimate"));
}

#[test]
fn test_ghz_state() {
    let mut system = QuantumSystem::new(3, 0.02);
    
    // Create GHZ state
    assert!(system.create_ghz_state().is_ok());

    // Calculate purity
    let purity = system.calculate_purity();

    // GHZ state should have purity 1.0 (pure state)
    // But with numerical errors, allow some tolerance
    assert!(purity > 0.99);
}

#[test]
fn test_quantum_fisher_information() {
    let system = QuantumSystem::new(2, 0.02);

    let fisher_info = system.calculate_quantum_fisher_information();

    // Fisher information should be non-negative
    assert!(fisher_info >= 0.0);

    // For a 2-qubit system, maximum QFI is 4N² = 16
    // But actual value depends on state
    assert!(fisher_info <= 20.0); // Allow some margin
}

#[test]
fn test_temperature_dependence() {
    // Test that coherence times decrease with temperature
    let mut system_cold = QuantumSystem::new(2, 0.01); // 10 mK
    let mut system_warm = QuantumSystem::new(2, 0.1); // 100 mK

    // T2 should be longer at lower temperature
    let avg_t2_cold: f64 = system_cold.t2_times.iter().sum::<f64>() / 2.0;
    let avg_t2_warm: f64 = system_warm.t2_times.iter().sum::<f64>() / 2.0;

    assert!(avg_t2_cold > avg_t2_warm);
}

#[test]
fn test_noise_model_scaling() {
    let mut noise = NoiseModel::thermal_with_rates(0.02, vec![1.0, 2.0, 3.0]);
    let initial_rate = noise.total_rate();

    noise.scale(2.0);
    let scaled_rate = noise.total_rate();

    // Rate should double
    assert!((scaled_rate - 2.0 * initial_rate).abs() < 1e-6);
}

#[test]
fn test_performance_benchmark() {
    // Quick performance test - should complete in reasonable time
    let start_time = std::time::Instant::now();

    let mut system = QuantumSystem::new(5, 0.02);
    let noise_models = vec![NoiseModel::comprehensive()];

    let results = test_superposition_preservation(&mut system, noise_models, 100);
    assert!(results.is_ok());

    let duration = start_time.elapsed();

    // Should complete in under 2 seconds
    assert!(duration < Duration::from_secs(2));
}

#[test]
fn test_edge_cases() {
    // Test with 1 qubit
    let mut system_single = QuantumSystem::new(1, 0.02);
    let results_single = test_superposition_preservation(
        &mut system_single,
        vec![NoiseModel::thermal(0.02)],
        10
    );
    assert!(results_single.is_ok());

    // Test with many qubits (but limit tests for speed)
    let mut system_many = QuantumSystem::new(10, 0.02);
    let results_many = test_superposition_preservation(
        &mut system_many,
        vec![NoiseModel::thermal(0.02)],
        5, // Fewer tests for speed
    );
    assert!(results_many.is_ok());

    // Test with very high temperature
    let mut system_hot = QuantumSystem::new(2, 1.0); // 1 K
    let results_hot = test_superposition_preservation(
        &mut system_hot,
        vec![NoiseModel::thermal(1.0)],
        10,
    );
    assert!(results_hot.is_ok());

    let results = results_hot.unwrap();
    // At high temperature, fidelity should be lower
    assert!(results.overall_stats.average_fidelity < 0.9);
}
