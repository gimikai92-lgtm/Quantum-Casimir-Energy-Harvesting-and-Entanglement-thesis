import pytest


def test_import_coherence_extension():
    # This test is skipped if the compiled Rust extension is not available.
    qc = pytest.importorskip("quantum_coherence")
    # The binding provides coherence_at_time(n_qubits, temperature, qubit, t)
    res = qc.coherence_at_time(2, 0.02, 0, 1.0)
    assert isinstance(res, float)
    assert 0.0 <= res <= 1.0
