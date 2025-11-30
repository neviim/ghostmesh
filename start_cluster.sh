#!/bin/bash

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 1. Compile
echo -e "${BLUE}Building GhostMesh (Release)...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed."
    exit 1
fi

BIN="./target/release/ghostmesh"
IP="192.168.15.251" # Your local IP

# Array to store PIDs
PIDS=()

# Function to start a node
start_node() {
    PORT=$1
    WEB_PORT=$((PORT + 1))
    
    echo -e "${GREEN}Starting Node on P2P Port $PORT...${NC}"
    # Run in background, redirect logs to file
    $BIN --port $PORT > "node_$PORT.log" 2>&1 &
    PID=$!
    PIDS+=($PID)
    
    echo -e "  -> PID: $PID"
    echo -e "  -> Dashboard: http://$IP:$WEB_PORT (or http://localhost:$WEB_PORT)"
    echo -e "  -> Log File: node_$PORT.log"
    echo "---------------------------------------------------"
}

# 2. Start Instances
echo -e "${BLUE}Launching Cluster...${NC}"

# Kill any existing instances to avoid port conflicts
pkill -f "ghostmesh --port" 2>/dev/null

start_node 8070
start_node 8080
start_node 8090

echo -e "${BLUE}GhostMesh Cluster is running!${NC}"
echo -e "Logs are being written to ${GREEN}node_*.log${NC} files."
echo -e "Use ${GREEN}tail -f node_8070.log${NC} to monitor a specific node."
echo "Press Ctrl+C to stop all nodes."

# 3. Cleanup on Exit
cleanup() {
    echo -e "\n${BLUE}Stopping all nodes...${NC}"
    for PID in "${PIDS[@]}"; do
        kill $PID 2>/dev/null
    done
    wait
    echo "Cluster stopped."
}

# Trap Ctrl+C (SIGINT)
trap cleanup INT

# Keep script running to maintain the trap
wait
