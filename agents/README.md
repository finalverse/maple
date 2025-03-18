# MAPLE Agents

Agent implementations for the MAPLE ecosystem.

## Features
- Configurable agents with message processing.
- Integration with UAL for communication.

## Usage
```rust
use maple_agents::{Agent, AgentConfig};

let config = AgentConfig { name: "logistics-bot".to_string(), role: "logistics".to_string() };
let agent = Agent::new(config);
```

## Build
```bash
cargo build --release -p maple-agents
```