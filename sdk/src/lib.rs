// Rust SDK for interacting with the MAPLE ecosystem
// © 2025 Finalverse Inc. All rights reserved.

use maple_agents::{Agent, AgentConfig};
use maple_map::{MapConfig, MapProtocol};
use maple_mrs::{Mrs, MrsConfig};
use maple_ual::{UalMessage, Mode};
use mapledb::MapleDb;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use thiserror::Error;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Custom errors for the SDK
#[derive(Error, Debug)]
pub enum SdkError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("MAPLE error: {0}")]
    Maple(String),
}

/// Configuration for the SDK
#[derive(Debug, Serialize, Deserialize)]
pub struct SdkConfig {
    api_url: String, // e.g., "http://localhost:8080"
    api_key: String, // Access key for API authentication
    map_listen_addr: String, // e.g., "/ip4/0.0.0.0/tcp/0"
    db_path: String, // Path to local MapleDB
}

/// Main SDK struct for MAPLE interactions
pub struct MapleSdk {
    client: Client,
    config: SdkConfig,
    map: MapProtocol,
    mrs: Mrs,
    db: MapleDb,
}

impl MapleSdk {
    /// Initializes a new SDK instance
    pub async fn new(config: SdkConfig) -> Result<Self, SdkError> {
        let client = Client::new();
        let map_config = MapConfig {
            listen_addr: config.map_listen_addr.clone(),
        };
        let map = MapProtocol::new(map_config).await?;
        let mrs = Mrs::new(MrsConfig { map_config }).await?;
        let db = MapleDb::new(&config.db_path)?;

        Ok(MapleSdk {
            client,
            config,
            map,
            mrs,
            db,
        })
    }

    /// Creates and registers a new agent
    pub async fn create_agent(&self, name: &str, role: &str) -> Result<String, SdkError> {
        let config = AgentConfig {
            name: name.to_string(),
            role: role.to_string(),
        };
        let agent = Agent::new(config.clone());
        let did = self.mrs.register_agent(config).await?;
        agent.dump_to_map(&format!("{}.map", name)).await?;
        Ok(did)
    }

    /// Spawns an agent from a .map file via API
    pub async fn spawn_agent(&self, map_file: &str) -> Result<String, SdkError> {
        let mut file = tokio::fs::File::open(map_file).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let response = self
            .client
            .post(&format!("{}/agents/spawn", self.config.api_url))
            .header("Authorization", &self.config.api_key)
            .body(buffer)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let did = response["did"]
            .as_str()
            .ok_or(SdkError::Maple("Missing DID in response".to_string()))?;
        Ok(did.to_string())
    }

    /// Sends a UAL message to an agent via the network
    pub async fn send_message(&self, did: &str, action: &str, payload: serde_json::Value) -> Result<(), SdkError> {
        let msg = UalMessage::new(action, Mode::Json).with_json_payload(&payload)?;
        let peer_id = self.resolve_did_to_peer(did).await?; // Placeholder for DID-to-PeerID mapping
        self.map.send_message(peer_id, serde_json::to_string(&msg)?).await?;
        Ok(())
    }

    /// Placeholder: Resolves DID to PeerID (requires MRS or network lookup)
    async fn resolve_did_to_peer(&self, _did: &str) -> Result<libp2p::PeerId, SdkError> {
        // TODO: Implement DID resolution
        Ok(libp2p::PeerId::random())
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn maple_sdk(py: Python, m: &PyModule) -> PyResult<()> {
    /// Python bindings for MapleSdk
    #[pyclass]
    struct PyMapleSdk {
        sdk: MapleSdk,
    }

    #[pymethods]
    impl PyMapleSdk {
        #[new]
        fn new(api_url: &str, api_key: &str, map_listen_addr: &str, db_path: &str) -> PyResult<Self> {
            let config = SdkConfig {
                api_url: api_url.to_string(),
                api_key: api_key.to_string(),
                map_listen_addr: map_listen_addr.to_string(),
                db_path: db_path.to_string(),
            };
            let sdk = py.allow_threads(|| {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(MapleSdk::new(config))
            })?;
            Ok(PyMapleSdk { sdk })
        }

        fn create_agent(&self, name: &str, role: &str) -> PyResult<String> {
            py.allow_threads(|| {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(self.sdk.create_agent(name, role))
            })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }

        fn spawn_agent(&self, map_file: &str) -> PyResult<String> {
            py.allow_threads(|| {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(self.sdk.spawn_agent(map_file))
            })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }
    }

    m.add_class::<PyMapleSdk>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_agent() {
        let config = SdkConfig {
            api_url: "http://localhost:8080".to_string(),
            api_key: "test-key".to_string(),
            map_listen_addr: "/ip4/127.0.0.1/tcp/0".to_string(),
            db_path: "test_sdk_db".to_string(),
        };
        let sdk = MapleSdk::new(config).await.unwrap();
        let did = sdk.create_agent("test-agent", "test-role").await.unwrap();
        assert!(did.starts_with("did:maple:agent:"));
        tokio::fs::remove_file("test-agent.map").await.unwrap();
    }
}