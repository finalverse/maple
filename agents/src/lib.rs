// Agent implementations for the MAPLE ecosystem
// © 2025 Finalverse Inc. All rights reserved.

use maple_ual::{UalMessage, Mode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;

/// Configuration for an agent
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub role: String, // e.g., "logistics", "research"
}

/// Represents a MAPLE agent
#[derive(Debug)]
pub struct Agent {
    config: AgentConfig,
    message_rx: mpsc::Receiver<UalMessage>,
    message_tx: mpsc::Sender<UalMessage>,
}

impl Agent {
    /// Creates a new agent instance
    pub fn new(config: AgentConfig) -> Self {
        let (tx, rx) = mpsc::channel(100);
        let agent = Agent {
            config,
            message_rx: rx,
            message_tx: tx,
        };
        tokio::spawn(agent.run());
        agent
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
                // Example: decode custom byte payload
                let coords = String::from_utf8(msg.payload.clone())?;
                Ok(format!(
                    "Agent {} handled byte-level {}: {}",
                    self.config.name, msg.action, coords
                ))
            }
            Mode::Grpc => unimplemented!("gRPC processing not yet implemented"),
        }
    }

    /// Sends a message to the agent
    pub async fn send(&self, msg: UalMessage) -> Result<(), Box<dyn Error>> {
        self.message_tx.send(msg).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_process() {
        let config = AgentConfig {
            name: "test-agent".to_string(),
            role: "test".to_string(),
        };
        let agent = Agent::new(config);
        let msg = UalMessage::new("move", Mode::Json)
            .with_json_payload(&serde_json::json!({"x": 10}))
            .unwrap();
        agent.send(msg).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Wait for processing
    }
}