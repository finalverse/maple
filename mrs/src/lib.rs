// MAPLE Registry Service for agent registration and DID management
// © 2025 Finalverse Inc. All rights reserved.

use maple_agents::AgentConfig;
use maple_map::{MapConfig, MapProtocol};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Configuration for the MRS
#[derive(Debug, Serialize, Deserialize)]
pub struct MrsConfig {
    map_config: MapConfig, // Configuration for underlying MAP Protocol
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredAgent {
    did: String, // Decentralized Identifier, e.g., "did:maple:agent:uuid"
    config: AgentConfig,
}

/// MAPLE Registry Service instance
pub struct Mrs {
    map: MapProtocol,
    agents: Vec<RegisteredAgent>,
    command_tx: mpsc::Sender<MrsCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MrsCommand {
    Register(AgentConfig), // Register a new agent
    GetAgent(String), // Retrieve agent by DID
}

impl Mrs {
    /// Initializes a new MRS instance
    pub async fn new(config: MrsConfig) -> Result<Self, Box<dyn Error>> {
        let map = MapProtocol::new(config.map_config).await?;
        let (command_tx, mut command_rx) = mpsc::channel(100);
        let mut agents = Vec::new();

        tokio::spawn(async move {
            while let Some(cmd) = command_rx.recv().await {
                match cmd {
                    MrsCommand::Register(config) => {
                        let did = format!("did:maple:agent:{}", Uuid::new_v4());
                        let agent = RegisteredAgent { did: did.clone(), config };
                        agents.push(agent);
                        println!("Registered agent: {}", did);
                    }
                    MrsCommand::GetAgent(did) => {
                        if let Some(agent) = agents.iter().find(|a| a.did == did) {
                            println!("Found agent: {:?}", agent);
                        }
                    }
                }
            }
        });

        Ok(Mrs {
            map,
            agents,
            command_tx,
        })
    }

    /// Registers a new agent and returns its DID
    pub async fn register_agent(&self, config: AgentConfig) -> Result<String, Box<dyn Error>> {
        let did = format!("did:maple:agent:{}", Uuid::new_v4());
        self.command_tx
            .send(MrsCommand::Register(config))
            .await?;
        Ok(did)
    }

    /// Retrieves an agent by DID
    pub async fn get_agent(&self, did: String) -> Result<RegisteredAgent, Box<dyn Error>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.command_tx.send(MrsCommand::GetAgent(did.clone())).await?;
        // Simulate retrieval (replace with actual storage lookup)
        let agent = self
            .agents
            .iter()
            .find(|a| a.did == did)
            .ok_or("Agent not found")?;
        tx.send(agent.clone()).unwrap();
        rx.await.map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_agent() {
        let map_config = MapConfig {
            listen_addr: "/ip4/127.0.0.1/tcp/0".to_string(),
        };
        let config = MrsConfig { map_config };
        let mrs = Mrs::new(config).await.unwrap();
        let agent_config = AgentConfig {
            name: "test-agent".to_string(),
            // Add more fields as needed
        };
        let did = mrs.register_agent(agent_config).await.unwrap();
        assert!(did.starts_with("did:maple:agent:"));
    }
}