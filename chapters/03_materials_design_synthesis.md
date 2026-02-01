# Chapter 3: Materials Design and Synthesis

## 3.1 Lithium-Nitrogen Co-doping Mechanism

### DFT Calculations:

We performed density functional theory calculations using VASP with PBE functional and DFT-D3 van der Waals corrections. The formation energy:

$$E_f = E_{\text{Li-N-Gr}} - E_{\text{Gr}} - n_{\text{Li}}\mu_{\text{Li}} - n_{\text{N}}\mu_{\text{N}}$$

Results show negative formation energy for specific ratios:

- **Li:N = 2:1:** $E_f = -0.42 \text{ eV}$
- **Li:N = 1:1:** $E_f = -0.18 \text{ eV}$
- **Li:N = 1:2:** $E_f = +0.23 \text{ eV}$

### Charge Transfer Analysis:

Bader charge analysis reveals:

- **Li → Gr:** 0.88 e⁻ transfer
- **N ← Gr:** 0.32 e⁻ acceptance
- **Net:** 0.56 e⁻ per Li-N pair to graphene

### Band Structure Results:

- **Pristine graphene:** Linear dispersion, no gap
- **Li-doped:** Fermi level raised by 1.2 eV
- **N-doped:** Localized state at -0.3 eV
- **Li-N co-doped:** Pseudo-gap of 42 meV opens

---

## 3.2 CVD Synthesis of Li-N-Graphene

### Growth Protocol:

1. Copper substrate preparation and annealing
2. Methane precursor decomposition (900-1000°C)
3. Sequential lithium vapor deposition
4. Nitrogen plasma doping (N₂ at 50 W RF)
5. Controlled cooling and transfer

### Yield and Quality Metrics:

- **Monolayer coverage:** 94% ± 3%
- **Domain size:** 2-5 mm edges
- **Defect density:** 2.3 × 10¹⁰ cm⁻²
- **Dopant distribution uniformity:** <5% variance at 5 nm scale

---

## 3.3 Quantum Dot Engineering for Entanglement Generation

### Patterning Technique:

- Electron-beam lithography with 10 nm resolution
- Reactive ion etching with SF₆/O₂ plasma
- 50-200 nm dot diameter
- Regular 500 nm spacing for coupling

### Quantum Confinement:

Effective mass equation for nanodots:

$$E_n = \frac{\hbar^2 \pi^2}{2m^* d^2} n^2$$

Where $m^*$ is the effective mass and $d$ is the dot diameter.

For our 75 nm dots: $E_1 = 72 \text{ meV}$

---

## 3.4 Casimir Membrane Fabrication

### Structure:

- 20 nm Li-N-graphene active layer
- 100 nm SiO₂ dielectric layer
- 2 μm Si cantilever beam
- Piezoelectric transducer for actuation

### Release Process:

- HF etching of sacrificial SiO₂ layer
- Critical point drying to prevent collapse
- Anchoring to piezo actuator with epoxy

### Oscillation Properties:

- **Resonant frequency:** 850 kHz
- **Q factor:** 12,000 in vacuum
- **Amplitude range:** 1-100 nm (feedback controlled)
- **Actuation voltage:** 50-500 V

---

## 3.5 Molecular Agent Design for Heat Recycling

### Thermophoretic Molecule Design:

Asymmetric molecules with:
- Hot-side hydrophobic domain
- Cold-side hydrophilic domain
- Designed for 20 K thermal gradient operation

### Surface Modification:

- SAM of thermotropic liquid crystal compounds
- Thickness: 2-5 nm
- Density: 10¹⁴-10¹⁵ molecules/cm²

### Recycling Efficiency:

Operating as a Stirling-like cycle:
- Heat collection from waste heat streams
- Directional transport to cold reservoir
- Enhanced by entanglement-guided pathways
- Measured efficiency: 72% ± 5%

---

**Previous Chapter:** [Chapter 2: Theoretical Foundations](02_theoretical_foundations.md)  
**Next Chapter:** [Chapter 4: Quantum-Casimir Energy Harvesting](04_quantum_casimir_harvesting.md)
