# Walkthrough - Phase 1: MVP (UDP Gossip)

We have successfully implemented the initial version of GhostMesh. Two nodes can now discover each other on the local network (or localhost) and exchange messages using the Gossip protocol.

## Changes Made

- **Project Setup**: Initialized Rust project with `tokio`, `libp2p`, `serde`, etc.
- **P2P Logic (`src/p2p.rs`)**:
    - Configured `libp2p` Swarm with **TCP**, **QUIC**, **Noise** (encryption), and **Yamux** (multiplexing).
    - Implemented **Gossipsub** for decentralized message broadcasting.
    - Integrated **mDNS** for automatic local peer discovery.
    - Added logic to **automatically dial** discovered peers to establish the mesh.
- **CLI (`src/main.rs`)**: Simple command-line interface to start a node on a specific port.

## Verification Results

We ran two nodes locally:
1. **Node A** on port 8080
2. **Node B** on port 8082

### 1. Discovery
Both nodes successfully discovered each other via mDNS.

**Node A Logs:**
```
INFO ghostmesh::p2p: mDNS discovered a new peer: 12D3KooWASyRWYszfCC2iGdgRoc5QyT59HderAk2Wjdq9TW9FAHF
```

**Node B Logs:**
```
INFO ghostmesh::p2p: mDNS discovered a new peer: 12D3KooWDq7yztU7X4WQ6V5jTV3U389ikb1ZgQm7sXdyMhbKn6RT
```

### 2. Message Exchange
We sent the message "Hello from A" from Node A. Node B received it instantly.

**Node B Logs:**
```
INFO ghostmesh::p2p: Got message: 'Hello from A' from peer: PeerId("12D3KooWDq7yztU7X4WQ6V5jTV3U389ikb1ZgQm7sXdyMhbKn6RT")
```
