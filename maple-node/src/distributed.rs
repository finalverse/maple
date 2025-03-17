// maple-node/src/distributed.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use futures::StreamExt;
use libp2p::{floodsub::{Floodsub, FloodsubEvent, Topic}, identity, swarm::{Config, SwarmEvent}, tcp::{self, tokio::Transport as TcpTransport}, Swarm, PeerId, Transport};
use serde_json;
use ual::UALStatement;
use map::{MapMessage, create_map_message};
use agent::AgentRegistry;

pub struct DistributedNode {
    swarm: Swarm<Floodsub>,
    running: bool,
    agents: AgentRegistry, // Added agent registry
}

impl DistributedNode {
    pub async fn new() -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local Peer ID: {}", local_peer_id);

        let transport = TcpTransport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(libp2p::noise::Config::new(&local_key).unwrap())
            .multiplex(libp2p::yamux::Config::default())
            .boxed();

        let floodsub = Floodsub::new(local_peer_id);
        let swarm_config = Config::with_tokio_executor();
        let mut swarm = Swarm::new(transport, floodsub, local_peer_id, swarm_config);

        swarm.behaviour_mut().subscribe(Topic::new("maple-tasks"));
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

        let mut agents = AgentRegistry::new();
        agents.register("agent1".to_string(), agent::SimpleAgent); // Register default agent

        DistributedNode {
            swarm,
            running: false,
            agents,
        }
    }

    pub async fn start(&mut self) {
        self.running = true;
        println!("MAPLE Node started in Distributed Mode.");

        while self.running {
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(FloodsubEvent::Message(message)) => {
                    if let Ok(msg) = serde_json::from_slice::<MapMessage>(&message.data) {
                        println!("Received MAP Message from {}: {:?}", message.source, msg);
                        // Execute the UAL statement if destined for this node
                        if let Err(e) = self.agents.execute(&msg.payload).await {
                            println!("Execution error: {}", e);
                        }
                    }
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {}", address);
                }
                _ => {}
            }
        }
    }

    pub async fn send_ual(&mut self, stmt: UALStatement) -> Result<(), String> {
        let msg = create_map_message(stmt, "maple-node");
        let msg_bytes = serde_json::to_vec(&msg).map_err(|e| e.to_string())?;
        self.swarm.behaviour_mut().publish(Topic::new("maple-tasks"), msg_bytes);
        println!("Sent MAP Message: {:?}", msg);
        Ok(())
    }
}