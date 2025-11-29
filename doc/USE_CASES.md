# Visão Geral e Casos de Uso do GhostMesh

**GhostMesh** é um sistema de orquestração P2P para IoT que cria uma "nuvem local" entre dispositivos, garantindo autonomia e resiliência sem depender de servidores centrais.

## Onde Utilizar (Casos de Uso)

O sistema brilha em cenários que exigem **autonomia local** e **resistência a falhas**:

1.  **Casa Inteligente (Smart Home)**
    *   Sincronização de estado entre cômodos (ex: interruptores e lâmpadas) sem depender do roteador Wi-Fi central ou da internet.

2.  **Indústria 4.0**
    *   Sensores em máquinas compartilhando dados de telemetria (temperatura, vibração) diretamente com painéis locais e atuadores, garantindo baixa latência e operação contínua mesmo se a rede da fábrica oscilar.

3.  **Redes Off-Grid / Emergência**
    *   Comunicação em áreas rurais, acampamentos ou situações de desastre. Dispositivos podem formar uma rede temporária (mesh) para trocar mensagens de coordenação sem infraestrutura prévia.

4.  **Robótica de Enxame**
    *   Coordenação entre múltiplos robôs ou drones (ex: "tarefa X concluída"), compartilhando um mapa lógico comum do ambiente.

## O Que Transportar (Tipos de Dados)

O GhostMesh utiliza **CRDTs (Conflict-free Replicated Data Types)**, otimizados para consistência eventual.

### ✅ Ideal Para
*   **Estado e Comandos:** "Luz: ON", "Portão: Aberto", "Válvula: 50%".
*   **Logs de Eventos:** "Movimento detectado às 14:00", "Erro no sensor 3".
*   **Configurações:** Propagação de parâmetros (ex: alterar intervalo de leitura de todos os sensores).
*   **Pequenos Objetos JSON:** Estruturas de dados leves (até alguns KBs).

### ❌ Não Recomendado (Versão Atual)
*   **Streaming de Mídia:** Vídeo ou áudio em tempo real (exige alta largura de banda e baixa latência constante).
*   **Arquivos Grandes:** Transferência de imagens pesadas ou firmwares (melhor realizado via conexão direta/stream, usando o mesh apenas para sinalização).
