use quantum_coherence::QuantumSystem;
use std::env;
use std::process::exit;

fn validate_required_secrets() {
    // If REQUIRE_SECRETS=1, fail fast when common secrets are missing
    let require = env::var("REQUIRE_SECRETS").unwrap_or_default();
    if require == "1" {
        let keys = [
            "SERVICE_API_KEY",
            "S3_ACCESS_KEY_ID",
            "S3_SECRET_ACCESS_KEY",
        ];
        let mut missing = Vec::new();
        for k in keys.iter() {
            if env::var(k).ok().filter(|v| !v.is_empty()).is_none() {
                missing.push(*k);
            }
        }
        if !missing.is_empty() {
            eprintln!("Missing required secrets: {:?}", missing);
            eprintln!("Set these in .env or CI secrets and retry.");
            exit(2);
        }
    }
}

fn masked_present(key: &str) -> bool {
    env::var(key).ok().filter(|v| !v.is_empty()).is_some()
}

fn main() {
    // Load local .env if present (safe — ignores if missing)
    let _ = dotenvy::dotenv();

    // Optionally validate that required secrets are present
    validate_required_secrets();

    println!("quantum-coherence-test: simple runner");
    let system = QuantumSystem::new(2, 0.02);
    println!("Created QuantumSystem with {} qubits", system.n_qubits);
    println!("T2 times: {:?}", system.t2_times);

    // Indicate presence of common secrets without printing values
    if masked_present("SERVICE_API_KEY") {
        println!("SERVICE_API_KEY is set (masked)");
    } else {
        println!("SERVICE_API_KEY not set");
    }
    if masked_present("S3_ACCESS_KEY_ID") && masked_present("S3_SECRET_ACCESS_KEY") {
        println!("S3 credentials present (masked)");
    } else {
        println!("S3 credentials not fully set");
    }
    if masked_present("DOCKERHUB_TOKEN") {
        println!("DOCKERHUB_TOKEN present (masked)");
    } else {
        println!("DOCKERHUB_TOKEN not set");
    }
}
