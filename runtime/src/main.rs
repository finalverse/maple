// runtime/src/main.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use clap::{Parser, Subcommand};
use tokio;
use ual::parse_ual;
use crate::distributed::DistributedNode;
use crate::enterprise::EnterpriseNode;

mod distributed;
mod enterprise;

// CLI structure for runtime executable
#[derive(Parser)]
#[command(name = "runtime", about = "MAPLE Node Server - © 2025 Finalverse Inc.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// CLI subcommands for node operations
#[derive(Subcommand)]
enum Commands {
    /// Start the node in Distributed Mode
    StartDistributed,
    /// Start the node in Enterprise Mode
    StartEnterprise,
    /// Send a UAL command
    Send { ual: String },
}

// Enum to hold either Distributed or Enterprise node
enum MapleNode {
    Distributed(DistributedNode),
    Enterprise(EnterpriseNode),
}

impl MapleNode {
    // Initialize a new MapleNode based on mode
    fn new(mode: &str) -> Self {
        match mode {
            "distributed" => MapleNode::Distributed(DistributedNode::new()),
            "enterprise" => MapleNode::Enterprise(EnterpriseNode::new()),
            _ => MapleNode::Distributed(DistributedNode::new()), // Default to Distributed
        }
    }

    // Start the node based on its type
    async fn start(&mut self) {
        match self {
            MapleNode::Distributed(node) => node.start().await,
            MapleNode::Enterprise(node) => node.start().await,
        }
    }

    // Send a UAL command based on node type
    async fn send_ual(&mut self, ual: &str) -> Result<(), String> {
        let stmt = parse_ual(ual)?;
        match self {
            MapleNode::Distributed(node) => node.send_ual(stmt).await,
            MapleNode::Enterprise(node) => node.send_ual(stmt).await,
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize the node based on the command
    let node = match &cli.command {
        Commands::StartDistributed => MapleNode::new("distributed"),
        Commands::StartEnterprise => MapleNode::new("enterprise"),
        Commands::Send { .. } => MapleNode::new("distributed"), // Default to Distributed for Send
    };
    let mut node = node;

    // Execute the CLI command
    match cli.command {
        Commands::StartDistributed | Commands::StartEnterprise => {
            node.start().await;
        }
        Commands::Send { ual } => {
            if let Err(e) = node.send_ual(&ual).await {
                eprintln!("Error sending UAL: {}", e);
            }
        }
    }
}