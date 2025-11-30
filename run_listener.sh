#!/bin/bash

# Define the virtual environment directory
VENV_DIR=".venv"
PYTHON_BIN="$VENV_DIR/bin/python3"
PIP_BIN="$VENV_DIR/bin/pip"

# Check if the virtual environment exists
if [ ! -d "$VENV_DIR" ]; then
    echo "Creating Python virtual environment in $VENV_DIR..."
    if ! python3 -m venv "$VENV_DIR"; then
        echo "Error: Failed to create virtual environment."
        echo "On Ubuntu/Debian, please run: sudo apt install -y python3-venv"
        exit 1
    fi
fi

# Install dependencies if not already installed (checking for websockets)
if ! "$PYTHON_BIN" -c "import websockets" 2>/dev/null; then
    echo "Installing 'websockets' library..."
    "$PIP_BIN" install websockets
fi

# Run the listener script
echo "Starting WebSocket Listener..."
"$PYTHON_BIN" scripts/ws_listener.py "$@"
