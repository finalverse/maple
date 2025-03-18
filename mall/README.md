# MAPLE Agent Learning Lab (MALL)

Maple Agent Learning Lab (MALL) for agent evolution.

## Features
- Simulated environment for agent training.
- Integration with Python-based agents via `mpy/`.

## Usage
```rust
use maple_mall::{Mall, MallConfig};

let config = MallConfig { task: "supply-chain".to_string(), iterations: 1000 };
let mall = Mall::new(config).await.unwrap();
mall.train("supply-chain".to_string(), 1000).await.unwrap();
```

## Build
```bash
cargo build --release -p maple-mall
```