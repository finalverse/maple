// config/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MapleConfig {
    pub node_mode: String, // "distributed" or "enterprise"
}

impl Default for MapleConfig {
    fn default() -> Self {
        MapleConfig {
            node_mode: "distributed".to_string(),
        }
    }
}