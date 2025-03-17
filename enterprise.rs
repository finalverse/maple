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
use std::rc::Rc;

pub struct EnterpriseNode {
    running: bool,
    agents: AgentRegistry,
    package_mgr: PackageManager,
    current_package: Option<MaplePackage>,
    db: Rc<MapleDB>,                // Use Rc to share ownership
    current_project: Option<i64>,   // Added for project management
}

impl EnterpriseNode {
    /// Initialize a new EnterpriseNode with default package manager
    pub fn new() -> Self {
        // Initialize SQLite database for persistence
        let db = Rc::new(MapleDB::new("maple.db").expect("Failed to initialize DB"));

        let mut agents = AgentRegistry::new(Rc::clone(&db));

        // Register a default agent with a simple implementation
        agents.register("agent1".to_string(), agent::SimpleAgent);

        agents.register("system_coordinator".to_string(), agent::SystemCoordinator);
        agents.register("product_manager".to_string(), agent::ProductManager);
        agents.register("architect".to_string(), agent::Architect);
        agents.register("project_manager".to_string(), agent::ProjectManager);
        agents.register("system_engineer".to_string(), agent::SystemEngineer);
        agents.register("app_developer".to_string(), agent::AppDeveloper);
        agents.register("qa_engineer".to_string(), agent::QAEngineer);

        let mut package_mgr = PackageManager::new();
        // Add default package using a public method
        package_mgr.add_package(maple_package::default_software_factory());

        EnterpriseNode {
            running: false,
            agents,
            package_mgr,
            current_package: None,
            db,
            current_project: None,
        }
    }

    /// Start the node in Enterprise Mode with admin capabilities
    pub async fn start(&mut self) {
        self.running = true;
        println!("MAPLE Node started in Enterprise Mode.");

        // Start TCP server for admin commands
        let listener = TcpListener::bind("127.0.0.1:8080")
            .await.expect("Failed to bind");
        println!("Admin server listening on 127.0.0.1:8080");

        // Load default software_factory package as current
        if let Some(pkg) = self.package_mgr.get("software_factory") {
            self.current_package = Some(pkg.clone()); // Assumes Clone is implemented
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

    /// Send a UAL command and execute it locally
    pub async fn send_ual(&mut self, stmt: UALStatement) -> Result<(), String> {
        let msg = create_map_message(stmt, "maple-node");
        println!("Enterprise Mode UAL: {:?}", msg);
        self.agents.execute(&msg.payload).await?;
        self.db.log_event(&format!("{:?}", msg)).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Import a package from a file
    pub fn import_package(&mut self, path: &str) -> Result<(), String> {
        self.package_mgr.import(path)?;
        println!("Imported package from {}", path);
        Ok(())
    }

    /// Export the current package to a file
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

    /// Execute the current package's workflow
    async fn execute_package(&mut self) {
        if let Some(pkg) = self.current_package.take() {
            println!("Executing package: {}", pkg.name);
            for ual_cmd in &pkg.workflow {
                if let Ok(stmt) = parse_ual(ual_cmd) {
                    if let Err(e) = self.send_ual(stmt).await {
                        println!("Error executing UAL: {}", e);
                    }
                }
            }
            self.current_package = Some(pkg);
        }
    }

    /// Handle admin commands from CLI over TCP
    async fn handle_admin_command(&mut self, cmd: &str, framed: &mut Framed<TcpStream, LinesCodec>) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let user_id = "admin"; // Hardcoded for now, replace with auth later
        let role = self.db.get_user_role(user_id).unwrap_or(None).unwrap_or("user".to_string());
        
        match *parts.get(0).unwrap_or(&"") {
            "define_agent" => {
                if role != "admin" {
                    let _ = framed.send("Permission denied: Admin only".to_string()).await;
                    return;
                }
                if parts.len() > 3 && parts[1] == "AGENT" {
                    let id = parts[2];
                    let role = parts.get(3).unwrap_or(&"Generic");
                    let desc = parts[4..].join(" ");
                    self.db.define_agent(id, role, &desc).map_err(|e| e.to_string())?;
                    let _ = framed.send(format!("Agent '{}' defined", id)).await;
                } else {
                    let _ = framed.send("Syntax: define_agent AGENT <id> <role> <description>").await;
                }
            }
            "create_project" => {
                if role != "admin" {
                    let _ = framed.send("Permission denied: Admin only".to_string()).await;
                    return;
                }
                if parts.len() > 2 {
                    match self.db.create_project(parts[1], &parts[2..].join(" ")) {
                        Ok(id) => {
                            self.current_project = Some(id);
                            let _ = framed.send(format!("Project created with ID: {}", id)).await;
                        }
                        Err(e) => {
                            let _ = framed.send(format!("Error creating project: {}", e)).await;
                        }
                    }
                } else {
                    let _ = framed.send("Error: Name and description required".to_string()).await;
                }
            }
            "list_projects" => {
                match self.db.list_projects() {
                    Ok(projects) => {
                        let response = projects.iter()
                            .map(|p| format!("{:?}", p))
                            .collect::<Vec<_>>()
                            .join("\n");
                        let _ = framed.send(response).await;
                    }
                    Err(e) => {
                        let _ = framed.send(format!("Error listing projects: {}", e)).await;
                    }
                }
            }
            "switch_project" => {
                if parts.len() > 1 {
                    match parts[1].parse::<i64>() {
                        Ok(id) => {
                            match self.db.switch_project_status(id, "active") {
                                Ok(()) => {
                                    self.current_project = Some(id);
                                    let _ = framed.send(format!("Switched to project ID: {}", id)).await;
                                }
                                Err(e) => {
                                    let _ = framed.send(format!("Error switching project: {}", e)).await;
                                }
                            }
                        }
                        Err(_) => {
                            let _ = framed.send("Error: Invalid project ID".to_string()).await;
                        }
                    }
                } else {
                    let _ = framed.send("Error: Project ID required".to_string()).await;
                }
            }
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