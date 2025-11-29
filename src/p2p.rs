use crdts::{GSet, CmRDT, CvRDT};
use libp2p::{
    gossipsub, mdns, noise, ping, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId, Swarm,
};
use libp2p::futures::StreamExt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt};
use tracing::{info, error};
use anyhow::Result;
use crate::state::AppState;
use crate::http;
use tokio::sync::mpsc;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,
}

pub async fn run_node(port: u16, id_keys: libp2p::identity::Keypair) -> Result<()> {
    let mut swarm = create_swarm(port, id_keys).await?;

    // Subscribe to topics
    let topic_global = gossipsub::IdentTopic::new("ghostmesh-global");
    let topic_crdt = gossipsub::IdentTopic::new("ghostmesh-crdt");
    
    swarm.behaviour_mut().gossipsub.subscribe(&topic_global)?;
    swarm.behaviour_mut().gossipsub.subscribe(&topic_crdt)?;

    // Initialize App State
    let app_state = AppState::new();

    // Channel for Web -> P2P communication
    let (log_tx, mut log_rx) = mpsc::unbounded_channel();

    // Spawn Web Server
    let web_state = app_state.clone();
    let web_tx = log_tx.clone();
    let web_port = port + 1;
    tokio::spawn(async move {
        http::start_server(web_port, web_state, web_tx).await;
    });

    // Read from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    info!("GhostMesh Node Started on port {}. Web Dashboard: http://localhost:{}", port, web_port);

    loop {
        tokio::select! {
            // Handle Web Input (Log)
            Some(msg) = log_rx.recv() => {
                app_state.log.write().unwrap().insert(msg.clone());
                info!("Web Logged: {}", msg);
                
                // Broadcast new state
                let state_bytes = serde_json::to_vec(&*app_state.log.read().unwrap())?;
                if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic_crdt.clone(), state_bytes) {
                    error!("Publish error: {:?}", e);
                }
            }
            // Handle Stdin Input
            line = stdin.next_line() => {
                if let Ok(Some(line)) = line {
                    if line.starts_with("/") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        match parts[0] {
                            "/peers" => {
                                let peers: Vec<_> = swarm.connected_peers().collect();
                                info!("Connected Peers: {} - {:?}", peers.len(), peers);
                            }
                            "/log" => {
                                if parts.len() > 1 {
                                    let msg = parts[1..].join(" ");
                                    app_state.log.write().unwrap().insert(msg.clone());
                                    info!("Logged: {}", msg);
                                    
                                    // Broadcast new state
                                    let state_bytes = serde_json::to_vec(&*app_state.log.read().unwrap())?;
                                    if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic_crdt.clone(), state_bytes) {
                                        error!("Publish error: {:?}", e);
                                    }
                                } else {
                                    info!("Usage: /log <message>");
                                }
                            }
                            "/show" => {
                                info!("Current Log: {:?}", app_state.log.read().unwrap().read());
                            }
                            _ => info!("Unknown command. Try /peers, /log, or /show"),
                        }
                    } else {
                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic_global.clone(), line.as_bytes()) {
                            error!("Publish error: {:?}", e);
                        }
                    }
                }
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, multiaddr) in list {
                        info!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        if let Err(e) = swarm.dial(multiaddr) {
                            error!("Dial error: {:?}", e);
                        }
                    }
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        info!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    info!("Connection established with peer: {peer_id}");
                    app_state.peers.write().unwrap().insert(peer_id);
                }
                SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                    info!("Connection closed with peer: {peer_id}. Cause: {cause:?}");
                    app_state.peers.write().unwrap().remove(&peer_id);
                }
                SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                    info!("Outgoing connection error with peer {:?}: {error:?}", peer_id);
                    if let Some(peer_id) = peer_id {
                        app_state.peers.write().unwrap().remove(&peer_id);
                    }
                }
                SwarmEvent::IncomingConnectionError { error, .. } => {
                    info!("Incoming connection error: {error:?}");
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: _id,
                    message,
                })) => {
                    if message.topic == topic_crdt.hash() {
                        match serde_json::from_slice::<GSet<String>>(&message.data) {
                            Ok(remote_state) => {
                                app_state.log.write().unwrap().merge(remote_state);
                                info!("Synced CRDT state. Current Log: {:?}", app_state.log.read().unwrap().read());
                            }
                            Err(e) => error!("Failed to deserialize CRDT state: {:?}", e),
                        }
                    } else {
                        info!(
                            "Got message: '{}' from peer: {:?}",
                            String::from_utf8_lossy(&message.data),
                            peer_id
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

async fn create_swarm(port: u16, id_keys: libp2p::identity::Keypair) -> Result<Swarm<MyBehaviour>> {
    let peer_id = PeerId::from(id_keys.public());
    info!("Local Peer ID: {peer_id}");

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?

        .with_behaviour(|key| {
            // Gossipsub configuration
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|msg| anyhow::anyhow!(msg))?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            
            let ping = ping::Behaviour::new(ping::Config::new());

            Ok(MyBehaviour { gossipsub, mdns, ping })
        })?
        .build();

    // Listen on all interfaces and the specified port

    swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;

    Ok(swarm)
}
