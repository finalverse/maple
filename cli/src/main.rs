// crate/maple/src/main.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use clap::{Parser, Subcommand, Args};
use std::process::Command;

use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};
use futures::sink::SinkExt;
use futures::StreamExt;

#[derive(Parser)]
#[command(name = "maple", about = "MAPLE CLI - © 2025 Finalverse Inc.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display MAPLE CLI information
    Info,
    /// Node control commands
    Node(NodeArgs),
}

#[derive(Args)]
struct NodeArgs {
    #[command(subcommand)]
    command: NodeCommands,
}

#[derive(Subcommand)]
enum NodeCommands {
    /// Start the MAPLE Node
    Start {
        #[arg(long, default_value = "distributed")]
        mode: String, // "distributed" or "enterprise"
    },
    /// Send a UAL command to the node
    Send { ual: String },
    /// Import a package from a file
    Import { path: String },
    /// Export a package to a file (placeholder workflow)
    Export { name: String, path: String },
    CreateProject { name: String, description: String },
    ListProjects,
    SwitchProject { id: String },
    DefineAgent { id: String, role: String, description: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info => {
            println!("MAPLE CLI - Version 0.1.0");
            println!("© 2025 Finalverse Inc.");
            println!("Email: maple@finalverse.com");
            println!("Website: https://mapleai.org");
            println!("GitHub: https://github.com/finalverse/mapleai.git");
        }
        Commands::Node(args) => match args.command {
            NodeCommands::Start { mode } => {
                let mode_arg = match mode.as_str() {
                    "distributed" => "start-distributed",
                    "enterprise" => "start-enterprise",
                    _ => {
                        eprintln!("Invalid mode: {}. Use 'distributed' or 'enterprise'", mode);
                        return;
                    }
                };
                // Spawn a new process to run the runtime package
                Command::new("cargo")
                    .args(["run", "--package", "runtime", "--", mode_arg])
                    .spawn()
                    .expect("Failed to start node");
            }
            NodeCommands::Send { ual } => {
                // Connect to the node at localhost:8080
                let stream = TcpStream::connect("127.0.0.1:8080")
                    .await
                    .expect("Node not running");
                // Create a framed transport with line-based codec
                let mut framed = Framed::new(stream, LinesCodec::new());
                // Send the UAL command prefixed with "ual"
                framed.send(format!("ual {}", ual)).await.unwrap();
                // Await and print the response from the node
                if let Some(Ok(response)) = framed.next().await {
                    println!("{}", response);
                }
            }
            NodeCommands::Import { path } => {
                // Connect to the node for importing a package
                let stream = TcpStream::connect("127.0.0.1:8080")
                    .await
                    .expect("Node not running");
                let mut framed = Framed::new(stream, LinesCodec::new());
                // Send the import command with the file path
                framed.send(format!("import {}", path)).await.unwrap();
                // Print the node’s response
                if let Some(Ok(response)) = framed.next().await {
                    println!("{}", response);
                }
            }
            NodeCommands::Export { name, path } => {
                // Connect to the node for exporting a package
                let stream = TcpStream::connect("127.0.0.1:8080")
                    .await
                    .expect("Node not running");
                let mut framed = Framed::new(stream, LinesCodec::new());
                // Send the export command with name, path, and a placeholder workflow
                framed
                    .send(format!("export {} {} sample_workflow", name, path))
                    .await
                    .unwrap();
                // Print the node’s response
                if let Some(Ok(response)) = framed.next().await {
                    println!("{}", response);
                }
            }
            NodeCommands::CreateProject { name, description } => {
                let stream = TcpStream::connect("127.0.0.1:8080").await.expect("Node not running");
                let mut framed = Framed::new(stream, LinesCodec::new());
                framed.send(format!("create_project {} {}", name, description)).await.unwrap();
                if let Some(Ok(response)) = framed.next().await {
                    println!("{}", response);
                }
            }
            NodeCommands::ListProjects => {
                let stream = TcpStream::connect("127.0.0.1:8080").await.expect("Node not running");
                let mut framed = Framed::new(stream, LinesCodec::new());
                framed.send("list_projects").await.unwrap();
                if let Some(Ok(response)) = framed.next().await {
                    println!("{}", response);
                }
            }
            NodeCommands::SwitchProject { id } => {
                let stream = TcpStream::connect("127.0.0.1:8080").await.expect("Node not running");
                let mut framed = Framed::new(stream, LinesCodec::new());
                framed.send(format!("switch_project {}", id)).await.unwrap();
                if let Some(Ok(response)) = framed.next().await {
                    println!("{}", response);
                }
            }
            NodeCommands::DefineAgent { id, role, description } => {
                let stream = TcpStream::connect("127.0.0.1:8080").await.expect("Node not running");
                let mut framed = Framed::new(stream, LinesCodec::new());
                framed.send(format!("define_agent AGENT {} {} {}", id, role, description)).await.unwrap();
                if let Some(Ok(response)) = framed.next().await {
                    println!("{}", response);
                }
            }
        },
    }
}