# MAPLE: Multi-Agent Protocol Learning Environment

MAPLE is a collection of Rust crates (with optional Python bindings) for building distributed networks of AI agents. Agents communicate through the MAP protocol, exchange structured messages using UAL and can evolve in the Maple Agent Learning Lab (MALL). The project aims to provide a scalable, decentralized platform for autonomous systems.

## Repository Layout

- `agents/` – Core agent implementation and utilities for handling `.map` DNA files.
- `api/` – REST API with JWT authentication for spawning and managing agents.
- `cli/` – Command line interface built with `clap`.
- `core/` – Governance logic and language model integration.
- `map/` – P2P communication layer using `libp2p`.
- `mall/` – Agent learning lab with optional Python agents.
- `mrs/` – Registry Service for DIDs and agent metadata.
- `runtime/` – Node runtime supporting distributed or enterprise modes.
- `sdk/` – Rust/Python SDK for developers.
- `storage/` – MapleDB, PostgreSQL and vector DB crates.
- `ual/` – Universal Agent Language message format.

See `docs/architecture.md` for a detailed overview of how these pieces connect.

## Quick Start

```bash
# build the entire workspace
cargo build --release

# run the CLI
cargo run --bin maple -- --help
```

### Spawning an Agent

```bash
# create an agent and dump its DNA
maple agent create --name "logistics-bot" --role "logistics"

# register with the registry service
maple mrs register --name "logistics-bot"

# start a local runtime and spawn the agent
maple runtime start --mode distributed --nodes 1
maple runtime spawn --did "did:maple:agent:1234"
```

### Using the SDK

```rust
use maple_sdk::{MapleSdk, SdkConfig};

let config = SdkConfig {
    api_url: "http://localhost:8080".to_string(),
    api_key: "test-key".to_string(),
    map_listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
    db_path: "maple_sdk_db".to_string(),
};
let sdk = MapleSdk::new(config).await.unwrap();
let did = sdk.create_agent("logistics-bot", "logistics").await.unwrap();
```

## Documentation

Additional documents can be found in the `docs/` directory:

- `architecture.md` – overview of the system design.
- `MAP.md` – details of the MAP peer-to-peer protocol.
- `AUL.md` – specification of the Universal Agent Language.
- `TODO.md` – planned improvements.

## Contributing

Pull requests are welcome! Feel free to open issues or propose new agents and integrations.

## Contact

- Email: [maple@finalverse.com](mailto:maple@finalverse.com)
- Website: <https://mapleai.org>
- GitHub: <https://github.com/finalverse/mapleai.git>

© 2025 Finalverse Inc. All rights reserved.
