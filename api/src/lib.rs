// Secure RESTful API for the MAPLE ecosystem
// © 2025 Finalverse Inc. All rights reserved.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use maple_agents::Agent;
use maple_runtime::{Runtime, RuntimeConfig, RuntimeMode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use warp::Filter;

/// Configuration for the API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    bind_addr: String, // e.g., "0.0.0.0:8080"
    secret_key: String, // Secret for JWT signing
}

/// JWT claims for access key authentication
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // User ID
    tier: String, // e.g., "free", "paid"
    exp: usize, // Expiration timestamp
}

/// API server state
pub struct ApiServer {
    runtime: Runtime,
    keys: HashMap<String, String>, // API key -> Tier (mock DB)
}

impl ApiServer {
    /// Initializes a new API server
    pub async fn new(api_config: ApiConfig) -> Result<Self, Box<dyn Error>> {
        let runtime_config = RuntimeConfig {
            mode: RuntimeMode::Enterprise, // API runs in enterprise mode
            map_listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
            db_path: "maple_api_db".to_string(),
        };
        let runtime = Runtime::new(runtime_config).await?;

        // Mock key store (replace with real DB)
        let mut keys = HashMap::new();
        keys.insert("free-key".to_string(), "free".to_string());
        keys.insert("paid-key".to_string(), "paid".to_string());

        Ok(ApiServer { runtime, keys })
    }

    /// Starts the API server
    pub async fn start(self, config: ApiConfig) {
        tracing_subscriber::fmt::init();

        let auth_filter = warp::any()
            .and(warp::header::<String>("authorization"))
            .and_then(move |auth: String| {
                let server = self.clone();
                async move {
                    let token = auth.trim();
                    let key = server.keys.get(token).ok_or("Invalid API key")?;
                    let claims = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(config.secret_key.as_ref()),
                        &Validation::default(),
                    )
                        .map_err(|_| "Invalid token")?;
                    Ok::<(String, String), warp::Rejection>((claims.claims.sub, key.clone()))
                }
                    .map_err(warp::reject::custom)
            });

        let spawn_agent = warp::post()
            .and(warp::path("agents"))
            .and(warp::path("spawn"))
            .and(auth_filter.clone())
            .and(warp::body::bytes())
            .and_then(move |(sub, tier), body: bytes::Bytes| {
                let server = self.clone();
                async move {
                    // Limit free tier to basic agents
                    if tier == "free" && body.len() > 1024 {
                        return Err(warp::reject::custom("Free tier limited to small agents"));
                    }
                    let agent = Agent::from_map_file(&format!("temp_{}.map", sub)).await?;
                    tokio::fs::write(format!("temp_{}.map", sub), &body).await?;
                    let did = agent.did.clone();
                    server.runtime.spawn_agent(did.clone()).await?;
                    Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&serde_json::json!({"did": did})))
                }
                    .map_err(warp::reject::custom)
            });

        let routes = spawn_agent.with(warp::log("maple_api"));
        warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_init() {
        let config = ApiConfig {
            bind_addr: "0.0.0.0:8080".to_string(),
            secret_key: "secret".to_string(),
        };
        let server = ApiServer::new(config).await;
        assert!(server.is_ok());
    }
}