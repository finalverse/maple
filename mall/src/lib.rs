// Maple Agent Learning Lab (MALL) for agent evolution in MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use maple_mpy::PythonAgent;
use maple_ual::{UalMessage, Mode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;

/// Configuration for the MALL
#[derive(Debug, Serialize, Deserialize)]
pub struct MallConfig {
    task: String, // e.g., "supply-chain-optimization"
    iterations: u32,
}

/// Learning Lab instance
pub struct Mall {
    agents: Vec<PythonAgent>,
    command_tx: mpsc::Sender<MallCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MallCommand {
    AddAgent(PythonAgent),
    Train(String, u32), // Task, iterations
}

impl Mall {
    /// Initializes a new MALL instance
    pub async fn new(config: MallConfig) -> Result<Self, Box<dyn Error>> {
        let (command_tx, mut command_rx) = mpsc::channel(100);
        let mut agents = Vec::new();

        tokio::spawn(async move {
            while let Some(cmd) = command_rx.recv().await {
                match cmd {
                    MallCommand::AddAgent(agent) => {
                        agents.push(agent);
                        println!("Added agent to MALL: {}", agents.len());
                    }
                    MallCommand::Train(task, iterations) => {
                        println!("Training on {} for {} iterations", task, iterations);
                        for agent in &mut agents {
                            // Simulate training by sending a UAL message
                            let msg = UalMessage::new("train", Mode::Json)
                                .with_json_payload(&serde_json::json!({"task": task.clone()}))
                                .unwrap();
                            agent.process_message(msg).await.unwrap();
                        }
                    }
                }
            }
        });

        Ok(Mall { agents, command_tx })
    }

    /// Adds an agent to the learning lab
    pub async fn add_agent(&self, agent: PythonAgent) -> Result<(), Box<dyn Error>> {
        self.command_tx.send(MallCommand::AddAgent(agent)).await?;
        Ok(())
    }

    /// Starts training for all agents
    pub async fn train(&self, task: String, iterations: u32) -> Result<(), Box<dyn Error>> {
        self.command_tx
            .send(MallCommand::Train(task, iterations))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mall_train() {
        let config = MallConfig {
            task: "test-task".to_string(),
            iterations: 10,
        };
        let mall = Mall::new(config).await.unwrap();
        let agent = PythonAgent::new("test-agent".to_string());
        mall.add_agent(agent).await.unwrap();
        mall.train("test-task".to_string(), 10).await.unwrap();
    }
}