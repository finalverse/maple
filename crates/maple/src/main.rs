// crate/maple/src/main.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use clap::{Parser, Subcommand, Args};
use tokio;
use std::process::Command;

#[derive(Parser)]
#[command(name = "maple", about = "MAPLE CLI - © 2025 Finalverse Inc.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display MAPLE CLI information
    Info,
    /// Node control commands
    Node(NodeArgs),
}

#[derive(Args)]
struct NodeArgs {
    #[command(subcommand)]
    command: NodeCommands,
}

#[derive(Subcommand)]
enum NodeCommands {
    /// Start the MAPLE Node
    Start {
        #[arg(long, default_value = "distributed")]
        mode: String, // "distributed" or "enterprise"
    },
    /// Send a UAL command to the node
    Send { ual: String },
    Import { path: String }, // New command
    Export { name: String, path: String }, // New command
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info => {
            println!("MAPLE CLI - Version 0.1.0");
            println!("© 2025 Finalverse Inc.");
            println!("Email: maple@finalverse.com");
            println!("Website: https://mapleai.org");
            println!("GitHub: https://github.com/finalverse/mapleai.git");
        }
        Commands::Node(args) => match args.command {
            NodeCommands::Start { mode } => {
                let mode_arg = match mode.as_str() {
                    "distributed" => "start-distributed",
                    "enterprise" => "start-enterprise",
                    _ => {
                        eprintln!("Invalid mode: {}. Use 'distributed' or 'enterprise'", mode);
                        return;
                    }
                };
                let status = Command::new("cargo")
                    .args(["run", "--package", "maple-node", "--", mode_arg])
                    .status()
                    .expect("Failed to start node");
                if !status.success() {
                    eprintln!("Node failed to start");
                }
            }
            NodeCommands::Send { ual } => {
                let status = Command::new("cargo")
                    .args(["run", "--package", "maple-node", "--", "send", &ual])
                    .status()
                    .expect("Failed to send UAL");
                if !status.success() {
                    eprintln!("Failed to send UAL");
                }
            }
            NodeCommands::Import { path } => {
                println!("Importing package from {}", path);
                // TODO: Integrate with running node (future step)
            }
            NodeCommands::Export { name, path } => {
                println!("Exporting package {} to {}", name, path);
                // TODO: Integrate with running node (future step)
            }
        },
    }
}