# MAPLE Runtime

Runtime environment for MAPLE nodes in distributed and enterprise modes.

## Features
- Supports distributed and enterprise deployment modes.
- Manages agent spawning and network communication.

## Usage
```rust
use maple_runtime::{RuntimeConfig, RuntimeMode};

let config = RuntimeConfig {
    mode: RuntimeMode::Distributed,
    map_listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
    db_path: "maple_db".to_string(),
};
let runtime = Runtime::new(config).await.unwrap();
runtime.spawn_agent("did:maple:agent:1234".to_string()).await.unwrap();