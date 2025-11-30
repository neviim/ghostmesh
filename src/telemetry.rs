use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum NetworkEvent {
    PeerConnected { peer_id: String },
    PeerDisconnected { peer_id: String },
    MessageSent { from: String, to: String, protocol: String },
    MessageReceived { from: String, to: String, protocol: String },
    LogEntry { from: String, content: String },
}
