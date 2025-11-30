# GhostMesh WebSocket API

GhostMesh exposes a WebSocket endpoint to stream real-time network telemetry events. This allows external tools, dashboards, and scripts to monitor the P2P network status, connections, and message traffic.

## Endpoint

**URL:** `ws://<HOST>:<PORT>/ws`

*   **Default Host:** `127.0.0.1`
*   **Default Port:** `8071` (for the first node), `8081`, `8091`, etc.

## Prerequisites (Ubuntu/Debian)

If you are running on a modern Ubuntu system, you must install the `python3-venv` package to run the example scripts:

```bash
sudo apt install -y python3-venv
```

## Event Format

All messages are sent as JSON objects with a `type` and `data` field.

### 1. Peer Connected
Triggered when a new P2P connection is established.

```json
{
  "type": "PeerConnected",
  "data": {
    "peer_id": "12D3KooW..."
  }
}
```

### 2. Peer Disconnected
Triggered when a connection is closed.

```json
{
  "type": "PeerDisconnected",
  "data": {
    "peer_id": "12D3KooW..."
  }
}
```

### 3. Message Sent
Triggered when this node sends a message (DM).

```json
{
  "type": "MessageSent",
  "data": {
    "from": "12D3KooW... (Local ID)",
    "to": "12D3KooW... (Target ID)",
    "protocol": "DM"
  }
}
```

### 4. Message Received
Triggered when this node receives and successfully decrypts a message.

```json
{
  "type": "MessageReceived",
  "data": {
    "from": "12D3KooW... (Sender ID)",
    "to": "12D3KooW... (Local ID)",
    "protocol": "DM"
  }
}
```

## Usage Examples

### Option 1: Automated Script (Recommended)
Use the helper script to handle dependencies automatically:

```bash
./run_listener.sh --port 8071
```

### Option 2: Manual Python Setup
If you prefer to run it manually, use a virtual environment:

```bash
# Create and activate venv
python3 -m venv .venv
source .venv/bin/activate

# Install dependencies
pip install websockets

# Run script
python3 scripts/ws_listener.py --port 8071
```

### Option 2: Command Line (wscat)
You can use `wscat` for quick testing.

```bash
npm install -g wscat
wscat -c ws://127.0.0.1:8071/ws
```

### Option 3: JavaScript (Browser)
```javascript
const ws = new WebSocket('ws://localhost:8071/ws');

ws.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    console.log('New Event:', msg);
};
```
