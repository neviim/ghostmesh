# GhostMesh

**GhostMesh** é um orquestrador P2P (Peer-to-Peer) leve para IoT, desenvolvido em Rust. Ele permite que dispositivos formem uma "nuvem local" autônoma, segura e resiliente, sem depender de servidores centrais ou conexão com a internet.

## 🚀 Funcionalidades

*   **Auto-Descoberta (mDNS):** Nós se encontram automaticamente na rede local.
*   **Memória Compartilhada (CRDTs):** Logs e estados são sincronizados entre todos os nós com consistência eventual.
*   **Dashboard Web:** Interface visual moderna para monitorar peers e logs em tempo real.
*   **Segurança:** Identidade persistente (Ed25519) e canais criptografados (Noise Protocol).
*   **Resiliência:** A rede continua operando mesmo se nós caírem ou forem reiniciados.

## 📦 Instalação

Pré-requisitos: [Rust](https://www.rust-lang.org/tools/install) instalado.

```bash
# 1. Clone o repositório (se aplicável) ou entre na pasta
cd ghostmesh

# 2. Compile o projeto (modo release para melhor performance)
cargo build --release
```

## ▶️ Como Executar

Para simular uma rede mesh, execute múltiplos nós em terminais diferentes, variando a porta.

**Nó 1:**
```bash
./target/release/ghostmesh --port 8080
```
*   P2P: Porta 8080
*   Dashboard: [http://localhost:8081](http://localhost:8081)

**Nó 2:**
```bash
./target/release/ghostmesh --port 8082
```
*   P2P: Porta 8082
*   Dashboard: [http://localhost:8083](http://localhost:8083)

> **Nota:** O Dashboard Web sempre roda na porta `P2P + 1`.

## 💻 Comandos

Você pode interagir com o GhostMesh via **Terminal** ou **Web Dashboard**.

### Via Terminal (CLI)

Digite estes comandos diretamente no terminal onde o nó está rodando:

| Comando | Descrição | Exemplo |
| :--- | :--- | :--- |
| `/peers` | Lista os IDs dos nós conectados atualmente. | `/peers` |
| `/log <msg>` | Adiciona uma mensagem ao log compartilhado e propaga para a rede. | `/log Alarme Disparado!` |
| `/show` | Exibe o conteúdo atual do log local. | `/show` |

### Via Web Dashboard

Acesse a URL do dashboard (ex: `http://localhost:8081`) para:
*   Visualizar a contagem de peers em tempo real.
*   Ler o log compartilhado.
*   Enviar novas mensagens de log via interface gráfica.

## 📚 Documentação Adicional

*   [Casos de Uso](doc/USE_CASES.md): Onde aplicar o GhostMesh.
*   [Instruções Detalhadas](doc/RUN_INSTRUCTIONS.md): Guia passo-a-passo de execução.
