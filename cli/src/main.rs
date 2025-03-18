// Command-line interface for interacting with MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use clap::{Parser, Subcommand};
use maple_agents::{Agent, AgentConfig};
use maple_mrs::{Mrs, MrsConfig};
use maple_runtime::{Runtime, RuntimeConfig, RuntimeMode};
use std::error::Error;
use tokio;

#[derive(Parser)]
#[command(name = "maple", about = "MAPLE CLI", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new agent
    AgentCreate {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        role: String,
    },
    /// Registers an agent with MRS
    MrsRegister {
        #[arg(short, long)]
        name: String,
    },
    /// Starts the runtime
    RuntimeStart {
        #[arg(short, long, default_value = "distributed")]
        mode: String,
        #[arg(short, long, default_value = "1")]
        nodes: usize,
    },
    /// Spawns an agent in the runtime
    RuntimeSpawn {
        #[arg(short, long)]
        did: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::AgentCreate { name, role } => {
            let config = AgentConfig { name, role };
            let agent = Agent::new(config.clone());
            agent.dump_to_map(&format!("{}.map", config.name)).await?;
            println!("Created agent: {}.map", config.name);
        }
        Commands::MrsRegister { name } => {
            let map_config = maple_map::MapConfig {
                listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
            };
            let mrs = Mrs::new(MrsConfig { map_config }).await?;
            let config = AgentConfig {
                name: name.clone(),
                role: "default".to_string(),
            };
            let did = mrs.register_agent(config).await?;
            println!("Registered agent {} with DID: {}", name, did);
        }
        Commands::RuntimeStart { mode, nodes } => {
            let runtime_mode = match mode.as_str() {
                "distributed" => RuntimeMode::Distributed,
                "enterprise" => RuntimeMode::Enterprise,
                _ => return Err("Invalid mode".into()),
            };
            let config = RuntimeConfig {
                mode: runtime_mode,
                map_listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
                db_path: "maple_cli_db".to_string(),
            };
            let runtime = Runtime::new(config).await?;
            println!("Started runtime in {} mode with {} nodes", mode, nodes);
            tokio::signal::ctrl_c().await?;
            runtime.shutdown().await?;
        }
        Commands::RuntimeSpawn { did } => {
            let config = RuntimeConfig {
                mode: RuntimeMode::Distributed, // Default for CLI simplicity
                map_listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
                db_path: "maple_cli_db".to_string(),
            };
            let runtime = Runtime::new(config).await?;
            runtime.spawn_agent(did.clone()).await?;
            println!("Spawned agent with DID: {}", did);
            tokio::signal::ctrl_c().await?;
            runtime.shutdown().await?;
        }
    }

    Ok(())
}