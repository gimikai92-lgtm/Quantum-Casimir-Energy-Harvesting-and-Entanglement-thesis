# Appendix C: Measurement Techniques

## C.1 Casimir Force Measurement via AFM

### Principle
The Casimir force is measured as a shift in the cantilever's resonant frequency when brought into proximity with a surface.

### Frequency Shift Method

For a cantilever with spring constant $k$ and mass $m$:

$$\omega_0 = \sqrt{\frac{k}{m}}$$

When subjected to gradient force:

$$F(z) = -\frac{dU}{dz} = F_{\text{Casimir}}(z) + F_{\text{vdW}}(z) + F_{\text{electrostatic}}(z)$$

The effective spring constant changes:

$$k_{\text{eff}} = k - \frac{dF}{dz}\bigg|_z$$

### AFM Calibration Procedure

#### Spring Constant Measurement
```
Method: Thermal noise spectroscopy
Procedure:
1. Record cantilever thermal vibration spectrum
2. Fit to harmonic oscillator model
3. Extract: k = m·ω₀² (requires mass determination)
4. Mass from geometric measurement + density
```

Uncertainty: ±5-10%

#### Sensitivity Calibration
```
Method: Optical lever
Procedure:
1. Record voltage signal vs. known cantilever deflection
2. Use piezo-stage for reference movement
3. Calibrate: V/nm sensitivity
4. Typical: 0.1-1.0 V/nm
```

### Force Extraction

#### Raw Data Processing
```
1. Acquire: Resonant frequency vs. z-position
2. Fit each spectrum to Lorentzian
3. Extract: ω(z) with <1% uncertainty
4. Calculate: k_eff(z) = m·ω²(z)
5. Compute: dF/dz = k - k_eff
```

#### Correction for Non-Casimir Forces

**Van der Waals force:**
$$F_{\text{vdW}} = -\frac{C}{z^6}$$
Estimate from short-range asymptote, subtract.

**Electrostatic force:**
$$F_{\text{elec}} = \frac{1}{2}\frac{dC}{dz}V^2$$
Measure with zero applied voltage or apply null voltage.

#### Final Casimir Force
$$F_{\text{Casimir}} = F_{\text{measured}} - F_{\text{vdW}} - F_{\text{elec}}$$

Typical uncertainty: ±2-5%

---

## C.2 Entanglement Verification via Quantum State Tomography

### NOON State Tomography

For two-photon NOON state:
$$|\psi\rangle = \frac{1}{\sqrt{2}}(|2,0\rangle + |0,2\rangle)$$

### Measurement Protocol

#### Step 1: Basis Selection
```
Perform measurements in 3 bases:
1. Fock basis (photon number)
   Measure: Number of photons in each mode
   
2. Phase basis (±π/4 relative phase)
   Apply: Phase shifter at ±π/4
   Measure: Photon number
   
3. Balanced basis (±π/8 phase)
   Apply: Phase shifter at ±π/8
   Measure: Photon number
```

#### Step 2: Statistical Collection
```
1. Repeat measurement 10,000 times for each basis
2. Record: (n_A, n_B) for each trial
3. Build: Empirical probability distribution
```

#### Step 3: State Reconstruction
```
Linear inversion method:
1. Build measurement matrix M
2. Solve: ρ = M⁻¹ · (measurement frequencies)
3. Verify: Tr(ρ) = 1, ρ = ρ†, ρ ≥ 0
4. Correct: If necessary, apply convex optimization
```

### Fidelity Calculation

Ideal state: $|\psi_{\text{ideal}}\rangle = \frac{1}{\sqrt{2}}(|2,0\rangle + |0,2\rangle)$

Reconstructed: $\rho_{\text{measured}}$

$$\mathcal{F} = \langle \psi_{\text{ideal}} | \rho_{\text{measured}} | \psi_{\text{ideal}} \rangle$$

---

## C.3 Energy Conversion Efficiency Measurement

### Calorimetric Measurement

#### Setup
```
1. Casimir oscillator in thermally isolated chamber
2. Thermoelectric heat flow sensor on graphene surface
3. Thermistors for absolute temperature measurement
4. Electrical power input: modulated at variable frequencies
```

#### Procedure
```
1. Establish baseline: no vibration, measure heat loss
2. Drive oscillator: amplitude A, frequency ω
3. Measure: Heat flow into cooling system
4. Calculate: Power from mechanical vibration
5. Measure: Electrical output voltage and current
6. Compute: Electrical power generated
```

### Efficiency Definition

$$\eta = \frac{P_{\text{electrical}}}{P_{\text{mechanical}}} = \frac{P_{\text{electrical}}}{F_{\text{Cas}} \cdot v_{\text{amplitude}} \cdot \omega}$$

Where $v_{\text{amplitude}} = A \cdot \omega$ is peak velocity.

### Uncertainty Analysis

Sources of uncertainty:
1. Mechanical power measurement: ±5%
2. Electrical power measurement: ±2%
3. Temperature stability: ±1%
4. Coupling efficiency: ±3%

**Total uncertainty: ±8%**

---

**All measurements performed with NIST-traceable calibration standards.**
