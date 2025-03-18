// Multi-Agent Protocol (MAP) for decentralized P2P messaging in MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use libp2p::{
    identity, mdns, noise, swarm::SwarmEvent, tcp, yamux, PeerId, Swarm, Transport,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;

/// Configuration for the MAP Protocol
#[derive(Debug, Serialize, Deserialize)]
pub struct MapConfig {
    listen_addr: String, // e.g., "/ip4/0.0.0.0/tcp/0"
}

/// MAP Protocol instance managing P2P communication
pub struct MapProtocol {
    swarm: Swarm<mdns::Behaviour>,
    command_tx: mpsc::Sender<MapCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MapCommand {
    SendMessage(PeerId, String), // Send message to a peer
    Broadcast(String), // Broadcast to all peers
}

impl MapProtocol {
    /// Initializes a new MAP Protocol instance
    pub async fn new(config: MapConfig) -> Result<Self, Box<dyn Error>> {
        // Generate a local keypair for this node
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer ID: {:?}", local_peer_id);

        // Set up the transport with TCP, noise, and yamux
        let transport = tcp::async_io::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        // Create an mDNS behaviour for peer discovery
        let behaviour = mdns::Behaviour::new(mdns::Config::default(), local_peer_id)?;

        // Build the swarm
        let mut swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            libp2p::swarm::Config::with_tokio_executor(),
        );

        // Listen on the specified address
        swarm.listen_on(config.listen_addr.parse()?)?;

        // Channel for sending commands to the swarm
        let (command_tx, mut command_rx) = mpsc::channel(100);

        // Spawn the swarm event loop
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    event = swarm.select_next_some() => match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {:?}", address);
                        }
                        SwarmEvent::Behaviour(mdns::Event::Discovered(peers)) => {
                            for (peer, addr) in peers {
                                println!("Discovered peer {} at {}", peer, addr);
                                swarm.dial(addr).unwrap(); // Dial discovered peers
                            }
                        }
                        _ => {}
                    },
                    Some(cmd) = command_rx.recv() => match cmd {
                        MapCommand::SendMessage(peer, msg) => {
                            println!("Sending to {}: {}", peer, msg);
                            // TODO: Implement message sending
                        }
                        MapCommand::Broadcast(msg) => {
                            println!("Broadcasting: {}", msg);
                            // TODO: Implement broadcast
                        }
                    }
                }
            }
        });

        Ok(MapProtocol { swarm, command_tx })
    }

    /// Sends a message to a specific peer
    pub async fn send_message(&self, peer: PeerId, message: String) -> Result<(), Box<dyn Error>> {
        self.command_tx
            .send(MapCommand::SendMessage(peer, message))
            .await?;
        Ok(())
    }

    /// Broadcasts a message to all connected peers
    pub async fn broadcast(&self, message: String) -> Result<(), Box<dyn Error>> {
        self.command_tx
            .send(MapCommand::Broadcast(message))
            .await?;
        Ok(())
    }

    /// Broadcasts a .map file to all peers
    pub async fn broadcast_map_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = tokio::fs::File::open(path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        self.broadcast(String::from_utf8_lossy(&buffer).to_string()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_map_init() {
        let config = MapConfig {
            listen_addr: "/ip4/127.0.0.1/tcp/0".to_string(),
        };
        let map = MapProtocol::new(config).await;
        assert!(map.is_ok());
    }
}