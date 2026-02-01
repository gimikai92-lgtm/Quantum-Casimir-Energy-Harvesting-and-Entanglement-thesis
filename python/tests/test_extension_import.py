import pytest


def test_import_coherence_extension():
    # This test is skipped if the compiled Rust extension is not available.
    qc = pytest.importorskip("quantum_coherence")
    # The binding provides coherence_at_time_py(n_qubits, temperature, qubit, t)
    res = qc.coherence_at_time_py(2, 0.02, 0, 1.0)
    assert isinstance(res, float)
    assert 0.0 <= res <= 1.0


def test_extension_additional_helpers():
    qc = pytest.importorskip("quantum_coherence")
    # coherence_ratio_py should equal exp(-(t2 - t1) / T2)
    ratio = qc.coherence_ratio_py(2, 0.02, 0, 2.0, 5.0)
    assert isinstance(ratio, float)
    assert 0.0 < ratio <= 1.0

    # purity_of_ghz_py should be near 1 for a GHZ state
    purity = qc.purity_of_ghz_py(3, 0.01)
    assert isinstance(purity, float)
    assert 0.9 <= purity <= 1.0
