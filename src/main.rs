mod ual;

use clap::{Parser, Subcommand};
use tokio;
use ual::{UALStatement, parse_ual};

#[derive(Parser)]
#[command(name = "maple_node", about = "MAPLE Node CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the MAPLE Node
    Start,
    /// Send a UAL command
    Send { ual: String },
}

struct MapleNode {
    running: bool,
}

impl MapleNode {
    fn new() -> Self {
        MapleNode { running: false }
    }

    async fn start(&mut self) {
        self.running = true;
        println!("MAPLE Node started.");
        // Placeholder for async loop
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }

    async fn send_ual(&self, ual: &str) -> Result<(), String> {
        let stmt = parse_ual(ual)?;
        println!("Processed UAL: {:?}", stmt);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut node = MapleNode::new();

    match cli.command {
        Commands::Start => node.start().await,
        Commands::Send { ual } => {
            if let Err(e) = node.send_ual(&ual).await {
                eprintln!("Error: {}", e);
            }
        }
    }
}

/*
Build and run:
    cargo run -- start
    Output: MAPLE Node started.

Test UAL command:
    cargo run -- send "EXEC task1 agent1 WITH priority=5"
    Output: Processed UAL: UALStatement { type_: "EXEC", target: "task1", destination: "agent1", params: [("priority", "5")] }
 */