# MAPLE CLI

Command-line interface for interacting with MAPLE.

## Features
- Create and dump agents to `.map` files.
- Register agents with MRS.
- Start runtime in distributed or enterprise mode.
- Spawn agents in the runtime.

## Usage
```bash
# Create an agent
maple agent create --name "logistics-bot" --role "logistics"

# Register an agent
maple mrs register --name "logistics-bot"

# Start runtime
maple runtime start --mode distributed --nodes 10

# Spawn an agent
maple runtime spawn --did "did:maple:agent:1234"
```

## Build
```bash
cargo build --release -p maple-cli
```