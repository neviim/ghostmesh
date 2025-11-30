# Como Executar o GhostMesh

Este guia explica como compilar e rodar a rede GhostMesh localmente.

## Pré-requisitos

Certifique-se de ter o Rust instalado. Se não tiver:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Configuração Permanente (Zsh)

Se você usa `zsh`, adicione o Rust ao seu PATH permanentemente:

```bash
echo 'source $HOME/.cargo/env' >> ~/.zshrc
source ~/.zshrc
```

## 1. Compilar o Projeto

Navegue até a pasta do projeto e compile:

```bash
cd ghostmesh
cargo build --release
```

## 2. Rodar os Nós

Para simular uma rede, vamos rodar dois nós em terminais separados.

### Terminal 1 (Nó A)
Este nó usará a porta P2P **8080** e o Dashboard Web na porta **8081**.

```bash
./target/release/ghostmesh --port 8080
```

### Terminal 2 (Nó B)
Este nó usará a porta P2P **8082** e o Dashboard Web na porta **8083**.

```bash
./target/release/ghostmesh --port 8082
```

> **Nota:** As identidades serão salvas automaticamente em `identity_8080.key` e `identity_8082.key`.

## 3. Acessar o Dashboard

Abra seu navegador e acesse:

*   **Nó A:** [http://localhost:8081](http://localhost:8081)
*   **Nó B:** [http://localhost:8083](http://localhost:8083)

Você verá o número de peers conectados e poderá enviar mensagens pelo campo de input.

## 4. Comandos CLI (Opcional)

Você também pode interagir diretamente pelo terminal onde o nó está rodando:

*   `/peers`: Lista os nós conectados.
*   `/log <mensagem>`: Adiciona uma mensagem ao log compartilhado.
*   `/show`: Mostra o log atual.

Exemplo:
```bash
/log Olá via Terminal!
```

## 5. Verificando a Observabilidade (War Room)

O GhostMesh possui um sistema de visualização em tempo real da topologia da rede.

### 1. Visualização do Grafo
1.  Abra o Dashboard.
2.  Clique na aba **"War Room"**.
3.  Você verá um grafo interativo onde:
    *   **Nó Azul:** É você.
    *   **Nós Verdes:** São os peers conectados.
    *   **Linhas:** Representam as conexões ativas.

### 2. Eventos em Tempo Real (Partículas)
Para ver o fluxo de dados acontecendo:

1.  Mantenha a aba **"War Room"** aberta em uma janela.
2.  Abra uma **segunda janela** do navegador (pode ser o mesmo dashboard ou de outro nó).
3.  Nessa segunda janela, vá em **"Private Messages"**.
4.  Envie uma mensagem para um peer.
5.  Olhe imediatamente para a janela da **"War Room"**.
6.  Você verá uma **partícula de luz** viajando pela linha de conexão, indicando que a mensagem foi trafegada com sucesso! ⚡️
