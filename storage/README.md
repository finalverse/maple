# MAPLE Storage

Data backends for the MAPLE ecosystem.

## Subcomponents
- `mapledb/`: Embedded key-value store for agent state.
- `pg/`: Structured data storage with PostgreSQL.
- `vectordb/`: Vector database for embeddings (placeholder).

## Usage
```rust
use mapledb::MapleDb;

let db = MapleDb::new("maple_data").unwrap();
db.store("agent1", b"data").unwrap();
```

## Notes
- Supports `.map` files for agent DNA storage via `maple-agents`.

## Build
```bash
cargo build --release -p mapledb
cargo build --release -p maple-pg
cargo build --release -p maple-vectordb
```