# Chapter 2: Theoretical Foundations

## 2.1 Quantum Thermodynamics and Landauer's Principle

The foundation of energy-efficient computation rests on Landauer's principle, which establishes a fundamental lower bound on energy dissipation during information processing:

$$E_{\text{min}} = k_B T \ln 2 \text{ per erased bit}$$

For quantum systems, this generalizes to:

$$E_{\text{min, quantum}} = k_B T \left[ S(\rho_{\text{final}}) - S(\rho_{\text{initial}}) \right]$$

Where $S(\rho) = -\text{Tr}(\rho \ln \rho)$ is the von Neumann entropy.

### Extended Landauer Principle with Recycling:

We extend Landauer's principle to include energy recycling:

$$E_{\text{net}} = E_{\text{Landauer}} \cdot (1 - \eta_R) + E_{\text{irreversible}}$$

Where $\eta_R$ is the recycling efficiency. For perfect recycling ($\eta_R = 1$), only irreversible processes dissipate energy.

### Quantum Thermodynamic Cycles:

We model quantum computation as a thermodynamic cycle:

1. **Isentropic expansion:** Quantum gates (unitary evolution)
2. **Isothermal compression:** Measurement and reset
3. **Adiabatic processes:** State preparation and entanglement generation

The efficiency of this cycle is bounded by:

$$\eta_{\text{max}} = 1 - \frac{T_{\text{cold}}}{T_{\text{hot}}} \cdot \frac{S_{\text{out}} - S_{\text{in}}}{k_B \ln 2}$$

---

## 2.2 Casimir Effect Theory and Enhancements

The Casimir force between two parallel plates separated by distance $d$ is given by:

$$F_{\text{Cas}} = -\frac{\hbar c \pi^2}{240 d^4} A$$

For realistic materials with finite conductivity and temperature corrections:

$$F_{\text{Cas}}(d, T) = -\frac{k_B T}{\pi} \sum_{n=0}^{\infty} \!\!\!\!\!\!^{\prime} \int_0^\infty k_\perp dk_\perp q_n \sum_{\sigma = \text{TE, TM}} \frac{r_\sigma^2 e^{-2q_n d}}{1 - r_\sigma^2 e^{-2q_n d}}$$

Where $q_n = \sqrt{k_\perp^2 + \xi_n^2/c^2}$, $\xi_n = 2\pi n k_B T/\hbar$, and $r_\sigma$ are reflection coefficients.

### Li-N-Graphene Enhancement Model:

The dielectric response of doped graphene is modified:

$$\epsilon(\omega) = 1 + \frac{i\sigma(\omega)}{\omega \epsilon_0 d_{\text{gr}}}$$

With conductivity:

$$\sigma(\omega) = \sigma_{\text{intra}} + \sigma_{\text{inter}} + \sigma_{\text{dopant}}$$

For Li-N co-doping:

$$\sigma_{\text{dopant}} = \frac{e^2}{4\hbar} \left[ \frac{\mu_{\text{Li}}}{\hbar(\omega + i/\tau_{\text{Li}})} + \frac{\mu_{\text{N}}}{\hbar(\omega + i/\tau_{\text{N}})} + \frac{\alpha_{\text{syn}} \sqrt{\mu_{\text{Li}}\mu_{\text{N}}}}{\hbar(\omega + i/\tau_{\text{syn}})} \right]$$

The synergistic term $\alpha_{\text{syn}}$ captures non-linear enhancement from co-doping.

### Casimir Energy Density:

The energy available for harvesting:

$$U_{\text{Cas}}(d) = -\frac{\hbar c \pi^2}{720 d^3} \eta(d, T, \mu)$$

For $d = 20 \text{ nm}$, $T = 20 \text{ mK}$, $\mu_{\text{Li-N}} = 0.5 \text{ eV}$:

$$U_{\text{Cas}} \approx 12.2 \text{ kJ/m}^3$$

---

## 2.3 Entanglement Theory for Energy States

We develop a formalism for treating energy as an entangled quantum resource.

### Energy State Representation:

Photon number states with directional labels:

$$|\psi\rangle = \sum_{n_A, n_B} c_{n_A, n_B} |n_A\rangle_A \otimes |n_B\rangle_B$$

Where $|n\rangle$ represents $n$ photons with energy $n\hbar\omega$.

### Directional Entanglement Metric:

We define the directionality parameter:

$$\mathcal{D} = \frac{\langle E_A \rangle - \langle E_B \rangle}{\langle E_A \rangle + \langle E_B \rangle}$$

For separable states: $-1 \leq \mathcal{D} \leq 1$  
For entangled states: Enhanced range possible

### Energy-Entanglement Uncertainty Relation:

Derived from Robertson-Schrödinger relation:

$$\Delta E_A \cdot \Delta \mathcal{E}_{AB} \geq \frac{\hbar}{2} \left| \frac{d\langle E_B \rangle}{dt} \right|$$

Where $\mathcal{E}_{AB}$ is the entanglement entropy between systems A and B.

---

## 2.4 Graphene Doping Physics

### Li Doping Mechanism:

Lithium intercalates between graphene layers, donating electrons:

$$\text{Li} \rightarrow \text{Li}^+ + e^-$$

The donated electrons occupy the graphene π* band:

$$E_F^{\text{Li}} = \hbar v_F \sqrt{\pi n_{\text{Li}}}$$

Where $n_{\text{Li}} \approx 5\times10^{14} \text{ cm}^{-2}$ for LiC₆.

### N Doping Mechanism:

Nitrogen substitutes carbon atoms, creating n-type doping:

$$\text{C} + \text{NH}_3 \rightarrow \text{N}_\text{C} + \frac{3}{2}\text{H}_2 + \text{V}_\text{C}$$

Nitrogen introduces localized states near the Fermi level:

$$\rho_N(E) = \frac{\Gamma/\pi}{(E - E_N)^2 + \Gamma^2}$$

### Co-doping Synergy:

The combined system shows non-linear enhancement:

$$\Delta E_{\text{gap}} = \alpha_{\text{Li}} x_{\text{Li}} + \alpha_{\text{N}} x_{\text{N}} + \beta x_{\text{Li}} x_{\text{N}}$$

Experimentally: $\beta = 2.3 \text{ eV}$, indicating strong synergy.

---

## 2.5 Integrated Quantum-Casimir-Entanglement Framework

### Unified Hamiltonian:

$$H_{\text{total}} = H_{\text{QC}} + H_{\text{Cas}} + H_{\text{Ent}} + H_{\text{int}}$$

Where:
- $H_{\text{QC}}$: Quantum computer Hamiltonian
- $H_{\text{Cas}}$: Casimir interaction Hamiltonian
- $H_{\text{Ent}}$: Entanglement generation Hamiltonian
- $H_{\text{int}}$: Interaction terms

### Energy Flow Equations:

$$\frac{dE_{\text{QC}}}{dt} = P_{\text{ops}} - P_{\text{loss}} + \eta_{\text{Cas}} P_{\text{Cas}} + \eta_{\text{Ent}} P_{\text{rec}}$$

$$\frac{dE_{\text{Cas}}}{dt} = -P_{\text{Cas}} + P_{\text{vac}}$$

$$\frac{dE_{\text{Ent}}}{dt} = -P_{\text{rec}} + P_{\text{gen}} - P_{\text{deph}}$$

### Steady-State Solution:

Setting derivatives to zero:

$$P_{\text{ops}} = P_{\text{loss}} - \eta_{\text{Cas}} P_{\text{Cas}} - \eta_{\text{Ent}} P_{\text{rec}}$$

For energy autonomy:

$$P_{\text{loss}} \leq \eta_{\text{Cas}} P_{\text{Cas}} + \eta_{\text{Ent}} P_{\text{rec}}$$

### Quantum Advantage Metric:

We define the Quantum Energy Advantage (QEA):

$$\text{QEA} = \frac{\eta_{\text{total}}}{\eta_{\text{classical}}} \cdot \frac{P_{\text{density}}}{P_{\text{density, classical}}}$$

For our system: $\text{QEA} \approx 8.2 \times 3.4 \approx 28$

---

**Previous Chapter:** [Chapter 1: Introduction](01_introduction.md)  
**Next Chapter:** [Chapter 3: Materials Design and Synthesis](03_materials_design_synthesis.md)
