# MAPLE: Multi-Agent Protocol Learning Environment

MAPLE is a scalable, multi-agent AI ecosystem for distributed and enterprise-grade applications.

## Copyright
© 2025 Finalverse Inc. All rights reserved.

## Contact
- Email: maple@finalverse.com
- Website: https://mapleai.org
- GitHub: https://github.com/finalverse/mapleai.git

## Components
- `maple-node`: Executable for MAPLE Node (Distributed & Enterprise Modes)
- `maple`: Main executable with CLI
- `api`: REST API service
- `maple-core`: Core MAPLE logic
- `config`: Configuration management
- `sdk`: MapleAI SDK
- `agent`: AI Agent implementations
- `map`: Multi-Agent Protocol (MAP)
- `ual`: Universal Agent Language (UAL)
- `llm`: Internal LLM components
- `storage/*`: Data storage layers

## Usage
- Build: `cargo build --workspace`
- Run CLI: `cargo run --package maple -- --help`
- Run Node: `cargo run --package maple-node -- --help`