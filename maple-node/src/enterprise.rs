// maple-node/src/enterprise.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use ual::{UALStatement, parse_ual};
use map::create_map_message;
use agent::AgentRegistry;
use maple_package::{MaplePackage, PackageManager};
use mapledb::MapleDB;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};
use futures::{SinkExt, StreamExt};

pub struct EnterpriseNode {
    running: bool,
    agents: AgentRegistry,
    package_mgr: PackageManager,
    current_package: Option<MaplePackage>,
    db: MapleDB,    // Added persistence
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

        // Start TCP server for admin commands
        let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");
        println!("Admin server listening on 127.0.0.1:8080");
        
        // Example: Load default software_factory package as current
        if let Some(pkg) = self.package_mgr.get("software_factory") {
            // Store an owned copy; assumes MaplePackage implements Clone, or we take ownership
            self.current_package = Some(pkg.clone()); // Adjust if Clone isn’t implemented
            println!("Loaded package: {}", pkg.name);
            self.execute_package().await;
        }

        // Admin command loop
        while self.running {
            let (stream, _) = listener
                .accept().await
                .expect("Failed to accept connection");
            let mut framed = Framed::new(stream, LinesCodec::new());
            while let Some(Ok(line)) = framed.next().await {
                self.handle_admin_command(&line, &mut framed).await;
            }
        }
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
    pub fn export_package(
        &self, 
        name: &str, 
        description: &str, 
        workflow: Vec<String>, 
        path: &str
    ) -> Result<(), String> {
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

    // Handle admin commands from CLI
    async fn handle_admin_command(&mut self, cmd: &str, framed: &mut Framed<TcpStream, LinesCodec>) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        // Match on the first part of the command, dereferencing to get &str
        match *parts.get(0).unwrap_or(&"") {
            "import" => {
                if parts.len() > 1 {
                    if let Err(e) = self.import_package(parts[1]) {
                        let _ = framed.send(format!("Error: {}", e)).await;
                    } else {
                        let _ = framed.send("Package imported".to_string()).await;
                    }
                }
            }
            "export" => {
                if parts.len() > 3 {
                    let workflow = parts[3..].iter().map(|s| s.to_string()).collect();
                    if let Err(e) = self.export_package(parts[1], "Exported package", workflow, parts[2]) {
                        let _ = framed.send(format!("Error: {}", e)).await;
                    } else {
                        let _ = framed.send("Package exported".to_string()).await;
                    }
                }
            }
            "ual" => {
                if parts.len() > 1 {
                    let ual_cmd = parts[1..].join(" ");
                    if let Ok(stmt) = parse_ual(&ual_cmd) {
                        if let Err(e) = self.send_ual(stmt).await {
                            let _ = framed.send(format!("Error: {}", e)).await;
                        } else {
                            let _ = framed.send("UAL executed".to_string()).await;
                        }
                    }
                }
            }
            _ => {
                let _ = framed.send("Unknown command".to_string()).await;
            }
        }
    }
}