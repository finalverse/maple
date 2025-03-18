# MAPLE Core

Governance and LLM/AGI layer for MAPLE.

## Subcomponents
- `maple/`: Internal "Maple" LLM.
- `llm/`: External LLM integrations (e.g., Llama.cpp, Mistral).

## Usage
```rust
use maple_core::MapleCore;

let core = MapleCore::new();
core.monitor_agents();
```

