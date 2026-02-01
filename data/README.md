# Data Directory

This directory contains experimental measurements and analysis results.

## Files

### Raw Measurements

- `casimir_measurements.csv` - Casimir force vs. separation for all graphene types
- `entanglement_fidelity_data.csv` - Bell measurement results and quantum state tomography
- `material_characterization.csv` - XPS, Raman, TEM data
- `energy_conversion_efficiency.csv` - Power conversion measurements

### Analysis Results

- `system_performance_log.csv` - 6-month continuous operation logs

## Data Access

**Online Repository:**
- Zenodo DOI: 10.5281/zenodo.XXXXXXXXX
- GitHub: https://github.com/gimikai92-lgtm/quantum-energy-thesis

**License:** CC BY 4.0

## Data Specifications

### Casimir Force Data
- **Columns:** separation_nm, undoped_nN, li_doped_nN, n_doped_nN, lin_codoped_nN, uncertainty_nN
- **Range:** 10-200 nm separation
- **Precision:** ±2-5%

### Entanglement Data  
- **Columns:** measurement_number, detected_A, detected_B, basis, timestamp
- **Count:** 40,000 measurements
- **Precision:** ±0.1%

### Energy Efficiency Data
- **Columns:** timestamp, mechanical_power_mW, electrical_power_mW, temperature_K, efficiency
- **Duration:** 1 minute samples
- **Precision:** ±8%

## Citation

If using this data:

```bibtex
@dataset{Garcia2026,
  title={Quantum-Casimir Energy Harvesting Experimental Data},
  author={Garcia, Rolando M},
  year={2026},
  doi={10.5281/zenodo.XXXXXXXXX}
}
```

## Processing Scripts

Analysis scripts in `/code/` directory:

- `analyze_casimir_force.m` - Casimir force analysis
- `reconstruct_entanglement.py` - State reconstruction
- `compute_efficiency.py` - Efficiency calculations

## Questions?

Contact: [author.email@institution.edu]

---

**Data Integrity:** All files verified with SHA-256 checksums  
**Last Updated:** February 1, 2026
