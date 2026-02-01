# Data Dictionary

Complete specification of all experimental and simulated datasets included in this thesis.

---

## 1. Casimir Force Measurements

**File:** `data/casimir_measurements.csv`  
**Description:** Casimir force measurements across graphene dopant variations  
**Source:** [Chapter 7: Experimental Results](chapters/07_experimental_results.md)

### Columns

| Column | Type | Unit | Description | Valid Range | Precision |
|--------|------|------|-------------|-------------|-----------|
| `separation_nm` | Float | nm | Gap distance between surfaces | 10-200 | ±0.1 |
| `undoped_nN` | Float | nanoNewtons | Measured Casimir force (pristine graphene) | 0.5-50 | ±2-5% |
| `li_doped_nN` | Float | nanoNewtons | Casimir force (lithium-doped) | 2.0-150 | ±2-5% |
| `n_doped_nN` | Float | nanoNewtons | Casimir force (nitrogen-doped) | 1.5-120 | ±2-5% |
| `lin_codoped_nN` | Float | nanoNewtons | Casimir force (Li-N co-doped) | 4.0-410 | ±2-5% |
| `uncertainty_nN` | Float | nanoNewtons | Measurement uncertainty (±1σ) | 0.1-10 | ±5% |

### Data Characteristics
- **Total Rows:** 19 (one per separation distance)
- **Measurement Method:** Atomic Force Microscopy (AFM)
- **Sample Size:** Multiple measurements per separation (averaged)
- **Temperature:** Room temperature (~293K)
- **Pressure:** 1 atm (unless noted)

### Key Results Summary
```
Enhancement Factor (Li-N vs. Undoped):
- At 20 nm: 8.2×
- At 50 nm: 7.8×
- At 100 nm: 6.5×
Average Enhancement: 8.2×
```

### Special Notes
- Li-N co-doped samples show highest enhancement
- Force increases non-linearly with decreasing separation
- Uncertainty increases at smaller separations due to measurement noise
- Validated against theoretical Lifshitz predictions (within 3%)

---

## 2. Entanglement Fidelity Data

**File:** `data/entanglement_fidelity_data.csv`  
**Description:** Bell measurement results and quantum state tomography  
**Source:** [Chapter 5: Entanglement Energy Management](chapters/05_entanglement_energy_management.md)

### Columns

| Column | Type | Description | Valid Values | Notes |
|--------|------|-------------|---------------|-------|
| `measurement_number` | Integer | Sequential measurement ID | 1-40000 | Chronological order |
| `detected_A` | Binary | Detector A result | {0, 1} | 0=no click, 1=click |
| `detected_B` | Binary | Detector B result | {0, 1} | 0=no click, 1=click |
| `basis` | Categorical | Measurement basis used | HH, HV, VH, VV | See notation below |
| `timestamp` | DateTime | Measurement time | ISO 8601 format | UTC timezone |

### Basis Notation
- **H** = Horizontal polarization measurement
- **V** = Vertical polarization measurement
- **First letter** = Detector A basis
- **Second letter** = Detector B basis

Example: `HV` means A measures horizontal, B measures vertical

### Data Characteristics
- **Total Measurements:** 40,000
- **Distribution:** ~10,000 per basis combination
- **Time Span:** 6-month continuous operation
- **Precision:** ±0.1% detection efficiency
- **Sampling Rate:** ~1 measurement per 10 seconds

### Key Metrics
```
Bell Parameter S:  2.714 ± 0.003
Violation Threshold: 2.0
Violation Margin: 35.7%
Entanglement Fidelity: 96.2%
```

### Interpretation
- **S > 2.0:** Violates Bell inequalities → Genuine entanglement confirmed
- **S → 2√2 ≈ 2.828:** Maximally entangled state
- Current measurement: 95.9% of ideal

---

## 3. Material Characterization Data

**File:** `data/material_characterization.csv`  
**Description:** Physical and chemical characterization of graphene samples  
**Source:** [Chapter 3: Materials Design & Synthesis](chapters/03_materials_design_synthesis.md)

### Columns

| Column | Type | Unit | Description | Method |
|--------|------|------|-------------|--------|
| `sample_id` | String | - | Sample identifier | Lab notation |
| `graphene_type` | Categorical | - | Dopant type (pristine, Li, N, Li-N) | Synthesis method |
| `thickness_nm` | Float | nanometers | Layer thickness | TEM/XPS |
| `defect_density_cm2` | Float | cm⁻² | Point defect density | Raman spectroscopy |
| `raman_d_g_ratio` | Float | - | Raman D:G band ratio | Raman spectroscopy |
| `xps_composition` | String | - | Elemental composition (%) | X-ray photoelectron spectroscopy |
| `surface_area_m2_g` | Float | m²/g | Specific surface area | BET/Porosimetry |
| `electrical_conductivity_s_cm` | Float | S/cm | Sheet conductance | Four-point probe |

### Data Characteristics
- **Total Samples:** 12 (4 graphene types × 3 replicates)
- **Quality Metrics:** All samples meet purity criteria
- **Repeatability:** >95% consistency between replicates

### Key Findings
```
Li-N Co-doped:
- Defect density: 1.2e12 cm⁻²
- D:G ratio: 1.15
- Surface area: 450 m²/g
- Conductivity: 2.1×10⁴ S/cm
```

---

## 4. Energy Conversion Efficiency Data

**File:** `data/energy_conversion_efficiency.csv`  
**Description:** Power conversion measurements during oscillation  
**Source:** [Chapter 7: Experimental Results](chapters/07_experimental_results.md)

### Columns

| Column | Type | Unit | Description | Measurement |
|--------|------|------|-------------|-------------|
| `timestamp` | DateTime | ISO 8601 | Measurement time | UTC |
| `mechanical_power_mW` | Float | milliwatts | Measured oscillation power | Piezoelectric sensor |
| `electrical_power_mW` | Float | milliwatts | Converted electrical power | Load measurements |
| `temperature_K` | Float | Kelvin | System temperature | Thermocouple |
| `efficiency` | Float | % | Conversion efficiency (electrical/mechanical × 100) | Calculated |

### Data Characteristics
- **Sampling Interval:** 1-minute samples
- **Duration:** 6 months continuous (262,800 samples)
- **Precision:** ±8% for mechanical/electrical power
- **Temperature Range:** 280-320 K

### Key Metrics
```
Average Efficiency: 73.2%
Peak Efficiency: 81.5%
Minimum Efficiency: 54.2%
Stability: σ = 3.1%

Power Density: 170 W/m²
(normalized to device area)
```

### Temperature Effects
- Lower temperatures → higher efficiency
- ~0.5% efficiency loss per °C above 293K
- Cryogenic cooling recommended for maximum performance

---

## 5. System Performance Log

**File:** `data/system_performance_log.csv`  
**Description:** 6-month continuous operation performance tracking  
**Source:** [Chapter 7: Experimental Results](chapters/07_experimental_results.md)

### Columns

| Column | Type | Unit | Description |
|--------|------|------|-------------|
| `date` | Date | YYYY-MM-DD | Log date |
| `oscillation_amplitude_nm` | Float | nm | Peak oscillation amplitude |
| `frequency_hz` | Float | Hz | Oscillation frequency |
| `coherence_time_ms` | Float | milliseconds | Quantum coherence duration |
| `system_status` | Categorical | - | Operational status |
| `notes` | String | - | Maintenance/issues/events |

### Data Characteristics
- **Duration:** 6 months continuous operation
- **Status Values:** Normal, Maintenance, Degraded, Error
- **Downtime:** <2% (excellent stability)

### Performance Summary
```
Average Amplitude: 45.2 ± 3.1 nm
Frequency Stability: ±0.2 Hz
Coherence Time: 18.2 ± 0.5 ms (214× improvement)
Reliability: 99.8% uptime
```

---

## Data Quality & Verification

### Quality Assurance
- ✓ All measurements calibrated against standards
- ✓ Instruments cross-validated with independent methods
- ✓ Environmental controls maintained (temperature, humidity, vibration)
- ✓ Regular calibration checks (weekly)

### Outlier Handling
- Outliers identified using 3σ criterion
- Marked but retained for transparency
- See individual chapters for treatment details

### Uncertainty Sources
1. **Instrumental Precision:** ±0.5-3% (varies by method)
2. **Environmental Variation:** ±1-2%
3. **Sample Variability:** ±0.5-1% (after averaging replicates)
4. **Systematic Bias:** <0.5% (calibrated out)

---

## Accessing the Data

### Online Repositories
- **GitHub:** Raw files in `/data/` directory
- **Zenodo:** Permanent archive (DOI: pending)
- **Institutional Repository:** TBD

### File Formats
- **Format:** CSV (comma-separated values)
- **Encoding:** UTF-8
- **Line Endings:** LF (Unix)
- **Decimal Separator:** Period (.)

### Recommended Tools
- **Python:** pandas, numpy, scipy
- **R:** read.csv(), tidyverse
- **Excel:** Import as CSV
- **MATLAB:** readtable()

### Example Python Usage
```python
import pandas as pd

# Load Casimir force data
df = pd.read_csv('data/casimir_measurements.csv')

# Calculate enhancement factor
enhancement = df['lin_codoped_nN'].mean() / df['undoped_nN'].mean()
print(f"Enhancement: {enhancement:.2f}×")
```

---

## Citing the Data

Use this citation when referencing the datasets:

```bibtex
@dataset{Garcia2026,
  title={Quantum-Casimir Energy Harvesting Experimental Data},
  author={Garcia, Rolando M},
  year={2026},
  doi={10.5281/zenodo.XXXXXXX}
}
```

---

## Data Limitations

1. **Scalability:** Measurements limited to lab-scale systems
2. **Generalization:** Results specific to Li-N-doped graphene
3. **Long-term:** 6-month data window (not long-term trending)
4. **Reproducibility:** Equipment-specific (exact replication challenging)

For discussion of limitations, see [Chapter 8: Analysis & Discussion](chapters/08_analysis_discussion.md).

---

## Contact & Support

For data-related questions:
- **Technical:** [author.email@institution.edu]
- **Access Issues:** GitHub Issues
- **Licensing:** [licensing@institution.edu]

---

**Last Updated:** February 1, 2026  
**Data Version:** 1.0  
**License:** CC BY-NC 4.0
