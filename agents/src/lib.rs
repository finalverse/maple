// Agent implementations for the MAPLE ecosystem
// © 2025 Finalverse Inc. All rights reserved.

use maple_ual::{UalMessage, Mode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

/// Configuration for an agent
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub role: String, // e.g., "logistics", "research"
}

/// Represents a MAPLE agent with DNA data
#[derive(Debug)]
pub struct Agent {
    did: String, // Decentralized Identifier
    config: AgentConfig,
    state: Vec<u8>, // Placeholder for agent state (e.g., memory, weights)
    message_rx: mpsc::Receiver<UalMessage>,
    message_tx: mpsc::Sender<UalMessage>,
}

impl Agent {
    /// Creates a new agent instance with a unique DID
    pub fn new(config: AgentConfig) -> Self {
        let did = format!("did:maple:agent:{}", Uuid::new_v4());
        let (tx, rx) = mpsc::channel(100);
        let agent = Agent {
            did,
            config,
            state: Vec::new(), // Initial empty state
            message_rx: rx,
            message_tx: tx,
        };
        tokio::spawn(agent.clone().run());
        agent
    }

    /// Spawns an agent from a .map file
    pub async fn from_map_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        // Validate header
        if &buffer[0..8] != b"MAPLEDNA" {
            return Err("Invalid .map file header".into());
        }

        // Parse version (skip for now)
        let version = u16::from_be_bytes([buffer[8], buffer[9]]);

        // Extract DID
        let did = String::from_utf8(buffer[10..46].to_vec())?;

        // Extract config
        let config_len = u32::from_be_bytes([buffer[46], buffer[47], buffer[48], buffer[49]]) as usize;
        let config_data = &buffer[50..50 + config_len];
        let config: AgentConfig = serde_json::from_slice(config_data)?;

        // Extract state
        let state_start = 50 + config_len;
        let state_len = u32::from_be_bytes([
            buffer[state_start],
            buffer[state_start + 1],
            buffer[state_start + 2],
            buffer[state_start + 3],
        ]) as usize;
        let state = buffer[state_start + 4..state_start + 4 + state_len].to_vec();

        let (tx, rx) = mpsc::channel(100);
        let agent = Agent {
            did,
            config,
            state,
            message_rx: rx,
            message_tx: tx,
        };
        tokio::spawn(agent.clone().run());
        Ok(agent)
    }

    /// Runs the agent's main loop
    async fn run(self) {
        let mut rx = self.message_rx;
        while let Some(msg) = rx.recv().await {
            match self.process_message(&msg) {
                Ok(response) => println!("Agent {} processed: {:?}", self.config.name, response),
                Err(e) => eprintln!("Agent {} error: {}", self.config.name, e),
            }
        }
    }

    /// Processes a UAL message
    fn process_message(&self, msg: &UalMessage) -> Result<String, Box<dyn Error>> {
        match msg.mode {
            Mode::Json => {
                let payload: serde_json::Value = msg.decode()?;
                Ok(format!(
                    "Agent {} handled {} with payload: {}",
                    self.config.name, msg.action, payload
                ))
            }
            Mode::ByteLevel => {
                if &msg.payload[0..8] == b"MAPLEDNA" {
                    // Recognize .map data broadcasted via UAL
                    Ok(format!("Agent {} recognized .map data", self.config.name))
                } else {
                    let coords = String::from_utf8(msg.payload.clone())?;
                    Ok(format!(
                        "Agent {} handled byte-level {}: {}",
                        self.config.name, msg.action, coords
                    ))
                }
            }
            Mode::Grpc => unimplemented!("gRPC processing not yet implemented"),
        }
    }

    /// Sends a message to the agent
    pub async fn send(&self, msg: UalMessage) -> Result<(), Box<dyn Error>> {
        self.message_tx.send(msg).await?;
        Ok(())
    }

    /// Dumps agent DNA to a .map file
    pub async fn dump_to_map(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path).await?;
        let config_data = serde_json::to_vec(&self.config)?;
        let config_len = config_data.len() as u32;
        let state_len = self.state.len() as u32;

        // Write header
        file.write_all(b"MAPLEDNA").await?; // 8 bytes
        file.write_all(&1u16.to_be_bytes()).await?; // Version 1.0 (2 bytes)

        // Write DID (fixed 36 bytes, padded if needed)
        let mut did_bytes = self.did.as_bytes().to_vec();
        did_bytes.resize(36, 0);
        file.write_all(&did_bytes).await?;

        // Write config length and data
        file.write_all(&config_len.to_be_bytes()).await?;
        file.write_all(&config_data).await?;

        // Write state length and data
        file.write_all(&state_len.to_be_bytes()).await?;
        file.write_all(&self.state).await?;

        file.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_dump_and_spawn() {
        let config = AgentConfig {
            name: "test-agent".to_string(),
            role: "test".to_string(),
        };
        let agent = Agent::new(config);
        agent.dump_to_map("test_agent.map").await.unwrap();

        let spawned_agent = Agent::from_map_file("test_agent.map").await.unwrap();
        assert_eq!(agent.config.name, spawned_agent.config.name);
        assert_eq!(agent.did, spawned_agent.did);

        // Cleanup
        tokio::fs::remove_file("test_agent.map").await.unwrap();
    }
}