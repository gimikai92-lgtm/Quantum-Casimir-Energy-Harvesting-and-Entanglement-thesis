# Code Directory

This directory contains all computational code, simulations, and algorithms related to the thesis.

## Files

### Core Simulations

- `quantum_casimir_simulation.py` - Main Casimir oscillator simulator
- `energy_routing_algorithms.py` - Entanglement-based energy routing
- `system_optimization.py` - Multi-objective system optimization

### Requirements

```
numpy>=1.21.0
scipy>=1.8.0
matplotlib>=3.5.0
jupyter>=1.0.0
```

Install with:
```bash
pip install -r requirements.txt
```

## Usage

### Running Simulations

```python
from quantum_casimir_simulation import CasimirOscillator

params = {
    'k': 0.1,          # Spring constant (N/m)
    'm': 1e-9,         # Mass (kg)
    'd0': 20e-9,       # Initial separation (m)
    'A': 1e-4,         # Area (m²)
    'T': 0.02,         # Temperature (K)
    'dopant_level': 8.2  # Enhancement factor
}

oscillator = CasimirOscillator(params)
# ... run simulations
```

### Running Optimization

```python
from system_optimization import SystemOptimizer

optimizer = SystemOptimizer({})
result = optimizer.optimize()
print(f"Optimal amplitude: {result.x[0]*1e9:.2f} nm")
```

## Documentation

Full implementation details in:
- [Appendix D: Simulation Code](../appendices/D_simulation_code.md)

## License

MIT License - See [LICENSE](../LICENSE)

## Author

Rolando M Garcia, 2026
