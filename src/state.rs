use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crdts::GSet;
use libp2p::PeerId;
use serde::Serialize;
use tokio::sync::broadcast;
use crate::telemetry::NetworkEvent;

#[derive(Clone, Serialize, Debug)]
pub struct DmEntry {
    pub from: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Clone, Serialize)]
pub struct AppStateSnapshot {
    pub peers: Vec<String>,
    pub log: Vec<String>,
    pub dms: Vec<DmEntry>,
    pub local_peer_id: String,
}

#[derive(Clone)]
pub struct AppState {
    pub log: Arc<RwLock<GSet<String>>>,
    pub peers: Arc<RwLock<HashSet<PeerId>>>,
    pub public_keys: Arc<RwLock<std::collections::HashMap<PeerId, Vec<u8>>>>,
    pub dms: Arc<RwLock<Vec<DmEntry>>>,
    pub local_peer_id: String,
    pub telemetry_tx: broadcast::Sender<NetworkEvent>,
}

impl AppState {
    pub fn new(local_peer_id: String) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {
            log: Arc::new(RwLock::new(GSet::new())),
            peers: Arc::new(RwLock::new(HashSet::new())),
            public_keys: Arc::new(RwLock::new(std::collections::HashMap::new())),
            dms: Arc::new(RwLock::new(Vec::new())),
            local_peer_id,
            telemetry_tx: tx,
        }
    }

    pub fn snapshot(&self) -> AppStateSnapshot {
        let peers = self.peers.read().unwrap().iter().map(|p| p.to_string()).collect();
        let log = self.log.read().unwrap().read().iter().cloned().collect();
        let dms = self.dms.read().unwrap().clone();
        let local_peer_id = self.local_peer_id.clone();
        
        AppStateSnapshot { peers, log, dms, local_peer_id }
    }
}
