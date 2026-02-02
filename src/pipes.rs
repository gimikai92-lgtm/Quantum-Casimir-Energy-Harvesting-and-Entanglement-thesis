//! Python pipe communication for quantum coherence testing
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::channel::{unbounded, Sender, Receiver};
use serde::{Serialize, Deserialize};
use crate::coherence::CoherenceTestResults;

/// Command to send to Python
#[derive(Debug, Serialize, Deserialize)]
pub enum PythonCommand {
    /// Run coherence test with specified parameters
    RunTest {
        /// Number of qubits in the system
        n_qubits: usize,
        /// System temperature in Kelvin
        temperature: f64,
        /// Noise configuration settings
        noise_config: NoiseConfig,
        /// Number of test iterations
        num_tests: usize,
    },
    /// Create superposition state on a qubit
    CreateSuperposition {
        /// Target qubit index
        qubit: usize,
        /// Polar angle (theta) for Bloch sphere
        theta: f64,
        /// Azimuthal angle (phi) for Bloch sphere
        phi: f64,
    },
    /// Measure fidelity of a qubit
    MeasureFidelity {
        /// Target qubit index
        qubit: usize,
    },
    /// Get results
    GetResults,
    /// Stop Python process
    Stop,
}

/// Response from Python
#[derive(Debug, Serialize, Deserialize)]
pub enum PythonResponse {
    /// Test results
    TestResults(CoherenceTestResults),
    /// Fidelity measurement
    Fidelity(f64),
    /// Error message
    Error(String),
    /// Acknowledgement
    Ack,
}

/// Noise configuration for Python
#[derive(Debug, Serialize, Deserialize)]
pub struct NoiseConfig {
    /// Temperature for thermal noise (Kelvin)
    pub thermal_temperature: Option<f64>,
    /// Dephasing rates per qubit (Hz)
    pub dephasing_rates: Option<Vec<f64>>,
    /// Amplitude damping rates per qubit (Hz)
    pub amplitude_damping_rates: Option<Vec<f64>>,
    /// 1/f noise strength coefficient
    pub one_over_f_strength: Option<f64>,
    /// 1/f noise frequency exponent
    pub one_over_f_exponent: Option<f64>,
}

/// Python pipe interface
pub struct PythonPipe {
    python_process: Arc<Mutex<Option<Child>>>,
    #[allow(dead_code)]
    stdout_receiver: Receiver<String>,
    command_sender: Sender<PythonCommand>,
    response_receiver: Receiver<PythonResponse>,
}

impl PythonPipe {
    /// Create new Python pipe interface
    pub fn new(python_script: Option<String>) -> Result<Self, io::Error> {
        let script = python_script.unwrap_or_else(|| {
            // Default Python script for coherence testing
            r#"
import json
import sys

def run_test(config):
    return {"result": "test_complete", "fidelity": 0.95}

while True:
    try:
        line = sys.stdin.readline()
        if not line:
            break
        config = json.loads(line)
        result = run_test(config)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({"error": str(e)}))
"#.to_string()
        });

        // Write Python script to temp file
        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("coherence_test.py");
        std::fs::write(&script_path, script)?;

        // Start Python process
        let mut python_process = std::process::Command::new("python3")
            .arg(&script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = python_process.stdin.take().unwrap();
        let stdout = python_process.stdout.take().unwrap();

        // Create channels for communication
        let (stdout_sender, stdout_receiver) = unbounded();
        let (command_sender, command_receiver) = unbounded();
        let (response_sender, response_receiver) = unbounded();

        // Start reader thread
        let stdout_reader = BufReader::new(stdout);
        thread::spawn(move || {
            for line in stdout_reader.lines() {
                match line {
                    Ok(line) => {
                        if let Ok(response) = serde_json::from_str::<PythonResponse>(&line) {
                            let _ = response_sender.send(response);
                        }
                        let _ = stdout_sender.send(line);
                    }
                    Err(e) => eprintln!("Error reading from Python: {}", e),
                }
            }
        });

        // Start writer thread
        let stdin_writer = Arc::new(Mutex::new(stdin));
        thread::spawn(move || {
            while let Ok(command) = command_receiver.recv() {
                let json = serde_json::to_string(&command).unwrap();
                if let Ok(mut stdin) = stdin_writer.lock() {
                    let _ = stdin.write_all(format!("{}\n", json).as_bytes());
                    let _ = stdin.flush();
                }
            }
        });

        Ok(PythonPipe {
            python_process: Arc::new(Mutex::new(Some(python_process))),
            stdout_receiver,
            command_sender,
            response_receiver,
        })
    }

    /// Send command to Python
    pub fn send_command(&self, command: PythonCommand) -> Result<(), io::Error> {
        self.command_sender.send(command)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    /// Receive response from Python
    pub fn receive_response(&self, timeout_ms: u64) -> Result<Option<PythonResponse>, io::Error> {
        use std::time::Duration;

        match self.response_receiver.recv_timeout(Duration::from_millis(timeout_ms)) {
            Ok(response) => Ok(Some(response)),
            Err(crossbeam::channel::RecvTimeoutError::Timeout) => Ok(None),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    /// Run coherence test via Python
    pub fn run_coherence_test(
        &self,
        n_qubits: usize,
        temperature: f64,
        noise_config: NoiseConfig,
        num_tests: usize,
    ) -> Result<CoherenceTestResults, io::Error> {
        self.send_command(PythonCommand::RunTest { n_qubits, temperature, noise_config, num_tests })?;

        // Wait for response
        let start_time = std::time::Instant::now();
        while start_time.elapsed().as_millis() < 30000 {
            if let Some(response) = self.receive_response(100)? {
                match response {
                    PythonResponse::TestResults(results) => return Ok(results),
                    PythonResponse::Error(err) => {
                        return Err(io::Error::new(io::ErrorKind::Other, err));
                    }
                    _ => {}
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::TimedOut, "Timeout waiting for test results"))
    }

    /// Close the pipe
    pub fn close(&mut self) -> Result<(), io::Error> {
        let _ = self.send_command(PythonCommand::Stop);

        if let Some(mut process) = self.python_process.lock().unwrap().take() {
            let _ = process.kill();
            let _ = process.wait();
        }

        Ok(())
    }
}

impl Drop for PythonPipe {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
