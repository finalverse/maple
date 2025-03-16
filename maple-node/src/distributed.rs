// maple-node/src/distributed.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use futures::StreamExt;
use libp2p::{floodsub::{Floodsub, FloodsubEvent, Topic}, identity, swarm::{Config, SwarmEvent}, tcp::{self, tokio::Transport as TcpTransport}, Swarm, PeerId, Transport};
use serde_json;
use ual::UALStatement;
use map::{MapMessage, create_map_message};

pub struct DistributedNode {
    swarm: Swarm<Floodsub>,
    running: bool,
}

impl DistributedNode {
    // Initialize a new DistributedNode with libp2p setup
    pub fn new() -> Self {
        // Generate a local keypair for node identity
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local Peer ID: {}", local_peer_id);

        // Create a TCP transport using tokio
        let transport = TcpTransport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(libp2p::noise::Config::new(&local_key).unwrap())
            .multiplex(libp2p::yamux::Config::default())
            .boxed();

        // Initialize Floodsub behavior for broadcasting messages
        let floodsub = Floodsub::new(local_peer_id);

        // Configure and create the swarm
        let swarm_config = Config::with_tokio_executor();
        let mut swarm = Swarm::new(transport, floodsub, local_peer_id, swarm_config);

        // Subscribe to the "maple-tasks" topic for UAL messages
        swarm.behaviour_mut().subscribe(Topic::new("maple-tasks"));

        // Listen on all interfaces on a random TCP port
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

        DistributedNode {
            swarm,
            running: false,
        }
    }

    // Start the node in Distributed Mode
    pub async fn start(&mut self) {
        self.running = true;
        println!("MAPLE Node started in Distributed Mode.");

        // Main event loop to handle P2P events
        while self.running {
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(FloodsubEvent::Message(message)) => {
                    // Deserialize incoming MAP message
                    if let Ok(msg) = serde_json::from_slice::<MapMessage>(&message.data) {
                        println!("Received MAP Message from {}: {:?}", message.source, msg);
                        // TODO: Process UALStatement payload locally or forward
                    }
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {}", address);
                }
                _ => {} // Ignore other events for now
            }
        }
    }

    // Send a UAL command to the MAP network
    pub async fn send_ual(&mut self, stmt: UALStatement) -> Result<(), String> {
        // Create a MAP message with the UAL payload
        let msg = create_map_message(stmt, "maple-node");
        let msg_bytes = serde_json::to_vec(&msg).map_err(|e| e.to_string())?;

        // Publish the MAP message to the "maple-tasks" topic
        self.swarm.behaviour_mut().publish(Topic::new("maple-tasks"), msg_bytes);
        println!("Sent MAP Message: {:?}", msg);
        Ok(())
    }
}