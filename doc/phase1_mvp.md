# GhostMesh - Phase 1: MVP (UDP Gossip)

## Visão Geral
O **GhostMesh** é uma ferramenta de orquestração de redes P2P efêmeras. Nesta primeira fase (MVP), implementamos a infraestrutura básica de comunicação descentralizada usando Rust e `libp2p`.

## Arquitetura

### Stack Tecnológico
- **Linguagem**: Rust (Edition 2021)
- **Runtime Async**: `tokio`
- **Networking**: `libp2p` (v0.53)
- **Transporte**:
    - **QUIC** (UDP): Prioritário para baixa latência.
    - **TCP**: Fallback para confiabilidade.
- **Segurança**: `noise` (Criptografia de handshake e stream).
- **Multiplexação**: `yamux`.
- **Descoberta**: `mdns` (Multicast DNS) para LAN discovery.
- **Protocolo de Mensagem**: `gossipsub` (Mesh pub/sub).

### Componentes Principais (`src/p2p.rs`)

1.  **`MyBehaviour`**: Struct que combina os comportamentos de rede:
    - `gossipsub`: Gerencia a propagação de mensagens em malha.
    - `mdns`: Detecta outros nós na mesma rede local automaticamente.

2.  **`create_swarm`**:
    - Gera uma identidade (Keypair Ed25519) temporária.
    - Configura o transporte (TCP/QUIC + Noise + Yamux).
    - Inicializa o `Swarm` (o "motor" do libp2p).

3.  **Event Loop (`run_node`)**:
    - Monitora `stdin` para entrada do usuário.
    - Monitora eventos do `Swarm` (novos peers, mensagens recebidas).
    - Realiza o *dial* automático para peers descobertos via mDNS.

## Como Executar

### Pré-requisitos
- Rust e Cargo instalados.

### Comandos
Para rodar múltiplos nós na mesma máquina, use portas diferentes:

**Terminal 1 (Nó A):**
```bash
cargo run -- --port 8080
```

**Terminal 2 (Nó B):**
```bash
cargo run -- --port 8081
```

### Uso
- Digite qualquer texto e pressione Enter para enviar a mensagem para todos os nós conectados.
- Os logs mostrarão quando novos peers forem descobertos (`mDNS discovered a new peer`) e quando mensagens forem recebidas.
