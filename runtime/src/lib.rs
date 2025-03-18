// Runtime environment core logic for MAPLE nodes
// © 2025 Finalverse Inc. All rights reserved.

use maple_agents::{Agent, AgentConfig};
use maple_map::{MapConfig, MapProtocol};
use maple_mrs::{Mrs, MrsConfig};
use mapledb::MapleDb;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;

/// Configuration for the runtime
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub mode: RuntimeMode,
    pub map_listen_addr: String, // e.g., "/ip4/0.0.0.0/tcp/0"
    pub db_path: String, // Path to MapleDB storage
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RuntimeMode {
    Distributed,
    Enterprise,
}

/// Runtime instance managing agents and network
pub struct Runtime {
    map: MapProtocol,
    mrs: Mrs,
    db: MapleDb,
    agents: Vec<Agent>,
    command_tx: mpsc::Sender<RuntimeCommand>,
}

#[derive(Debug)]
pub enum RuntimeCommand {
    SpawnAgent(String), // DID of agent to spawn
    Shutdown,
}

impl Runtime {
    /// Initializes a new runtime instance
    pub async fn new(config: RuntimeConfig) -> Result<Self, Box<dyn Error>> {
        let map_config = MapConfig {
            listen_addr: config.map_listen_addr.clone(),
        };
        let map = MapProtocol::new(map_config).await?;
        let mrs = Mrs::new(MrsConfig { map_config }).await?;
        let db = MapleDb::new(&config.db_path)?;

        let (command_tx, mut command_rx) = mpsc::channel(100);
        let mut agents = Vec::new();

        tokio::spawn(async move {
            while let Some(cmd) = command_rx.recv().await {
                match cmd {
                    RuntimeCommand::SpawnAgent(did) => {
                        // Placeholder: Load from MRS or .map file
                        let config = AgentConfig {
                            name: format!("agent-{}", did),
                            role: "default".to_string(),
                        };
                        let agent = Agent::new(config);
                        agents.push(agent);
                        println!("Spawned agent with DID: {}", did);
                    }
                    RuntimeCommand::Shutdown => {
                        println!("Shutting down runtime...");
                        break;
                    }
                }
            }
        });

        Ok(Runtime {
            map,
            mrs,
            db,
            agents,
            command_tx,
        })
    }

    /// Spawns an agent by DID
    pub async fn spawn_agent(&self, did: String) -> Result<(), Box<dyn Error>> {
        self.command_tx.send(RuntimeCommand::SpawnAgent(did)).await?;
        Ok(())
    }

    /// Shuts down the runtime
    pub async fn shutdown(&self) -> Result<(), Box<dyn Error>> {
        self.command_tx.send(RuntimeCommand::Shutdown).await?;
        Ok(())
    }
}