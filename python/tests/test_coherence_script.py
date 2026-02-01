import subprocess
import json
import sys
import os


def test_coherence_script_runs_and_exits():
    """Start the coherence_test.py script, send a stop command, and verify it exits cleanly."""
    script_path = os.path.join(os.path.dirname(__file__), '..', 'coherence_test.py')
    script_path = os.path.normpath(script_path)
    # Start the script as a subprocess
    proc = subprocess.Popen([sys.executable, script_path], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    # Send a 'stop' command in JSON form
    cmd = json.dumps({"command": "stop"}) + "\n"
    try:
        out, err = proc.communicate(cmd, timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()
        out, err = proc.communicate()
        raise
    # Process should exit normally
    assert proc.returncode == 0
