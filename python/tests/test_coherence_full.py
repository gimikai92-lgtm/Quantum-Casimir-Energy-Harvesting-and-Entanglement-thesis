import json
import os
import subprocess
import sys


def send_command(proc, cmd):
    proc.stdin.write(json.dumps(cmd) + "\n")
    proc.stdin.flush()
    # read one line of output
    line = proc.stdout.readline()
    return json.loads(line)


def test_run_test_and_measure():
    script_path = os.path.join(os.path.dirname(__file__), "..", "coherence_test.py")
    script_path = os.path.normpath(script_path)
    proc = subprocess.Popen([sys.executable, script_path], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    try:
        # Run test
        resp = send_command(proc, {"command": "run_test", "n_qubits": 2, "temperature": 0.02, "noise_config": {}, "num_tests": 1})
        assert resp.get("type") == "test_results"
        results = resp.get("results")
        assert results.get("n_qubits") == 2

        # Create superposition
        ack = send_command(proc, {"command": "create_superposition", "qubit": 0, "theta": 1.5708, "phi": 0.0})
        assert ack.get("type") in ("ack", "test_results", "error")

        # Measure fidelity
        fid = send_command(proc, {"command": "measure_fidelity", "qubit": 0})
        assert fid.get("type") == "fidelity"
        assert isinstance(fid.get("fidelity"), float)

        # Stop - no response expected, just send command
        proc.stdin.write(json.dumps({"command": "stop"}) + "\n")
        proc.stdin.flush()
    finally:
        # Try graceful shutdown and ensure all pipes are closed to avoid ResourceWarning
        try:
            # If the test flow didn't already stop the child, send stop command
            if proc.poll() is None:
                try:
                    proc.stdin.write(json.dumps({"command": "stop"}) + "\n")
                    proc.stdin.flush()
                except Exception:
                    pass
            # Wait briefly for clean exit
            try:
                proc.wait(timeout=2)
            except subprocess.TimeoutExpired:
                proc.terminate()
                try:
                    proc.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait()
        finally:
            # Close any open file objects
            try:
                if proc.stdin:
                    proc.stdin.close()
            except Exception:
                pass
            try:
                if proc.stdout:
                    proc.stdout.close()
            except Exception:
                pass
            try:
                if proc.stderr:
                    proc.stderr.close()
            except Exception:
                pass
