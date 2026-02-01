# Appendix A: Mathematical Derivations

## A.1 Full Casimir Force Derivation

### Lifshitz Theory for Doped Graphene

Starting from the Casimir interaction energy in terms of reflection coefficients:

$$U(d) = \frac{\hbar c}{2\pi} \sum_{n=0}^{\infty} \!\!\!\!\!\!^{\prime} \int_0^\infty k_\perp dk_\perp q_n \left[ \ln(1 - r_\text{TE}^2 e^{-2q_n d}) + \ln(1 - r_\text{TM}^2 e^{-2q_n d}) \right]$$

For doped graphene with complex dielectric function:

$$\epsilon(\omega, k) = 1 + \frac{i\sigma(\omega, k)}{\omega \epsilon_0}$$

Where the conductivity includes:

$$\sigma(\omega, k) = \sigma_{\text{intra}}(\omega) + \sigma_{\text{inter}}(\omega) + \sigma_{\text{dopant}}(\omega, k)$$

### Dopant Contribution

For Li-N co-doping:

$$\sigma_{\text{dopant}} = \frac{e^2}{4\hbar} \sum_{\alpha \in \{Li, N\}} \left[ \frac{\mu_\alpha}{\hbar(\omega + i/\tau_\alpha)} + \text{higher order terms} \right]$$

The synergistic term arises from cross-dopant interactions:

$$\sigma_{\text{syn}} = \frac{\alpha_{\text{syn}} e^2}{4\hbar} \frac{\sqrt{\mu_{\text{Li}}\mu_{\text{N}}}}{\hbar(\omega + i/\tau_{\text{syn}})}$$

### Reflection Coefficients

For transverse electric (TE) modes:

$$r_\text{TE} = \frac{q_n - q_0}{q_n + q_0}$$

For transverse magnetic (TM) modes:

$$r_\text{TM} = \frac{\epsilon q_n - q_0}{\epsilon q_n + q_0}$$

Where $q_0 = i\omega/c$ and $q_n = \sqrt{k_\perp^2 + \xi_n^2/c^2}$ with $\xi_n = 2\pi n k_B T/\hbar$.

---

## A.2 Entanglement Measures for Energy States

### Directionality Parameter Definition

Given a two-mode photon state:

$$|\psi\rangle = \sum_{n_A, n_B} c_{n_A, n_B} |n_A\rangle_A \otimes |n_B\rangle_B$$

We define:
- Energy in mode A: $E_A = \hbar \omega_0 \langle n_A \rangle$
- Energy in mode B: $E_B = \hbar \omega_0 \langle n_B \rangle$

The directionality:

$$\mathcal{D} = \frac{E_A - E_B}{E_A + E_B} = \frac{\langle n_A \rangle - \langle n_B \rangle}{\langle n_A \rangle + \langle n_B \rangle}$$

### Entanglement Entropy

Subsystem B density matrix:

$$\rho_B = \text{Tr}_A(|\psi\rangle\langle\psi|)$$

Entanglement entropy:

$$S(\rho_B) = -\text{Tr}(\rho_B \ln \rho_B)$$

For NOON states: $S = \ln 2$

---

## A.3 Quantum Thermodynamic Cycle Efficiency

### System as Heat Engine

Three-stroke cycle:

1. **Isentropic:** Quantum gate (unitary, $S = 0$)
2. **Isothermal:** Measurement at $T_{\text{hot}}$
3. **Adiabatic:** Entropy change due to information

### Carnot-like Efficiency

For extended system with entanglement:

$$\eta = 1 - \frac{T_c}{T_h} - \frac{\Delta S_{\text{en}}}{k_B \ln 2} \cdot \frac{T_c}{T_h}$$

Where $\Delta S_{\text{en}}$ is the change in entanglement entropy.

For our system:
- $T_h = 300$ K, $T_c = 20$ mK
- $\Delta S_{\text{en}} = 0.5 k_B \ln 2$ (50% entanglement reduction)

$$\eta = 1 - \frac{0.02}{300} - 0.5 \times \frac{0.02}{300} = 0.9967$$

---

**See chapters for detailed application to specific systems.**
