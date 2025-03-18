# MAPLE API

Secure RESTful API for the MAPLE ecosystem.

## Features
- Access key authentication with JWT.
- Tier-based restrictions (free vs. paid users).
- Agent spawning endpoint.

## Usage
```bash
curl -X POST http://localhost:8080/agents/spawn \
  -H "Authorization: paid-key" \
  -d @logistics.map
```

## Configuration
- Set `API_SECRET_KEY` env var for JWT signing.

## Build and Run
```bash
cargo build --release -p maple-api
cargo run --release -p maple-api
```