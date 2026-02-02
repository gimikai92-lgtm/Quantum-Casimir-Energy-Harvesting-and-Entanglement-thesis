Server (coherence-test)
========================

This project ships a small example binary `coherence-test` which can run in two modes:

- CLI informational runner (default)
- Lightweight server that returns a single JSON object per TCP connection (serve mode)

Usage
-----

Run in serve mode (localhost):

```bash
cargo run --bin coherence-test -- --serve
```

Bind to all interfaces (0.0.0.0):

```bash
cargo run --bin coherence-test -- --serve --bind-all
```

Specify a port via env or CLI:

```bash
PORT=7878 cargo run --bin coherence-test -- --serve
```

Enable TLS (requires certificates):

```bash
# Provide cert/key via env
TLS_CERT=./cert.pem TLS_KEY=./key.pem SERVE_TLS=1 PORT=7878 SERVE=1 cargo run --bin coherence-test

# Or use flags (defaults to cert.pem/key.pem)
cargo run --bin coherence-test -- --serve --tls --bind-all
```

JSON response
-------------

Each incoming TCP connection receives a single JSON object and then the connection is closed. Example:

```json
{
  "service": "quantum-coherence-test",
  "n_qubits": 2,
  "t2_times": [85.0, 85.0]
}
```

TLS notes
---------

- For development, generate a self-signed cert with OpenSSL:

```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout key.pem -out cert.pem -subj "/CN=localhost"
```

- Production-grade TLS requires proper certificate management. Consider using a reverse proxy (nginx, Caddy) or ACME for automated certs.

Next steps
----------

If you prefer proper HTTP(S) endpoints, I can integrate `hyper`/`warp` and return structured HTTP responses with headers, logging, and graceful shutdown support.