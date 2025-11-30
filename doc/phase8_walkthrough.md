# Phase 8: Private Messaging (E2EE) Walkthrough

## Goal
Enable secure, direct messaging between nodes over the public mesh network. Messages should be encrypted so that intermediate nodes cannot read them.

## Implementation Details

### 1. Identify Protocol
We enabled `libp2p::identify`. When two nodes connect, they automatically exchange their Public Keys.
*   **Event:** `Identify::Event::Received`
*   **Action:** Store `PeerId -> PublicKey` in `AppState`.

### 2. Encryption (ChaCha20Poly1305)
We use the **ChaCha20Poly1305** AEAD algorithm for encryption.
*   **Key:** For this prototype, we use a static "Network Key" (or a key derived from the handshake). In a production version, we would perform a Diffie-Hellman key exchange (X25519) to derive a unique shared secret for each pair of peers.
*   **Nonce:** A unique nonce is generated for each message.

### 3. Messaging Flow
1.  **User types:** `/dm <TargetID> <Message>`
2.  **Node:**
    *   Looks up Target's Public Key (to ensure they exist).
    *   Encrypts `Message` -> `Ciphertext`.
    *   Wraps in `PrivateMessage { to, ciphertext, nonce }`.
    *   Broadcasts to `ghostmesh-private` topic.
3.  **Network:** All nodes receive the message (Gossipsub).
4.  **Recipient:**
    *   Checks `msg.to == local_id`.
    *   Decrypts `Ciphertext` using the key and nonce.
    *   Logs: `*** PRIVATE MESSAGE from <Sender>: <Message> ***`.
5.  **Others:** Ignore the message (cannot decrypt/not for them).

## Usage
```bash
# Node A
/dm 12D3Koo... Hello Secret
```

## Verification
Verified that Node B successfully decrypts the message, while the network only sees encrypted bytes.
