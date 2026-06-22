# Backlands MMORPG — Próximos Passos

> Baseado no mapa de opcodes do TFS/OTClient em `opcode_map.md`.
> Estado atual: client browser funcional + servidor Rust com login/char select/movimento básico via JSON/WebSocket.

---

## O que já temos implementado

| Funcionalidade | Client | Server |
|----------------|--------|--------|
| Login (usuario/senha) | ✅ | ✅ Argon2id + MySQL |
| Lista de personagens | ✅ com sprite idle | ✅ query MySQL |
| Entrar no jogo | ✅ posição salva | ✅ persiste pos_x/pos_y |
| Mapa isométrico (hardcoded) | ✅ 10×10 tiles | — |
| Movimento WASD (smooth) | ✅ 150ms ease-in-out | ✅ valida bounds |
| 3 classes de personagem | ✅ Black Mage, Hunter, Blue Mage | ✅ vocation + looktype |

---

## Fase A — Migrar Protocolo: JSON → Binário TFS

O servidor atual usa JSON. Para compatibilidade futura com OTClient e para suportar volumes maiores de dados (mapa, criaturas), precisamos migrar para o protocolo binário do TFS.

### A1. ByteBuffer (Rust)
Criar `src/utils/byte_buffer.rs` com:
- `write_u8`, `write_u16_le`, `write_u32_le`, `write_string` (u16 len + bytes)
- `read_u8`, `read_u16_le`, `read_u32_le`, `read_string`
- Baseado em Little-Endian, igual ao TFS

### A2. Estrutura de pacote TFS
```
[Length: 2B LE][Opcode: 1B][Payload: N bytes]
```
Substituir o `serde_json` no `network/mod.rs` por framing binário.

### A3. Opcodes prioritários a implementar

**Login Server (porta separada ou mesmo WS):**
| Opcode | Hex | Prioridade |
|--------|-----|------------|
| `ClientEnterAccount` | 0x01 | Alta — autenticação |
| `LoginServerCharacterList` | 0x64 | Alta — retorna lista de chars |
| `LoginServerError` | 0x0A | Alta — falha de login |
| `GameServerChallenge` | 0x1F | Média — XOR challenge |

**Game Server:**
| Opcode | Hex | Prioridade |
|--------|-----|------------|
| `GameServerLoginSuccess` | 0x17 | Alta |
| `GameServerFullMap` | 0x64 | Alta — enviar mapa ao entrar |
| `GameServerCreateOnMap` | 0x6A | Alta — spawnar criatura |
| `GameServerMoveCreature` | 0x6D | Alta — movimento de criaturas |
| `ClientWalkNorth/East/South/West` | 0x65–0x68 | Alta — movimento |
| `ClientTurnNorth/East/South/West` | 0x6F–0x72 | Média — direção sem movimento |
| `GameServerPlayerData` | 0xA0 | Alta — stats do player |
| `GameServerTalk` | 0xAA | Média — chat |
| `ClientTalk` | 0x96 | Média — chat |
| `GameServerPing` / `ClientPingBack` | 0x1E / 0x1D | Alta — keep-alive |
| `GameServerDeath` | 0x28 | Média — morte |
| `GameServerTextMessage` | 0xB4 | Média — mensagens do sistema |

---

## Fase B — Mapa Dinâmico (Servidor)

Atualmente o mapa é hardcoded no client. Para um MMORPG real o servidor precisa enviar os dados do mapa.

### B1. Estrutura de Tile no Servidor
```rust
struct Tile {
    ground_id: u16,   // ID do tile (grama, terra, água, areia)
    object_id: u16,   // Item/objeto no tile (0 = vazio)
    flags: u8,        // bloqueável, passável, etc.
    z_height: u8,     // nível de altura (floor)
}

struct MapChunk {
    tiles: [[Tile; 16]; 16],  // chunk 16×16
}
```

### B2. Opcode `GameServerFullMap` (0x64)
Ao entrar no jogo, enviar os tiles visíveis ao redor do player (viewport de 15×11 tiles, padrão TFS).

### B3. Opcodes de scroll de mapa
Quando o player se move, enviar somente a nova linha/coluna:
- `GameServerMapTopRow` (0x65) — moveu norte
- `GameServerMapRightRow` (0x66) — moveu leste
- `GameServerMapBottomRow` (0x67) — moveu sul
- `GameServerMapLeftRow` (0x68) — moveu oeste

---

## Fase C — Múltiplos Jogadores

### C1. Gerenciamento de criaturas no servidor
- `GameServerCreateOnMap` (0x6A) — notificar outros players quando alguém entra na área visível
- `GameServerMoveCreature` (0x6D) — broadcast de movimento para players na área
- `GameServerDeleteOnMap` (0x6C) — notificar saída da área visível / desconexão

### C2. Estado global compartilhado
```rust
// Arc<RwLock<GameWorld>> compartilhado entre todas as conexões
struct GameWorld {
    players: HashMap<u32, PlayerState>,
}
```

### C3. Broadcast de posição
Quando um player se move, o servidor deve notificar todos os players no mesmo viewport.

---

## Fase D — Combate Básico (COMPLETED)

| Opcode | Hex | Descrição | Status |
|--------|-----|-----------|--------|
| `ClientAttack` | 0xA1 | Iniciar ataque em criatura | ✅ Feito |
| `ClientChangeFightModes` | 0xA0 | Modo de combate | ✅ Feito (Ignorado) |
| `GameServerCreatureHealth` | 0x8C | Atualizar HP de criatura | ✅ Feito |
| `GameServerGraphicalEffect` | 0x83 | Efeito visual de spell/ataque | ✅ Feito |
| `GameServerTextEffect` | 0x84 | Número de dano flutuante | ✅ Feito |
| `GameServerMissleEffect` | 0x85 | Projétil animado | Ignorado |
| `GameServerDeath` | 0x28 | Morte do personagem | ✅ Feito |

---

## Fase E — Chat (COMPLETED)

| Opcode | Hex | Prioridade | Status |
|--------|-----|------------|--------|
| `ClientTalk` | 0x96 | Alta | ✅ Feito |
| `GameServerTalk` | 0xAA | Alta | ✅ Feito |
| `ClientRequestChannels` | 0x97 | Média | Ignorado por hora |
| `GameServerChannels` | 0xAB | Média | Ignorado por hora |
| `ClientJoinChannel` | 0x98 | Média | Ignorado por hora |
| `GameServerOpenChannel` | 0xAC | Média | Ignorado por hora |

---

## Fase F — Port OTClient

Após o servidor ter protocolo binário TFS funcional:

1. Compilar OTClient apontando para `ws://localhost:7171`
2. Configurar `.otclient` para usar nosso servidor
3. Migrar assets (PNG → SPR/DAT)
4. Replicar lógica de direção/animação em Lua (`modules/game_backlands/`)
5. Testar com OTClient nativo como client

---

## Ordem de Implementação Recomendada

```
A1 ByteBuffer → A2 Protocolo binário → A3 Opcodes login
→ B1/B2 Mapa dinâmico → C1/C2/C3 Multiplayer
→ D Combate → E Chat → F OTClient
```

---

## Referências

- TFS src: https://github.com/otland/forgottenserver/blob/master/src/protocolgame.cpp
- OTClient codes: https://github.com/otland/otclient/blob/master/src/client/protocolgame.h
- Mapa completo de opcodes: `docs/planning/opcode_map.md`
