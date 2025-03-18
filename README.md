# MAPLE: Multi-Agent Protocol Learning Environment

MAPLE is a decentralized, scalable platform for creating, registering, and evolving AI agents. Built with Rust for performance and Python for flexibility, it powers a global ecosystem of self-governing agents via the MAP Protocol, Universal Agent Language (UAL), and Maple Agent Learning Lab (MALL).

## Copyright
© 2025 Finalverse Inc. All rights reserved.

## Contact
- Email: maple@finalverse.com
- Website: https://mapleai.org
- GitHub: https://github.com/finalverse/mapleai.git

## High-Level Components
- **core/**: Governance and Maple Core LLM/AGI.
- **map/**: P2P messaging protocol (MAP).
- **mrs/**: Registry for agent DIDs and "DNA."
- **ual/**: Multi-mode communication language (JSON, gRPC, byte-level).
- **mall/**: Learning environment for agent evolution.
- **agents/**: Pre-built and custom agent implementations.
- **storage/**: Data backends (MapleDB, PostgreSQL, VectorDB).
- **sdk/**: Tools for developers to build agents.
- **api/**: External REST/gRPC interfaces.
- **runtime/**: Distributed and enterprise node runtime.
- **cli/**: Command-line interface for MAPLE.

## Getting Started
1. Clone the repo: `git clone https://github.com/finalverse/maple.git`
2. Build: `cargo build --release`
3. Run CLI: `cargo run --bin maple -- --help`

## Sample Usage Scenarios

### For Developers
#### 1. Create and Register a Custom Agent
```bash
# Define agents "DNA" (config + LLM weights)
maple agents create --name "logistics-bot" --config logistics.json --llm mistral-7b

# Register with MRS
maple mrs register --agents "logistics-bot" --output did:maple:agents:1234
```
Build a logistics optimization agent, register it globally, and share it with the Mapleverse.

#### 2. Train an Agent in the Learning Lab
```bash
# Start MALL sandbox
maple mall start --task "supply-chain-optimization"

# Train agents
maple mall train --agents "logistics-bot" --iterations 1000
```
Evolve your agent’s capabilities in a simulated environment, then publish updates via MRS.

### For Businesses
#### 3. Deploy a Distributed Supply Chain Network
```bash
# Launch distributed runtime
maple runtime start --mode distributed --nodes 10

# Spawn registered agents
maple runtime spawn --did did:maple:agents:1234 --count 5
```
Run a decentralized network of logistics agents to streamline supply chain operations.

#### 4. Integrate with Enterprise Systems
```bash
# Start enterprise runtime
maple runtime start --mode enterprise --config enterprise.toml

# Query via API
curl -X GET "http://localhost:8080/agents/status" -H "Authorization: Bearer token"
```
Monitor and manage agents via REST API in a secure enterprise setup.

### For Researchers
#### 5. Experiment with UAL Communication
```rust
use maple::ual::{Message, Mode};

let msg = Message::new("move", Mode::Json)
    .with_payload(r#"{"x": 10, "y": 20}"#);
agent.send(msg);
```
Test agent communication in JSON mode, then switch to byte-level for optimization.

## Why MAPLE?
- **Decentralized:** No central authority—agents self-govern via MAP and MRS.
- **Scalable:** From edge devices to enterprise clusters.
- **Evolving:** Agents learn and adapt in MALL.
- **Developer-Friendly:** SDK and CLI accelerate adoption.

## Contributing
Fork, branch, and submit PRs! Join our Discord for discussions.

## Contact
- **Email:** maple@finalverse.com
- **Website:** https://mapleai.org
- **GitHub:** https://github.com/finalverse/mapleai.git

## License
© 2025 Finalverse Inc. All rights reserved.