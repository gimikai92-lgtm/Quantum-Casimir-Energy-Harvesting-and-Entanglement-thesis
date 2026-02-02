use predicates::prelude::*;

#[test]
fn run_cli_prints_info() {
    // Use the cargo macro-compatible command generator to avoid deprecated API
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("coherence-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created QuantumSystem"));
}
