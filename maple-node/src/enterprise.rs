// maple-node/src/enterprise.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use ual::{UALStatement, parse_ual};
use map::create_map_message;
use agent::AgentRegistry;
use maple_package::{MaplePackage, PackageManager};
use tokio;
use mapledb::MapleDB;

pub struct EnterpriseNode {
    running: bool,
    agents: AgentRegistry,
    package_mgr: PackageManager,
    current_package: Option<MaplePackage>,
    db: MapleDB, // Added persistence
}

impl EnterpriseNode {
    // Initialize a new EnterpriseNode with default package manager
    pub fn new() -> Self {
        let mut agents = AgentRegistry::new();
        // Register a default agent with a simple implementation
        agents.register("agent1".to_string(), agent::SimpleAgent);

        let mut package_mgr = PackageManager::new();
        // Add default package using a public method (assuming one exists or needs to be added)
        package_mgr.add_package(maple_package::default_software_factory()); // Replace direct access

        // Initialize SQLite database for persistence
        let db = MapleDB::new("maple.db").expect("Failed to initialize DB");

        EnterpriseNode {
            running: false,
            agents,
            package_mgr,
            current_package: None,
            db,
        }
    }

    // Start the node in Enterprise Mode with admin capabilities
    pub async fn start(&mut self) {
        self.running = true;
        println!("MAPLE Node started in Enterprise Mode.");

        // Example: Load default software_factory package as current
        if let Some(pkg) = self.package_mgr.get("software_factory") {
            // Store an owned copy; assumes MaplePackage implements Clone or we take ownership
            self.current_package = Some(pkg.clone()); // Adjust if Clone isn’t implemented
            println!("Loaded package: {}", pkg.name);
            self.execute_package().await;
        }

        // Placeholder for admin loop (e.g., listen for commands)
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }

    // Send a UAL command and execute it locally
    pub async fn send_ual(&mut self, stmt: UALStatement) -> Result<(), String> {
        // Create a MAP message from the UAL statement
        let msg = create_map_message(stmt, "maple-node");
        println!("Enterprise Mode UAL: {:?}", msg);

        // Execute the UAL payload using the agent registry
        self.agents.execute(&msg.payload).await?;

        // Log the event to the database
        self.db.log_event(&format!("{:?}", msg))?;

        Ok(())
    }

    // Import a package from a file
    pub fn import_package(&mut self, path: &str) -> Result<(), String> {
        self.package_mgr.import(path)?;
        println!("Imported package from {}", path);
        Ok(())
    }

    // Export the current package to a file
    pub fn export_package(&self, name: &str, description: &str, workflow: Vec<String>, path: &str) -> Result<(), String> {
        self.package_mgr.export(name, description, workflow, path)?;
        println!("Exported package to {}", path);
        Ok(())
    }

    // Execute the current package's workflow
    async fn execute_package(&mut self) {
        // Take ownership of current_package temporarily to avoid borrow issues
        if let Some(pkg) = self.current_package.take() {
            println!("Executing package: {}", pkg.name);
            for ual_cmd in &pkg.workflow {
                if let Ok(stmt) = parse_ual(ual_cmd) {
                    if let Err(e) = self.send_ual(stmt).await {
                        println!("Error executing UAL: {}", e);
                    }
                }
            }
            // Restore the package after execution
            self.current_package = Some(pkg);
        }
    }
}