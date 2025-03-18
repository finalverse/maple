# MAPLE SDK (Rust)

Rust SDK for interacting with the MAPLE ecosystem.

## Features
- Create and manage agents.
- Send messages via MAP Protocol.
- Python SDK available via `pip install maple-sdk` (see below).

## Usage
```rust
use maple_sdk::{MapleSdk, SdkConfig};

let config = SdkConfig {
    api_endpoint: "http://localhost:8080".to_string(),
    access_key: "your-access-key".to_string(),
};
let sdk = MapleSdk::new(config).await.unwrap();
let agent = sdk.create_agent("logistics-bot", "logistics").await.unwrap();
```

## Build
```bash
cargo build --release -p maple-sdk
```

## Publishing
```bash
cargo publish -p maple-sdk
```

## Python SDK
Install via pip:
```bash
pip install maple-sdk
```
See `python/` directory for details (coming soon).
```