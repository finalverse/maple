// REST and gRPC API for the MAPLE ecosystem with access key security
// © 2025 Finalverse Inc. All rights reserved.

use maple_agents::AgentConfig;
use maple_mrs::{Mrs, MrsConfig};
use maple_map::MapConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

/// Configuration for the API
#[derive(Debug)]
pub struct ApiConfig {
    port: u16,
    access_keys: HashMap<String, UserTier>, // Key -> Tier mapping
}

/// User tier for access control
#[derive(Debug, PartialEq)]
enum UserTier {
    Free,
    Paid,
}

/// API state
struct ApiState {
    mrs: Mrs,
    access_keys: HashMap<String, UserTier>,
}

/// Starts the API server
pub async fn start(config: ApiConfig) -> Result<(), Box<dyn Error>> {
    let map_config = MapConfig {
        listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
    };
    let mrs = Mrs::new(MrsConfig { map_config }).await?;
    let state = Arc::new(Mutex::new(ApiState {
        mrs,
        access_keys: config.access_keys,
    }));

    // Authentication filter
    let auth = warp::header::<String>("authorization")
        .and_then(move |auth: String| {
            let state = state.clone();
            async move {
                let key = auth.strip_prefix("Bearer ").ok_or(warp::reject::custom("Invalid token"))?;
                let state = state.lock().await;
                if state.access_keys.contains_key(key) {
                    Ok(key.to_string())
                } else {
                    Err(warp::reject::custom("Unauthorized"))
                }
            }
        });

    // Register agent endpoint
    let register = warp::path!("agents" / "register")
        .and(warp::post())
        .and(auth.clone())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(register_agent);

    // Routes
    let routes = register.with(warp::log("api"));
    println!("API running on port {}", config.port);
    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
    Ok(())
}

fn with_state(state: Arc<Mutex<ApiState>>) -> impl Filter<Extract = (Arc<Mutex<ApiState>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn register_agent(
    key: String,
    body: RegisterRequest,
    state: Arc<Mutex<ApiState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut state = state.lock().await;
    let tier = state.access_keys.get(&key).unwrap();
    if *tier == UserTier::Free && body.role == "premium" {
        return Err(warp::reject::custom("Free tier cannot use premium roles"));
    }

    let config = AgentConfig {
        name: body.name,
        role: body.role,
    };
    let did = state.mrs.register_agent(config).await.map_err(|e| warp::reject::custom(e.to_string()))?;
    Ok(warp::reply::json(&RegisterResponse { did }))
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    name: String,
    role: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    did: String,
}

#[cfg(test)]
mod tests {
    // Tests require a running server, omitted for brevity
}