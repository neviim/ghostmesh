# Walkthrough - Phase 5: Web Dashboard

We have added a modern Web Dashboard to visualize the GhostMesh node state.

## Changes Made

- **Web Server**: Integrated `warp` to serve HTTP requests on `port + 1`.
- **Shared State**: Created `AppState` with `Arc<RwLock<...>>` to share data between the P2P loop and the Web Server.
- **Frontend**: Created `web/index.html` with a dark-themed UI to view peers, logs, and send messages.
- **API**:
    - `GET /api/state`: Returns JSON with connected peers and log entries.
    - `POST /api/log`: Accepts a plain text message and broadcasts it to the mesh.

## Verification Results

We ran two nodes locally:
1.  **Node A** (P2P: 8080, Web: 8081)
2.  **Node B** (P2P: 8082, Web: 8083)

### 1. API Verification
We used `wget` to interact with Node A's API.

**Post Log:**
```bash
wget -qO- --post-data="Hello Web Dashboard" --header="Content-Type: text/plain" http://localhost:8081/api/log
```

**Result:**
Node B received the message instantly:
```
INFO ghostmesh::p2p: Synced CRDT state. Current Log: {"Hello Web Dashboard"}
```

### 2. UI
The dashboard is available at `http://localhost:8081` (for Node A) and displays the real-time state of the mesh.
