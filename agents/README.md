# MAPLE Agents

Agent implementations for the MAPLE ecosystem.

## Features
- Configurable agents with message processing.
- Integration with UAL for communication.
- DNA data dumping to `.map` files for transport and spawning.

## Usage
```rust
use maple_agents::{Agent, AgentConfig};

let config = AgentConfig { name: "logistics-bot".to_string(), role: "logistics".to_string() };
let agent = Agent::new(config);
agent.dump_to_map("logistics.map").await.unwrap();
let spawned = Agent::from_map_file("logistics.map").await.unwrap();
```

## Build
```bash
cargo build --release -p maple-agents
```