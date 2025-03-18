// Standalone runtime executable for MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use maple_runtime::{start_distributed, start_enterprise};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <mode> [args]", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "distributed" => {
            let nodes = args.get(2).unwrap_or(&"1".to_string()).parse()?;
            let runtime = start_distributed(nodes).await?;
            tokio::signal::ctrl_c().await?;
            runtime.shutdown().await?;
        }
        "enterprise" => {
            let config_path = args.get(2).unwrap_or(&"config.toml".to_string());
            let runtime = start_enterprise(config_path).await?;
            tokio::signal::ctrl_c().await?;
            runtime.shutdown().await?;
        }
        _ => eprintln!("Unknown mode: {}", args[1]),
    }

    Ok(())
}