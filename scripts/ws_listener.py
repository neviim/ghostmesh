import asyncio
import websockets
import json
import argparse

async def listen(uri):
    print(f"Connecting to {uri}...")
    try:
        async with websockets.connect(uri) as websocket:
            print(f"Connected! Listening for events...")
            while True:
                try:
                    message = await websocket.recv()
                    data = json.loads(message)
                    print(f"\n[Event Received] Type: {data.get('type')}")
                    print(json.dumps(data, indent=2))
                except websockets.exceptions.ConnectionClosed:
                    print("Connection closed by server.")
                    break
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="GhostMesh WebSocket Listener")
    parser.add_argument("--port", type=int, default=8071, help="Port of the GhostMesh node (default: 8071)")
    parser.add_argument("--host", type=str, default="127.0.0.1", help="Host of the GhostMesh node (default: 127.0.0.1)")
    
    args = parser.parse_args()
    uri = f"ws://{args.host}:{args.port}/ws"
    
    try:
        asyncio.run(listen(uri))
    except KeyboardInterrupt:
        print("\nExiting...")
