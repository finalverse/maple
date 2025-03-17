// sdk/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use ual::{UALStatement, parse_ual};
use agent::{AgentRegistry, MapleAgent};
use maple_package::MaplePackage;
use mapledb::MapleDB;

pub struct MapleSDK {
    agents: AgentRegistry,
    packages: Vec<MaplePackage>,
}

impl MapleSDK {
    pub fn new() -> Self {
        // Initialize MapleDB for the AgentRegistry
        let db = MapleDB::new("maple_sdk.db").expect("Failed to initialize DB");
        MapleSDK {
            agents: AgentRegistry::new(db), // Provide the required MapleDB instance
            packages: Vec::new(),
        }
    }

    pub fn register_agent(&mut self, id: &str, agent: impl MapleAgent + 'static) {
        self.agents.register(id.to_string(), agent);
    }

    pub async fn execute_ual(&self, stmt: UALStatement) -> Result<(), String> {
        self.agents.execute(&stmt).await
    }

    // Add a package to the SDK
    pub fn add_package(&mut self, pkg: MaplePackage) {
        self.packages.push(pkg);
    }

    // Execute a named package
    pub async fn execute_package(&self, name: &str) -> Result<(), String> {
        if let Some(pkg) = self.packages.iter().find(|p| p.name == name) {
            for ual_cmd in &pkg.workflow {
                if let Ok(stmt) = parse_ual(ual_cmd) {
                    self.execute_ual(stmt).await?;
                }
            }
            Ok(())
        } else {
            Err(format!("Package '{}' not found", name))
        }
    }
}