// Rust SDK for interacting with the MAPLE ecosystem
// © 2025 Finalverse Inc. All rights reserved.

use maple_agents::{Agent, AgentConfig};
use maple_map::{MapConfig, MapProtocol};
use maple_ual::{UalMessage, Mode};
use serde_json::json;
use std::error::Error;
use tokio;

/// Configuration for the SDK
pub struct SdkConfig {
    api_endpoint: String, // e.g., "http://localhost:8080"
    access_key: String,   // For API authentication
}

/// MAPLE Rust SDK
pub struct MapleSdk {
    config: SdkConfig,
    map: MapProtocol,
}

impl MapleSdk {
    /// Initializes a new SDK instance
    pub async fn new(config: SdkConfig) -> Result<Self, Box<dyn Error>> {
        let map_config = MapConfig {
            listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
        };
        let map = MapProtocol::new(map_config).await?;
        Ok(MapleSdk { config, map })
    }

    /// Creates and registers a new agent
    pub async fn create_agent(&self, name: &str, role: &str) -> Result<Agent, Box<dyn Error>> {
        let agent_config = AgentConfig {
            name: name.to_string(),
            role: role.to_string(),
        };
        let agent = Agent::new(agent_config.clone());

        // Optionally register via API (MRS integration example)
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/agents/register", self.config.api_endpoint))
            .header("Authorization", format!("Bearer {}", self.config.access_key))
            .json(&json!({ "name": name, "role": role }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(agent)
        } else {
            Err(format!("Failed to register agent: {}", response.status()).into())
        }
    }

    /// Sends a message to an agent via MAP
    pub async fn send_message(&self, agent_did: &str, action: &str, payload: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let msg = UalMessage::new(action, Mode::Json).with_json_payload(&payload)?;
        self.map.broadcast(format!("{}:{}", agent_did, serde_json::to_string(&msg)?)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_agent() {
        let config = SdkConfig {
            api_endpoint: "http://localhost:8080".to_string(),
            access_key: "test-key".to_string(),
        };
        let sdk = MapleSdk::new(config).await.unwrap();
        let agent = sdk.create_agent("test-agent", "test-role").await;
        assert!(agent.is_ok());
    }
}