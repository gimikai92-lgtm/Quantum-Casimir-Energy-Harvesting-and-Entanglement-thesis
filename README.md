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

Build and install Python extension (optional):

```bash
# create a virtualenv and activate it
python -m venv .venv
source .venv/bin/activate
# install maturin and build/install the extension
python -m pip install --upgrade pip setuptools wheel maturin
maturin develop --release --features python
# then run python tests
python -m pytest python/tests

Troubleshooting
--------------
- If `maturin` fails complaining about a missing virtualenv, create one with `python -m venv .venv` and activate it before running `maturin develop`.
- On some systems `maturin` may try to generate cffi bindings or fail while parsing C macros. Installing `cffi` in the virtualenv (`pip install cffi`) or upgrading `pip`/`setuptools` usually resolves this.
- CI will build and install the extension automatically (see `.github/workflows/ci.yml`). If you still see tests skipped locally, the extension probably isn't installed in your current interpreter; use the steps above to install it.
```

Run the example binary:

```bash
cargo run --bin coherence-test
```

Serve mode (TCP JSON responses / optional TLS)
---------------------------------------------

The example binary can run as a lightweight server that returns JSON-formatted status information.

- Start serve mode (binds to localhost by default):

```bash
cargo run --bin coherence-test -- --serve
```

- Bind to all interfaces (0.0.0.0):

```bash
cargo run --bin coherence-test -- --serve --bind-all
```

- Use an explicit port (or set `PORT` env):

```bash
PORT=7878 cargo run --bin coherence-test -- --serve
```

- Enable TLS (requires `cert.pem` and `key.pem` or set `TLS_CERT`/`TLS_KEY`):

```bash
# environment-driven TLS
TLS_CERT=./cert.pem TLS_KEY=./key.pem SERVE_TLS=1 PORT=7878 SERVE=1 cargo run --bin coherence-test

# or using flags
cargo run --bin coherence-test -- --serve --tls --bind-all
```

The server returns a single JSON object per TCP connection, for example:

```json
{
  "service": "quantum-coherence-test",
  "n_qubits": 2,
  "t2_times": [85.0, 85.0]
}
```

TLS certificate/key generation (development)
------------------------------------------

For development you can generate a self-signed certificate with OpenSSL:

```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout key.pem -out cert.pem -subj "/CN=localhost"
```

Notes
-----
- The serve mode is intentionally minimal (JSON-over-TCP). If you need a full HTTP/HTTPS server, I can add `hyper`/`warp` integration.
- Defaults: binds `127.0.0.1` and uses `cert.pem`/`key.pem` when TLS is enabled unless `TLS_CERT`/`TLS_KEY` are set.


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