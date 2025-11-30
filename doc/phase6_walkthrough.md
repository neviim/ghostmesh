# Phase 6: Hybrid Transport (BLE) Walkthrough

## Goal
Integrate Bluetooth Low Energy (BLE) to enable "Presence Detection" and potential future off-grid communication between nodes.

## Changes
1.  **Dependencies**: Added `btleplug` and `uuid` to `Cargo.toml`.
2.  **BLE Service**: Created `src/ble.rs` which:
    *   Initializes the Bluetooth adapter.
    *   Starts scanning for devices.
    *   Filters for devices advertising "GhostMesh" (conceptually).
3.  **Integration**: Updated `src/main.rs` and `src/p2p.rs` to spawn the BLE service alongside the P2P and Web tasks.

## Verification
The project compiles successfully with the system dependencies (`libdbus-1-dev`).

### Logs
When running the node, you should see:
```
INFO src/ble.rs: Starting BLE Service...
INFO src/ble.rs: Using Bluetooth Adapter: ...
INFO src/ble.rs: BLE Scanning started...
```

> **Note:** Actual discovery requires two physical devices with Bluetooth adapters. Running multiple nodes on the same machine usually won't trigger BLE discovery because the adapter cannot scan its own advertisements easily.

## Next Steps
*   Implement full Advertising logic (requires more complex DBus interaction on Linux).
*   Implement a "Transport Switcher" to route messages via BLE if TCP fails.
