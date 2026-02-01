# Appendix D: Simulation Code

## D.1 Quantum-Casimir Simulation (Python)

### Core Simulator Module

```python
"""
QuantumCasimirSimulator: Coupled quantum-Casimir dynamics
Author: Rolando M Garcia, 2026
"""

import numpy as np
from scipy.integrate import odeint
from scipy.optimize import minimize

class CasimirOscillator:
    """Models Casimir force driven oscillator with quantum effects"""
    
    def __init__(self, params):
        """
        params: dict with keys
            k: spring constant (N/m)
            m: effective mass (kg)
            d0: initial separation (m)
            A: graphene area (m²)
            T: temperature (K)
            dopant_level: Li-N doping enhancement factor
        """
        self.k = params['k']
        self.m = params['m']
        self.d0 = params['d0']
        self.A = params['A']
        self.T = params['T']
        self.dopant_factor = params['dopant_level']
        
        # Physical constants
        self.hbar = 1.055e-34
        self.c = 3e8
        self.pi = np.pi
        
    def casimir_force(self, separation):
        """
        Calculate Casimir force with doping enhancement
        F = -α·hbar·c·π²/(240·d⁴)·A·dopant_factor
        """
        d = separation
        base_force = (
            self.dopant_factor * self.hbar * self.c * 
            (self.pi**2) / (240 * d**4) * self.A
        )
        return -base_force  # Negative = attractive
    
    def dynamics(self, state, t):
        """
        ODE system: d²x/dt² = F_casimir/m + damping
        state = [x, v] where x = d - d0, v = dx/dt
        """
        x, v = state
        d = self.d0 + x
        
        # Casimir force
        F_cas = self.casimir_force(d)
        
        # Damping (Q = 12000)
        Q = 12000
        omega0 = np.sqrt(self.k / self.m)
        damping = -2 * omega0 * v / Q
        
        # Acceleration
        a = (F_cas + self.k * (-x)) / self.m + damping
        
        return [v, a]
    
    def simulate(self, t_span, x0=1e-9, v0=0):
        """
        Simulate system dynamics
        t_span: time array (seconds)
        x0: initial displacement (m)
        v0: initial velocity (m/s)
        """
        state0 = [x0, v0]
        solution = odeint(self.dynamics, state0, t_span)
        return solution  # shape: (len(t_span), 2)
    
    def power_output(self, amplitude, frequency):
        """
        Calculate power from Casimir oscillator
        P = 0.5 * F_avg * A_osc * ω
        """
        d_min = self.d0 - amplitude
        d_max = self.d0 + amplitude
        
        # Average force (numerical integration)
        d_vals = np.linspace(d_min, d_max, 100)
        F_vals = np.array([self.casimir_force(d) for d in d_vals])
        F_avg = np.mean(np.abs(F_vals))
        
        power = 0.5 * F_avg * amplitude * frequency
        return power


# Example usage
if __name__ == "__main__":
    params = {
        'k': 0.1,  # N/m
        'm': 1e-9,  # kg (1 ng)
        'd0': 20e-9,  # 20 nm
        'A': 1e-4,  # 1 cm²
        'T': 0.02,  # 20 K
        'dopant_level': 8.2  # Enhancement factor
    }
    
    oscillator = CasimirOscillator(params)
    
    # Simulate 10 microseconds
    t = np.linspace(0, 10e-6, 10000)
    trajectory = oscillator.simulate(t)
    
    # Calculate power
    amplitude = 10e-9  # 10 nm
    frequency = 850e3  # 850 kHz
    power = oscillator.power_output(amplitude, frequency)
    
    print(f"Power output: {power*1000:.1f} mW")
```

---

## D.2 Energy Routing Algorithms (Python)

### Quantum Routing Engine

```python
"""
QuantumEnergyRouter: Entanglement-based energy routing
"""

import numpy as np
from collections import defaultdict

class QuantumRouter:
    """Routes quantum energy using entanglement"""
    
    def __init__(self, network_graph):
        """
        network_graph: dict mapping node -> [neighbors]
        Weighted by entanglement fidelity
        """
        self.graph = network_graph
        self.nodes = set()
        for node, neighbors in network_graph.items():
            self.nodes.add(node)
            self.nodes.update(neighbors)
    
    def compute_entanglement_path(self, source, target, available_entanglement):
        """
        Find path maximizing entanglement-directed routing
        available_entanglement: dict of link -> fidelity
        """
        # Dijkstra-like algorithm modified for entanglement
        distances = {node: -np.inf for node in self.nodes}
        distances[source] = 1.0  # Start with perfect fidelity
        
        visited = set()
        while len(visited) < len(self.nodes):
            # Find unvisited node with highest fidelity
            best_node = max(
                (n for n in self.nodes if n not in visited),
                key=lambda x: distances[x]
            )
            visited.add(best_node)
            
            if best_node == target:
                return distances[target]
            
            # Update neighbors
            if best_node in self.graph:
                for neighbor in self.graph[best_node]:
                    link = (best_node, neighbor)
                    ent_fidelity = available_entanglement.get(link, 0.95)
                    new_dist = distances[best_node] * ent_fidelity
                    distances[neighbor] = max(distances[neighbor], new_dist)
        
        return distances[target]
    
    def optimize_routing(self, sources, targets, energy_demand):
        """
        Distribute energy from sources to targets
        Minimize: (energy loss) + (1 - fidelity)
        """
        routing = defaultdict(float)
        
        for target, demand in energy_demand.items():
            remaining = demand
            for source in sources:
                path_fidelity = self.compute_entanglement_path(
                    source, target, {}
                )
                allocation = remaining * path_fidelity
                routing[(source, target)] = allocation
                remaining -= allocation
        
        return routing


# Example usage
if __name__ == "__main__":
    network = {
        'harvester': ['router1', 'router2'],
        'router1': ['qubit1', 'qubit2'],
        'router2': ['qubit3', 'qubit4'],
    }
    
    router = QuantumRouter(network)
    
    demand = {
        'qubit1': 400e-3,  # 400 mW
        'qubit2': 300e-3,
        'qubit3': 500e-3,
        'qubit4': 500e-3,
    }
    
    routing = router.optimize_routing(['harvester'], 
                                     demand.keys(), demand)
    
    total_allocated = sum(routing.values())
    print(f"Total energy routed: {total_allocated*1000:.1f} mW")
```

---

## D.3 System Optimization Module

```python
"""
SystemOptimizer: Multi-objective optimization for energy system
"""

from scipy.optimize import minimize, LinearConstraint
import numpy as np

class SystemOptimizer:
    """Optimizes complete quantum energy system"""
    
    def __init__(self, system_params):
        self.params = system_params
    
    def objective(self, x):
        """
        Minimize: E_loss + (1 - entanglement_fidelity)
        x = [casimir_amplitude, entanglement_generation_rate, ...]
        """
        casimir_amp, ent_rate = x[:2]
        
        # Energy loss component
        power_generated = 1700 * (casimir_amp / 10e-9) ** 2  # Cascades with amplitude
        power_needed = 2440
        energy_loss = abs(power_generated - power_needed)
        
        # Fidelity component  
        entanglement_fidelity = 0.5 + 0.5 * np.exp(-ent_rate / 100e3)
        fidelity_cost = 1 - entanglement_fidelity
        
        return energy_loss / 1000 + fidelity_cost
    
    def optimize(self):
        """Find optimal operating parameters"""
        x0 = [10e-9, 50e3]  # Initial guess
        bounds = [(1e-9, 100e-9), (1e3, 1e6)]
        
        result = minimize(
            self.objective,
            x0,
            bounds=bounds,
            method='L-BFGS-B'
        )
        
        return result


# Run optimization
if __name__ == "__main__":
    optimizer = SystemOptimizer({})
    result = optimizer.optimize()
    
    print("Optimal parameters:")
    print(f"  Casimir amplitude: {result.x[0]*1e9:.2f} nm")
    print(f"  Entanglement rate: {result.x[1]/1e3:.1f} kHz")
    print(f"Minimum objective value: {result.fun:.4f}")
```

---

**All code is MIT-licensed and available on GitHub.**
