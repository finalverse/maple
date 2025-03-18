# MAPLE MAP Protocol

Multi-Agent Protocol (MAP) for decentralized P2P messaging.

## Features
- Peer discovery via mDNS.
- P2P messaging with `libp2p`.

## Usage
```rust
use maple_map::{MapConfig, MapProtocol};

let config = MapConfig { listen_addr: "/ip4/0.0.0.0/tcp/0".to_string() };
let map = MapProtocol::new(config).await.unwrap();
map.broadcast("Hello, Mapleverse!".to_string()).await.unwrap();
```

## Build
```bash
cargo build --release -p maple-map
```