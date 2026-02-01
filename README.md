# quantum-coherence-test


Lightweight wrapper project for the Quantum Coherence Testing Framework.

Quick start

Build (debug):

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run the example binary:

```bash
cargo run --bin coherence-test
```

Project structure

- `Cargo.toml` - Rust manifest for the crate
- `src/lib.rs` - Library code (minimal implementation)
- `src/bin/main.rs` - Small CLI example binary
- `tests/` - Test files (`tests/unit.rs`)
- `python/` - Python helpers and scripts
- `scripts/` - Helper scripts (e.g. `run_tests.sh`)
- `Dockerfile`, `docker-compose.yml` - Containerization
- `.github/workflows/ci.yml` - CI workflow

Notes

This repository contains a minimal, deterministic implementation so the test-suite and CI can run quickly. If you want to restore the full research implementation, provide the original `src/` modules and I can help re-integrate them.

Secrets and API keys

- Do NOT commit real API keys. Use the `.env.example` as a template and store secrets in a local `.env` file that is ignored by Git.
- For CI (GitHub Actions) set repository secrets under Settings → Secrets and reference them (the workflow already reads `SERVICE_API_KEY`, `S3_ACCESS_KEY_ID`, `S3_SECRET_ACCESS_KEY`, `DOCKERHUB_TOKEN`).
- Example local flow:

```bash
cp .env.example .env
# edit .env and fill values
export $(grep -v '^#' .env | xargs)
```

When adding third-party services, add only placeholder names in `.env.example` and put real values either in `.env` locally or in GitHub Secrets for CI.

Optional fast-fail validation

Set `REQUIRE_SECRETS=1` in your environment (or in CI) to make the example binary fail-fast when common secrets are missing. This is helpful for CI jobs that must not run without required credentials.
# Quantum-Casimir Energy Harvesting and Entanglement-Directed Energy Management

**Doctoral Thesis** | **Rolando M Garcia** | **Department of Quantum Physics** | **February 1, 2026**

## 📋 Overview

This thesis presents a comprehensive framework for **energy-autonomous quantum computing** through the synergistic integration of three groundbreaking technologies.

## 🔬 Key Achievements

- **8.2× Enhanced Casimir Force** in Li-N-graphene
- **18.2 ms Coherence Time** (214× improvement)
- **170 W/m² Power Density** from quantum vacuum fluctuations
- **96% Directionality Efficiency** (exceeding 50% classical limit)
- **3.2× Landauer Limit Performance**

## 📚 Repository Structure

```
├── chapters/                 # 10 thesis chapters
├── appendices/              # Mathematical derivations & protocols
├── code/                    # Simulations and algorithms
├── data/                    # Experimental measurements
└── figures/                 # Visualizations and charts
```

## 📖 Citation

```bibtex
@phdthesis{Garcia2026,
  author = {Rolando M Garcia},
  title = {Quantum-Casimir Energy Harvesting and Entanglement-Directed Energy Management},
  school = {[Institution Name]},
  year = {2026}
}
```

## 📜 License - AUTHOR-PROTECTIVE

**Proprietary Author-Protective Licensing Model**

| Component | License | Key Restrictions |
|-----------|---------|-----------------|
| **Code** | GNU AGPL-3.0 | Source required for network use; commercial license needed |
| **Thesis Content** | CC BY-NC-ND 4.0 | ⚠️ **NO commercial use, NO derivatives** |
| **Experimental Data** | CC BY-NC 4.0 | NO commercial use allowed |
| **Patents** | Patent-pending | Cannot be implemented without license |

⚠️ **COMMERCIAL USE STRICTLY PROHIBITED** without explicit written permission.

See [LICENSE](LICENSE) and [COPYRIGHT](COPYRIGHT) for complete terms, enforcement, and licensing options.

**Copyright © 2026 Rolando M Garcia. All rights reserved.**

## 🔐 Authentication & Integrity

**Date of Completion:** February 1, 2026  
**Digital Signature:** Verified via cryptographic checksums  
**Git Commit:** 59a739991676f9eaa788dbe1513bc579d6f170d8  
**Timestamp:** 2026-02-01T00:00:00Z

For integrity verification, see [.thesis_signatures.txt](.thesis_signatures.txt)