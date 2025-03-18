# MAPLE API

REST and gRPC API for the MAPLE ecosystem with access key security.

## Features
- Secure endpoints with access keys.
- Differentiates between free and paid users.

## Usage
```bash
curl -X POST "http://localhost:8080/agents/register" \
     -H "Authorization: Bearer your-access-key" \
     -d '{"name": "logistics-bot", "role": "logistics"}'
```

## Build
```bash
cargo build --release -p maple-api
```