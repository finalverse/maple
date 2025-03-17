// maple-node/src/enterprise.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use ual::UALStatement;
use map::{MapMessage, create_map_message};
use agent::AgentRegistry;

pub struct EnterpriseNode {
    running: bool,
    agents: AgentRegistry, // Added agent registry
}

impl EnterpriseNode {
    pub fn new() -> Self {
        let mut agents = AgentRegistry::new();
        agents.register("agent1".to_string(), agent::SimpleAgent); // Register default agent
        EnterpriseNode { running: false, agents }
    }

    pub async fn start(&mut self) {
        self.running = true;
        println!("MAPLE Node started in Enterprise Mode (placeholder).");
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }

    pub async fn send_ual(&mut self, stmt: UALStatement) -> Result<(), String> {
        let msg = create_map_message(stmt, "maple-node");
        println!("Enterprise Mode UAL: {:?}", msg);
        self.agents.execute(&msg.payload).await // Execute locally
    }
}