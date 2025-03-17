// maple-core/package/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use serde::{Serialize, Deserialize};
use ual::UALStatement;
use std::fs;
use std::path::Path;

// Structure representing a MAPLE package
#[derive(Serialize, Deserialize, Clone)]
pub struct MaplePackage {
    pub name: String,
    pub description: String,
    pub workflow: Vec<String>, // List of UAL commands
}

// Package management operations
pub struct PackageManager {
    packages: Vec<MaplePackage>,
}

impl PackageManager {
    // Create a new PackageManager
    pub fn new() -> Self {
        PackageManager { packages: Vec::new() }
    }

    // Import a package from a file (e.g., software_factory.map)
    pub fn import(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let pkg: MaplePackage = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        self.packages.push(pkg);
        Ok(())
    }

    // Export the current workflow as a package to a file
    pub fn export(&self, name: &str, description: &str, workflow: Vec<String>, path: &str) -> Result<(), String> {
        let pkg = MaplePackage {
            name: name.to_string(),
            description: description.to_string(),
            workflow,
        };
        let json = serde_json::to_string_pretty(&pkg).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    // Get a package by name
    pub fn get(&self, name: &str) -> Option<&MaplePackage> {
        self.packages.iter().find(|p| p.name == name)
    }

    /// Add a package to the manager’s list
    pub fn add_package(&mut self, package: MaplePackage) {
        self.packages.push(package);
    }
}

// Example package: software_factory.map
pub fn default_software_factory() -> MaplePackage {
    MaplePackage {
        name: "software_factory".to_string(),
        description: "Software Factory workflow for MAPLE".to_string(),
        workflow: vec![
            "EXEC define_product product_manager".to_string(),
            "EXEC requirements architect WITH details=auth,tasks,realtime".to_string(),
            "EXEC plan project_manager WITH arch=rest,react,postgres,websocket".to_string(),
            "EXEC setup_infra system_engineer".to_string(),
            "EXEC develop_app app_developer WITH features=auth,tasks,realtime".to_string(),
            "EXEC test_app qa_engineer".to_string(),
            "EXEC release system_engineer WITH marketing=enabled".to_string(),
        ],
    }
}