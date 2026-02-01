#!/usr/bin/env python3
"""
Python side of the quantum coherence testing pipe.
Communicates with Rust via stdin/stdout JSON messages.
This file mirrors the original project script with a mock fallback when the Rust
extension is not available.
"""
import sys
import json
import time
import numpy as np
from typing import Dict, List, Any, Optional
import traceback

# Try to import the Rust module
try:
    import quantum_coherence
    RUST_AVAILABLE = True
except ImportError:
    print("Warning: Rust module not found. Using mock implementation.", file=sys.stderr)
    RUST_AVAILABLE = False


class MockQuantumSystem:
    """Mock implementation when Rust module is not available"""
    def __init__(self, n_qubits: int, temperature: float):
        self.n_qubits = n_qubits
        self.temperature = temperature
        self.t2_times = [85.0 * (temperature * 1000)**-0.7 for _ in range(n_qubits)]

    def create_superposition(self, qubit: int, theta: float, phi: float):
        """Mock create superposition"""
        pass

    def measure_superposition_fidelity(self, qubit: int) -> float:
        """Mock measure fidelity"""
        base_fidelity = 0.99
        noise = 0.01 * np.random.random()
        return max(0.0, min(1.0, base_fidelity - noise))

    def apply_noise(self, duration: float):
        """Mock apply noise"""
        pass

    def calculate_purity(self) -> float:
        """Mock calculate purity"""
        return 0.98 + 0.02 * np.random.random()


class QuantumCoherenceTest:
    """Main test class for Python side"""
    def __init__(self):
        self.system = None
        self.results = []

    def run_test(self, n_qubits: int, temperature: float,
                 noise_config: Dict[str, Any], num_tests: int) -> Dict[str, Any]:
        """Run coherence test"""
        if RUST_AVAILABLE:
            # Use Rust implementation
            import quantum_coherence as qc
            # Create system
            self.system = qc.QuantumSystem(n_qubits, temperature)
            # Create superposition on each qubit
            for qubit in range(n_qubits):
                self.system.create_superposition(qubit, np.pi/2, 0.0)
                fidelity = self.system.measure_superposition_fidelity(qubit)
                fidelities = []
                for t in np.linspace(0, self.system.t2_times[qubit], 10):
                    test_system = self.system
                    fidelity_t = test_system.measure_superposition_fidelity(qubit)
                    fidelities.append(fidelity_t)
                self.results.append({
                    'qubit': qubit,
                    'initial_fidelity': fidelity,
                    'fidelities': fidelities,
                    't2': self.system.t2_times[qubit],
                })
        else:
            # Use mock implementation
            self.system = MockQuantumSystem(n_qubits, temperature)
            for qubit in range(n_qubits):
                self.system.create_superposition(qubit, np.pi/2, 0.0)
                fidelity = self.system.measure_superposition_fidelity(qubit)
                fidelities = []
                for _ in range(10):
                    self.system.apply_noise(0.1)
                    fidelity_t = self.system.measure_superposition_fidelity(qubit)
                    fidelities.append(fidelity_t)
                self.results.append({
                    'qubit': qubit,
                    'initial_fidelity': fidelity,
                    'fidelities': fidelities,
                    't2': self.system.t2_times[qubit],
                })
        return self.compile_results()

    def compile_results(self) -> Dict[str, Any]:
        """Compile results into dictionary"""
        if not self.results:
            return {}
        avg_fidelity = np.mean([r['initial_fidelity'] for r in self.results])
        avg_t2 = np.mean([r['t2'] for r in self.results])
        min_t2 = min([r['t2'] for r in self.results])
        max_t2 = max([r['t2'] for r in self.results])
        return {
            'n_qubits': self.system.n_qubits if self.system else 0,
            'temperature': self.system.temperature if self.system else 0.0,
            'average_fidelity': float(avg_fidelity),
            'average_t2': float(avg_t2),
            'min_t2': float(min_t2),
            'max_t2': float(max_t2),
            'qubit_results': self.results,
            'timestamp': time.time(),
        }


def main():
    """Main function for pipe communication"""
    test = QuantumCoherenceTest()
    try:
        for line in sys.stdin:
            line = line.strip()
            if not line:
                continue
            try:
                command = json.loads(line)
            except json.JSONDecodeError as e:
                response = {
                    'type': 'error',
                    'message': f'JSON decode error: {str(e)}'
                }
                print(json.dumps(response))
                sys.stdout.flush()
                continue
            if command.get('command') == 'run_test':
                results = test.run_test(
                    n_qubits=command.get('n_qubits', 1),
                    temperature=command.get('temperature', 0.02),
                    noise_config=command.get('noise_config', {}),
                    num_tests=command.get('num_tests', 1),
                )
                response = {
                    'type': 'test_results',
                    'results': results,
                }
                print(json.dumps(response))
                sys.stdout.flush()
            elif command.get('command') == 'create_superposition':
                if test.system:
                    test.system.create_superposition(
                        qubit=command['qubit'],
                        theta=command['theta'],
                        phi=command['phi'],
                    )
                response = {'type': 'ack'}
                print(json.dumps(response))
                sys.stdout.flush()
            elif command.get('command') == 'measure_fidelity':
                fidelity = 0.0
                if test.system:
                    fidelity = test.system.measure_superposition_fidelity(command['qubit'])
                response = {
                    'type': 'fidelity',
                    'fidelity': fidelity,
                }
                print(json.dumps(response))
                sys.stdout.flush()
            elif command.get('command') == 'get_results':
                results = test.compile_results()
                response = {
                    'type': 'test_results',
                    'results': results,
                }
                print(json.dumps(response))
                sys.stdout.flush()
            elif command.get('command') == 'stop':
                break
            else:
                response = {
                    'type': 'error',
                    'message': f'Unknown command: {command.get("command")}'
                }
                print(json.dumps(response))
                sys.stdout.flush()
    except Exception as e:
        response = {'type': 'error', 'message': f'Error processing command: {str(e)}', 'traceback': traceback.format_exc()}
        print(json.dumps(response))
        sys.stdout.flush()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
#!/usr/bin/env python3
"""
Minimal Python-side coherence test script used for CI/testing.
Accepts JSON commands on stdin and writes JSON responses to stdout.
This is a lightweight stub to exercise the Python test environment.
"""
import sys
import json


def main():
    try:
        for line in sys.stdin:
            line = line.strip()
            if not line:
                continue
            try:
                cmd = json.loads(line)
            except json.JSONDecodeError:
                resp = {"type": "error", "message": "invalid json"}
                print(json.dumps(resp))
                sys.stdout.flush()
                continue
            c = cmd.get("command")
            if c == "run_test":
                # Return a minimal test result
                resp = {"type": "test_results", "results": {"n_qubits": cmd.get("n_qubits", 1), "average_fidelity": 0.99}}
                print(json.dumps(resp))
                sys.stdout.flush()
            elif c == "stop":
                break
            else:
                resp = {"type": "ack"}
                print(json.dumps(resp))
                sys.stdout.flush()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
