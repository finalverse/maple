# MAPLE SDK

Rust and Python SDK for interacting with the MAPLE ecosystem.

## Features
- Create and register agents.
- Spawn agents via API.
- Send UAL messages over the network.
- Publishable as Rust crate and Python pip package.

## Rust Usage
```rust
use maple_sdk::{MapleSdk, SdkConfig};

let config = SdkConfig {
    api_url: "http://localhost:8080".to_string(),
    api_key: "your-api-key".to_string(),
    map_listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
    db_path: "maple_sdk_db".to_string(),
};
let sdk = MapleSdk::new(config).await.unwrap();
let did = sdk.create_agent("logistics-bot", "logistics").await.unwrap();
```

## Python Usage
```python
from maple_sdk import PyMapleSdk

sdk = PyMapleSdk("http://localhost:8080", "your-api-key", "/ip4/0.0.0.0/tcp/0", "maple_sdk_db")
did = sdk.create_agent("logistics-bot", "logistics")
print(f"Created agent with DID: {did}")
```

## Build (Rust)
```bash
cargo build --release -p maple-sdk
# For Python bindings
cargo build --release -p maple-sdk --features python
```

## Publish

- Rust: `cargo publish -p maple-sdk`
- Python: `python setup.py sdist bdist_wheel && twine upload dist/*`