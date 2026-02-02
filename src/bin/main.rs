//! Main binary for quantum coherence testing
use clap::{Parser, Subcommand};
use quantum_coherence::{test_superposition_preservation, NoiseModel, QuantumSystem};
use std::fs;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "info")]
    log_level: String,

    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run local coherence test
    Run {
        #[arg(short = 'n', long, default_value_t = 5)]
        qubits: usize,

        #[arg(short = 'T', long, default_value_t = 0.02)]
        temperature: f64,

        #[arg(short = 't', long, default_value_t = 100)]
        tests: usize,

        #[arg(long, default_value_t = false)]
        comprehensive_noise: bool,
    },

    /// Benchmark different configurations
    Benchmark {
        #[arg(short = 'i', long, default_value_t = 1)]
        min_qubits: usize,

        #[arg(short = 'a', long, default_value_t = 10)]
        max_qubits: usize,

        #[arg(short, long, default_value_t = 1)]
        step: usize,
    },

    /// Generate example files
    GenerateBindings,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Set up logging
    env_logger::Builder::new()
        .filter_level(match cli.log_level.as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        })
        .init();

    match cli.command {
        Commands::Run {
            qubits,
            temperature,
            tests,
            comprehensive_noise,
        } => {
            run_local_test(qubits, temperature, tests, comprehensive_noise, cli.output)?;
        }
        Commands::Benchmark {
            min_qubits,
            max_qubits,
            step,
        } => {
            run_benchmark(min_qubits, max_qubits, step, cli.output)?;
        }
        Commands::GenerateBindings => {
            generate_example_files()?;
        }
    }

    Ok(())
}

fn run_local_test(
    n_qubits: usize,
    temperature: f64,
    num_tests: usize,
    comprehensive_noise: bool,
    output: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running local coherence test...");
    println!(" Qubits: {}", n_qubits);
    println!(
        " Temperature: {} K ({} mK)",
        temperature,
        temperature * 1000.0
    );
    println!(" Tests: {}", num_tests);

    let start_time = Instant::now();

    let mut system = QuantumSystem::new(n_qubits, temperature);

    let noise_models = if comprehensive_noise {
        vec![NoiseModel::comprehensive()]
    } else {
        vec![
            NoiseModel::thermal(temperature),
            NoiseModel::dephasing(vec![]),
        ]
    };

    let results = test_superposition_preservation(&mut system, noise_models, num_tests)?;
    let duration = start_time.elapsed();

    print_results(&results, duration);

    if let Some(output_path) = output {
        save_results(&results, &output_path)?;
    }

    Ok(())
}

fn run_benchmark(
    min_qubits: usize,
    max_qubits: usize,
    step: usize,
    output: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Running benchmark from {} to {} qubits (step: {})",
        min_qubits, max_qubits, step
    );
    println!("{:-<60}", "");
    println!(
        "{:>10} {:>12} {:>12} {:>12} {:>12}",
        "Qubits", "Time (ms)", "Avg T2 (μs)", "Min T2 (μs)", "Fidelity"
    );
    println!("{:-<60}", "");

    let mut benchmark_results = Vec::new();

    for n_qubits in (min_qubits..=max_qubits).step_by(step) {
        let start_time = Instant::now();

        let mut system = QuantumSystem::new(n_qubits, 0.02);
        let noise_models = vec![NoiseModel::comprehensive()];

        let results = test_superposition_preservation(&mut system, noise_models, 100)?;
        let duration = start_time.elapsed();

        let avg_t2 = results.overall_stats.average_t2;
        let system_t2 = results.overall_stats.system_t2;
        let avg_fidelity = results.overall_stats.average_fidelity;

        println!(
            "{:10} {:12.2} {:12.1} {:12.1} {:12.4}",
            n_qubits,
            duration.as_millis(),
            avg_t2,
            system_t2,
            avg_fidelity
        );

        benchmark_results.push((n_qubits, duration, avg_t2, system_t2, avg_fidelity));
    }

    if let Some(output_path) = output {
        let csv_path = if output_path.ends_with(".csv") {
            output_path.clone()
        } else {
            format!("{}.csv", output_path)
        };

        let mut csv = String::new();
        csv.push_str("qubits,duration_ms,avg_t2_us,system_t2_us,avg_fidelity\n");

        for (n_qubits, duration, avg_t2, system_t2, avg_fidelity) in benchmark_results {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                n_qubits,
                duration.as_millis(),
                avg_t2,
                system_t2,
                avg_fidelity
            ));
        }

        fs::write(csv_path, csv)?;
        println!("\nBenchmark results saved to CSV file.");
    }

    Ok(())
}

fn generate_example_files() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating example files...");

    fs::create_dir_all("examples")?;

    let example = r#"//! Example usage of quantum coherence testing framework
use quantum_coherence::{QuantumSystem, NoiseModel, test_superposition_preservation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Quantum Coherence Testing - Example");
    
    // Create a 5-qubit system at 20 mK
    let mut system = QuantumSystem::new(5, 0.02);
    
    // Define noise models
    let noise_models = vec![
        NoiseModel::thermal(0.02),
        NoiseModel::dephasing(vec![]),
    ];
    
    // Run coherence test with 100 trials
    let results = test_superposition_preservation(&mut system, noise_models, 100)?;
    
    // Display results
    println!("\nCoherence Test Results:");
    println!("  Average T₂: {:.1} μs", results.overall_stats.average_t2);
    println!("  System T₂: {:.1} μs", results.overall_stats.system_t2);
    println!("  Average Fidelity: {:.6}", results.overall_stats.average_fidelity);
    println!("  GHZ Fidelity: {:.6}", results.overall_stats.average_ghz_fidelity);
    
    Ok(())
}
"#;

    fs::write("examples/basic_test.rs", example)?;
    println!("Example file created: examples/basic_test.rs");

    Ok(())
}

fn print_results(results: &quantum_coherence::CoherenceTestResults, duration: std::time::Duration) {
    println!("\n=== COHERENCE TEST RESULTS ===");
    println!("Test Duration: {:.2?}", duration);
    println!("Number of Qubits: {}", results.n_qubits);
    println!(
        "Total Measurements: {}",
        results.overall_stats.total_measurements
    );
    println!("\nOverall Statistics:");
    println!(" Average T₂: {:.1} μs", results.overall_stats.average_t2);
    println!(
        " System T₂ (worst): {:.1} μs",
        results.overall_stats.system_t2
    );
    println!(
        " Average Fidelity: {:.6}",
        results.overall_stats.average_fidelity
    );
    println!(
        " Average GHZ Fidelity: {:.6}",
        results.overall_stats.average_ghz_fidelity
    );

    println!("\nPer-Qubit Statistics:");
    println!("{:-<60}", "");
    println!(
        "{:>6} {:>10} {:>10} {:>10} {:>10}",
        "Qubit", "Avg T₂", "Min T₂", "Max T₂", "Fidelity"
    );
    println!("{:-<60}", "");

    for qubit in 0..results.n_qubits {
        if let Some(stats) = results.statistics.get(&qubit) {
            println!(
                "{:6} {:10.1} {:10.1} {:10.1} {:10.6}",
                qubit, stats.average_t2, stats.min_t2, stats.max_t2, stats.average_fidelity
            );
        }
    }

    if let Some((worst_qubit, worst_t2)) = results.get_worst_t2() {
        println!("\nWorst Qubit: #{} (T₂ = {:.1} μs)", worst_qubit, worst_t2);
    }

    if let Some((best_qubit, best_t2)) = results.get_best_t2() {
        println!("Best Qubit: #{} (T₂ = {:.1} μs)", best_qubit, best_t2);
    }

    let avg_fidelity = results.overall_stats.average_fidelity;
    println!("\nSuperposition Preservation:");
    if avg_fidelity > 0.95 {
        println!("✓ Superposition well preserved!");
    } else if avg_fidelity > 0.90 {
        println!(" Superposition moderately preserved");
    } else {
        println!("✗ Superposition significantly degraded");
    }
}

fn save_results(
    results: &quantum_coherence::CoherenceTestResults,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = results.to_json()?;
    let json_path = if output_path.ends_with(".json") {
        output_path.to_string()
    } else {
        format!("{}.json", output_path)
    };
    fs::write(&json_path, json)?;
    println!("Results saved to JSON: {}", json_path);

    let csv = results.to_csv();
    let csv_path = if output_path.ends_with(".csv") {
        output_path.to_string()
    } else {
        format!("{}.csv", output_path)
    };
    fs::write(&csv_path, csv)?;
    println!("Results saved to CSV: {}", csv_path);

    Ok(())
}
