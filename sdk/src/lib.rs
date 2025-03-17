// sdk/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use ual::UALStatement;
use agent::{AgentRegistry, MapleAgent};

pub struct MapleSDK {
    agents: AgentRegistry,
}

impl MapleSDK {
    pub fn new() -> Self {
        MapleSDK {
            agents: AgentRegistry::new(),
        }
    }

    pub fn register_agent(&mut self, id: &str, agent: impl MapleAgent + 'static) {
        self.agents.register(id.to_string(), agent);
    }

    pub async fn execute_ual(&self, stmt: UALStatement) -> Result<(), String> {
        self.agents.execute(&stmt).await
    }
}