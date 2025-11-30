use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crdts::GSet;
use libp2p::PeerId;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct AppStateSnapshot {
    pub peers: Vec<String>,
    pub log: Vec<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub log: Arc<RwLock<GSet<String>>>,
    pub peers: Arc<RwLock<HashSet<PeerId>>>,
    pub public_keys: Arc<RwLock<std::collections::HashMap<PeerId, Vec<u8>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            log: Arc::new(RwLock::new(GSet::new())),
            peers: Arc::new(RwLock::new(HashSet::new())),
            public_keys: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub fn snapshot(&self) -> AppStateSnapshot {
        let peers = self.peers.read().unwrap().iter().map(|p| p.to_string()).collect();
        let log = self.log.read().unwrap().read().iter().cloned().collect();
        
        AppStateSnapshot { peers, log }
    }
}
