// agents/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use async_trait::async_trait;
use ual::UALStatement;
use mapledb::{MapleDB, MapleDBError};
use std::collections::HashMap;

// Trait defining the behavior of a Maple Agent, now using MapleDBError
#[async_trait]
pub trait MapleAgent: Send + Sync {
    // Execute a UAL statement with access to the database, returning MapleDBError on failure
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError>;
}

// Registry to manage multiple agents with database access
pub struct AgentRegistry {
    agents: HashMap<String, Box<dyn MapleAgent>>,
    pub db: MapleDB, // Made public to allow access in EnterpriseNode
}

impl AgentRegistry {
    // Create a new registry with a shared database instance
    pub fn new(db: MapleDB) -> Self {
        AgentRegistry {
            agents: HashMap::new(),
            db,
        }
    }

    // Register an agents with a unique ID and persist its definition in the database asynchronously
    pub async fn register(&mut self, id: String, agent: impl MapleAgent + 'static) {
        self.agents.insert(id.clone(), Box::new(agent));
        // Await the async define_agent call
        if let Err(e) = self.db.define_agent(&id, "Generic", "Generic agents").await {
            eprintln!("Failed to register agents {} in DB: {}", id, e);
        }
    }

    // Execute a UAL statement using the appropriate agents, converting error to String
    pub async fn execute(&self, stmt: &UALStatement) -> Result<(), String> {
        if let Some(agent) = self.agents.get(&stmt.destination) {
            agent.execute(stmt, &self.db).await.map_err(|e| e.to_string())
        } else {
            Err(format!("Agent '{}' not found", stmt.destination))
        }
    }
}

// System Coordinator Agent
pub struct SystemCoordinator;
#[async_trait]
impl MapleAgent for SystemCoordinator {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("SystemCoordinator coordinating task: {:?}", stmt);
        // Example: Generate sub-tasks
        let sub_tasks = vec![
            "EXEC subtask1 agent1".to_string(),
            "EXEC subtask2 product_manager".to_string(),
        ];
        db.log_event(&format!("SystemCoordinator generated sub-tasks: {:?}", sub_tasks)).await?;
        Ok(())
    }
}

// Software Factory Agents
pub struct ProductManager;
#[async_trait]
impl MapleAgent for ProductManager {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("ProductManager defining product: {:?}", stmt);
        db.log_event(&format!("ProductManager executed: {:?}", stmt)).await?;
        Ok(())
    }
}

pub struct Architect;
#[async_trait]
impl MapleAgent for Architect {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("Architect defining requirements: {:?}", stmt);
        db.log_event(&format!("Architect executed: {:?}", stmt)).await?;
        Ok(())
    }
}

pub struct ProjectManager;
#[async_trait]
impl MapleAgent for ProjectManager {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("ProjectManager planning: {:?}", stmt);
        db.log_event(&format!("ProjectManager executed: {:?}", stmt)).await?;
        Ok(())
    }
}

pub struct SystemEngineer;
#[async_trait]
impl MapleAgent for SystemEngineer {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("SystemEngineer managing infra/release: {:?}", stmt);
        db.log_event(&format!("SystemEngineer executed: {:?}", stmt)).await?;
        Ok(())
    }
}

pub struct AppDeveloper;
#[async_trait]
impl MapleAgent for AppDeveloper {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("AppDeveloper developing app: {:?}", stmt);
        db.log_event(&format!("AppDeveloper executed: {:?}", stmt)).await?;
        Ok(())
    }
}

pub struct QAEngineer;
#[async_trait]
impl MapleAgent for QAEngineer {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("QAEngineer testing app: {:?}", stmt);
        db.log_event(&format!("QAEngineer executed: {:?}", stmt)).await?;
        Ok(())
    }
}

pub struct SimpleAgent;
#[async_trait]
impl MapleAgent for SimpleAgent {
    async fn execute(&self, stmt: &UALStatement, db: &MapleDB) -> Result<(), MapleDBError> {
        println!("SimpleAgent executing: {:?}", stmt);
        db.log_event(&format!("SimpleAgent executed: {:?}", stmt)).await?;
        Ok(())
    }
}