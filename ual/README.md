# MAPLE Universal Agent Language (UAL)

Universal Agent Language (UAL) for multi-mode agent communication.

## Features
- Supports JSON, gRPC, and byte-level communication modes.
- Flexible encoding/decoding for agent interactions.

## Usage
```rust
use maple_ual::{UalMessage, Mode};

let msg = UalMessage::new("move", Mode::Json)
    .with_json_payload(&serde_json::json!({"x": 10, "y": 20})).unwrap();
let payload: serde_json::Value = msg.decode().unwrap();