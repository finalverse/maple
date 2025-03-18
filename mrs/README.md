# MAPLE Registry Service (MRS)

MAPLE Registry Service for agent registration and DID management.

## Features
- Register agents with unique DIDs.
- Retrieve agent configurations by DID.

## Usage
```rust
use maple_mrs::{Mrs, MrsConfig};
use maple_map::MapConfig;

let map_config = MapConfig { listen_addr: "/ip4/0.0.0.0/tcp/0".to_string() };
let mrs = Mrs::new(MrsConfig { map_config }).await.unwrap();
let did = mrs.register_agent(AgentConfig { name: "logistics-bot".to_string() }).await.unwrap();
```

## Build
```bash
cargo build --release -p maple-mrs
```