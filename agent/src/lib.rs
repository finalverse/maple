// agent/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use async_trait::async_trait;
use ual::UALStatement;
use std::collections::HashMap;

// Trait defining the behavior of a Maple Agent
#[async_trait]
pub trait MapleAgent: Send + Sync {
    async fn execute(&self, stmt: &UALStatement) -> Result<(), String>;
}

// Agent Registry to manage multiple agents
pub struct AgentRegistry {
    agents: HashMap<String, Box<dyn MapleAgent>>,
}

impl AgentRegistry {
    // Create a new empty agent registry
    pub fn new() -> Self {
        AgentRegistry {
            agents: HashMap::new(),
        }
    }

    // Register an agent with a unique ID
    pub fn register(&mut self, id: String, agent: impl MapleAgent + 'static) {
        self.agents.insert(id, Box::new(agent));
    }

    // Execute a UAL statement on the specified agent
    pub async fn execute(&self, stmt: &UALStatement) -> Result<(), String> {
        if let Some(agent) = self.agents.get(&stmt.destination) {
            agent.execute(stmt).await
        } else {
            Err(format!("Agent '{}' not found", stmt.destination))
        }
    }
}

// Example SimpleAgent implementation
pub struct SimpleAgent;

#[async_trait]
impl MapleAgent for SimpleAgent {
    async fn execute(&self, stmt: &UALStatement) -> Result<(), String> {
        println!("SimpleAgent executing: {:?}", stmt);
        Ok(())
    }
}