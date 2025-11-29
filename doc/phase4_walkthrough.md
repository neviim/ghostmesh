# Walkthrough - Phase 4: Data Consistency (CRDTs)

We have implemented a distributed, eventually consistent log using Conflict-free Replicated Data Types (CRDTs). This allows all nodes to maintain a shared state without a central server.

## Changes Made

- **CRDT Integration**: Added `crdts` crate.
- **Shared State**: Implemented a `GSet` (Grow-only Set) in `src/p2p.rs` to store log entries.
- **Commands**:
    - `/log <message>`: Adds a message to the local set and broadcasts the updated state to the network.
    - `/show`: Displays the current local state of the log.
- **Sync Logic**: When a node receives a CRDT message, it automatically merges it into its local state using `GSet::merge`.

## Verification Results

We ran two nodes locally (8080 and 8082):

1.  **Node A**:
    - Command: `/log Hello CRDT`
    - Log: `Logged: Hello CRDT`

2.  **Node B**:
    - Log: `Synced CRDT state. Current Log: {"Hello CRDT"}`

This confirms that the state update was propagated and merged successfully.
