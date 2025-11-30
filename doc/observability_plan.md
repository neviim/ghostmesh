# Observability & Telemetry Implementation Plan

This document outlines the roadmap for implementing a real-time "War Room" style dashboard for GhostMesh, enabling visualization of network topology, connections, and message flow.

## 1. Architecture Overview

The solution bridges the decentralized nature of the P2P network with a centralized (or local) visualization interface.

*   **Source of Truth (Rust Node):** Each GhostMesh node acts as a telemetry emitter.
*   **Transport:** WebSockets (for real-time, low-latency event streaming).
*   **Visualization (Frontend):** A rich web interface rendering the graph and events.

## 2. Technology Stack

### Backend (Rust)
*   **Web Server:** Continue using `Warp` (already integrated).
*   **Protocol:** `WebSockets` (via `warp::ws`).
*   **Serialization:** `serde` + `serde_json`.
*   **Geolocation (Future):** `maxminddb` for IP-to-Geo resolution.

### Frontend (Web Dashboard)
*   **Framework:** Migration to **Next.js** (React) is recommended for complex state management, or use **Vue.js/Vanilla** with CDN libraries if keeping the single-binary simplicity is preferred.
    *   *Recommendation:* For the "War Room" experience, a dedicated frontend build (Next.js/Vite) served by the Rust binary is best.
*   **Graph Visualization:** `react-force-graph` (2D/3D) for topology.
*   **Maps:** `Mapbox GL JS` or `Leaflet` for geospatial view.

## 3. Implementation Roadmap

### Phase 1: Backend Telemetry Infrastructure
**Goal:** Enable the Rust node to stream internal events to connected clients.

1.  **Define Event Schema:**
    Create a structured enum for all network events.
    ```rust
    #[derive(Serialize, Clone)]
    #[serde(tag = "type", content = "data")]
    pub enum NetworkEvent {
        PeerConnected { peer_id: String, addr: String },
        PeerDisconnected { peer_id: String },
        MessageSent { from: String, to: String, protocol: String },
        MessageReceived { from: String, to: String, protocol: String },
        // Heartbeat / Status updates
    }
    ```

2.  **Telemetry Channel:**
    *   Create a `broadcast` channel (tokio) to distribute events from the P2P loop to WebSocket handlers.
    *   Update `AppState` to hold the sender part of this channel.

3.  **WebSocket Endpoint:**
    *   Add a `/ws` route in `http.rs`.
    *   Upgrade connections to WebSockets and subscribe them to the broadcast channel.

### Phase 2: Instrumentation (P2P Layer)
**Goal:** Capture relevant events from `libp2p` and feed them into the telemetry system.

1.  **Connection Events:**
    *   Instrument `SwarmEvent::ConnectionEstablished` and `ConnectionClosed` in `p2p.rs`.
    *   Send `PeerConnected`/`PeerDisconnected` events.

2.  **Traffic Events:**
    *   Instrument `Gossipsub` message handling.
    *   Send `MessageReceived` events when DMs or Logs arrive.

### Phase 3: Frontend Visualization (The "War Room")
**Goal:** Render the data visually.

1.  **Project Setup:**
    *   Initialize a modern frontend project (e.g., `web-dashboard/` with Vite/React).
    *   *Alternative:* Enhance existing `index.html` with `force-graph` via CDN for immediate prototyping.

2.  **Topology Graph:**
    *   Implement `ForceGraph2D` or `ForceGraph3D`.
    *   **Nodes:** Represent Peers (Local Node = Center/Distinct Color).
    *   **Links:** Represent active connections.

3.  **Real-time Animations:**
    *   On `MessageSent` event: Emit a "particle" traveling along the link between nodes.
    *   Visual feedback for "Log" broadcasts (e.g., ripple effect from sender).

4.  **Geolocalized Map (Optional Extension):**
    *   Integrate a map view.
    *   Plot nodes based on IP geolocation.
    *   Draw arcs for connections.

## 4. Task Breakdown

- [ ] **Backend: Telemetry Core**
    - [ ] Define `NetworkEvent` enum.
    - [ ] Add `telemetry_tx` to `AppState`.
    - [ ] Implement `/ws` endpoint in `http.rs`.
- [ ] **Backend: Instrumentation**
    - [ ] Emit events on Connection/Disconnection.
    - [ ] Emit events on Message Send/Receive.
- [ ] **Frontend: Visualization**
    - [ ] Connect to WebSocket.
    - [ ] Integrate Graph Library.
    - [ ] Render Nodes and Links dynamically.
    - [ ] Implement particle effects for messages.
