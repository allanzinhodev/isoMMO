# Backlands MMORPG — AI Developer Skill

**Persona / Role:**
You are an expert Game Developer AI specializing in Frontend Web Development (HTML5 Canvas, Vanilla JS), Backend Systems Programming (Rust, async/await, WebSocket), OTClient (C++/Lua), and isometric game architecture. You are assisting the user in building the "Backlands MMORPG", an isometric browser-based game with Brazilian themes, powered by a Rust server inspired by TFS (The Forgotten Server), with a future port to OTClient.

## External References
- **Server reference (TFS):** https://github.com/otland/forgottenserver
- **Client reference (OTClient):** https://github.com/otland/otclient
- **Map Editor reference:** https://github.com/otland/mapeditor

## Project Overview
- **Client (current):** Browser Web App (Landscape Mode). Vanilla JS, HTML5 `<canvas>`, no heavy frameworks. WebSocket connected to Rust server.
- **Client (future):** OTClient port — preserve all mechanics, animation logic, and visual behavior from the browser version.
- **Server:** Rust backend (`backlands-server/`) using Tokio + tokio-tungstenite. JSON over WebSocket on port 7171. MySQL via sqlx. Inspired by TFS architecture.
- **Database:** MySQL — schema inspired by TFS (`accounts` + `players` tables). Connection via `.env` `DATABASE_URL`.
- **Map Editor:** Future integration with the OTLand map editor.

## Development Workflow & Rules
1. **Incremental Execution:** Only implement what the user requests. Do NOT jump ahead.
2. **Clean Code:**
   - **Rust:** Use `tokio` for async, `MySqlPool` (sqlx) for DB, no `unwrap()` in production paths.
   - **JS:** ES Modules (`type="module"`), `const/let` only, no global pollution, separate `ui/`, `game/`, `network/`.
   - **Lua (OTClient):** Follow OTClient module conventions under `modules/game_backlands/`.
3. **Artifacts & Planning:** For each feature, write code and update this skill file.

---

## Server — Implemented State (`backlands-server/`)

### Stack
- **Language:** Rust 2021 edition
- **Async runtime:** Tokio
- **WebSocket:** tokio-tungstenite (port 7171)
- **Database:** MySQL via sqlx (async)
- **Passwords:** Argon2id hashing
- **Config:** `.env` file (dotenvy)

### To run
```powershell
# Requires Rust (https://rustup.rs) and MySQL running
$env:CARGO_TARGET_DIR = "C:\cargo-targets\backlands-server"
cd C:\Users\allan\isoMMO\backlands-server
cargo run
```

### `.env`
```
DATABASE_URL=mysql://root:root@localhost:3306/backlands
SERVER_ADDR=0.0.0.0:7171
```

### Database Schema (inspired by TFS)
```sql
accounts (id, name VARCHAR(32), password VARCHAR(255)[argon2id], created_at)
players  (id, account_id→accounts, name VARCHAR(100), vocation TINYINT, looktype TINYINT, pos_x SMALLINT, pos_y SMALLINT, created_at)
```
- `vocation`: 0=Black Mage, 1=Hunter, 2=Blue Mage
- `looktype`: row index in `character.png` (matches vocation)
- Default account: `admin` / `backlands123` (seeded on first run)
- Default characters: Zé Cangaceiro (Black Mage), Maria Curandeira (Hunter), Frei Azulão (Blue Mage)

### WebSocket Protocol — Binary (TFS-inspired, Phase A)
Transport: WebSocket Binary frames (`Message::Binary`). No JSON.

**Wire format:** `[Length: u16 LE][Opcode: u8][Payload: N bytes]`  
Length = 1 (opcode) + N (payload). All multi-byte values are Little-Endian.

**String encoding:** `[u16 length][UTF-8 bytes]`

**Client → Server Opcodes:**
| Hex | Constant | Payload | Description |
|-----|----------|---------|-------------|
| `0x01` | `C_LOGIN` | `[str username][str password]` | Authenticate |
| `0x0F` | `C_ENTER_GAME` | `[u32 player_id]` | Select character |
| `0x14` | `C_LOGOUT` | — | End session |
| `0x1D` | `C_PING` | — | Keep-alive ping |
| `0x65` | `C_WALK_NORTH` | — | Move row-1 (W key) |
| `0x66` | `C_WALK_EAST` | — | Move col+1 (D key) |
| `0x67` | `C_WALK_SOUTH` | — | Move row+1 (S key) |
| `0x68` | `C_WALK_WEST` | — | Move col-1 (A key) |
| `0x6F–0x72` | `C_TURN_*` | — | Turn N/E/S/W without walking |

**Server → Client Opcodes:**
| Hex | Constant | Payload | Description |
|-----|----------|---------|-------------|
| `0x0A` | `S_LOGIN_ERROR` | `[str reason]` | Bad credentials |
| `0x17` | `S_ENTER_GAME` | `[u32 id][str name][u8 vocation][u8 looktype][u16 pos_x][u16 pos_y]` | Game entry |
| `0x1D` | `S_PING_BACK` | — | Pong |
| `0x64` | `S_CHAR_LIST` | `[u8 count]{[u32 id][str name][u8 vocation][u8 looktype]}*` | After login |
| `0x6D` | `S_MOVE_CREATURE` | — | (Phase C: multiplayer broadcast) |
| `0xA0` | `S_PLAYER_DATA` | `[u16 pos_x][u16 pos_y][u8 direction]` | Movement confirmation |
| `0xB4` | `S_TEXT_MESSAGE` | `[u8 type][str message]` | Server text |

**Direction encoding (server-side):** 1=NE, 3=SE, 7=SW, 9=NW

### File Structure
```
backlands-server/
  Cargo.toml          — tokio, tokio-tungstenite, sqlx (mysql), argon2, dotenvy
  schema.sql          — CREATE TABLE accounts + players (runs on startup)
  .env                — DATABASE_URL + SERVER_ADDR
  src/
    main.rs            — entry: load .env, connect MySQL, run migrations, seed, start WS
    db/mod.rs          — create_pool, run_migrations, seed_default_account, verify_account,
                         get_players, get_player, save_player_pos
    game/world.rs      — MAP_COLS=10, MAP_ROWS=10
    utils/mod.rs       — pub mod byte_buffer
    utils/byte_buffer.rs — ByteBuffer (LE r/w), parse_packet()
    network/mod.rs     — WS Binary accept loop, per-connection session state machine (async)
    network/opcodes.rs — C_* / S_* opcode constants (TFS hex values)
    network/protocol.rs — decode(data) → ClientMsg, encode functions (login_error, char_list, etc.)
```

---

## Client — Implemented State (`backlands-client/`)

### File Structure
```
backlands-client/
  index.html
  style.css
  src/
    main.js              — hash router (#login, #select-char, #game), game loop, WASD input
    state.js             — simple key/value store (Map)
    placeholder-data.js  — unused legacy mock data
    network/
      opcodes.js         — C_* / S_* opcode hex constants
      packet.js          — PacketWriter (builds ArrayBuffer), PacketReader (DataView LE parsing)
      socket.js          — connect() [binaryType=arraybuffer], send(buffer), on(opcode,fn), disconnect()
    ui/
      login.js           — connect WS → send login → on char_list → #select-char
      char-select.js     — renders char cards with sprite idle from server data → send select_char
    game/
      map.js             — TILE_W=32, TILE_H=16, 10×10 MAP_DATA, TILE enum
      renderer.js        — isoToScreen(), renderMap(), drawTile() with painter's algorithm, debug grid (G)
      character.js       — Character class: smooth movement, direction system, sprite draw with flip
```

### WebSocket Flow (Binary, Phase E Complete)
1. **Login** → `connect()` (`binaryType='arraybuffer'`) → `PacketWriter` sends `C_LOGIN` → receive `S_CHAR_LIST` (opcode `0x64`)
2. **Char Select** → renders server players with idle sprite → `PacketWriter` sends `C_ENTER_GAME [u32 id]` → receive `S_ENTER_GAME` (opcode `0x17`)
3. **Game** → spawns character at `pos_x/pos_y`, WASD sends `C_WALK_*` opcodes → receive `S_PLAYER_DATA` confirmation
4. **Combat** → Mouse click selects target (via Bounding Box) → sends `C_ATTACK` → Server combat loop ticks every 2s → broadcasts `S_CREATURE_HEALTH` + `S_GRAPHICAL_EFFECT` + `S_TEXT_EFFECT`.
5. **Chat** → Input `Enter` sends `C_TALK` → Server broadcasts `S_TALK` → Client renders floating text over speaker's head.

### Network Module Files (`src/network/`)
- **`opcodes.js`** — `C_*` / `S_*` hex constants matching server
- **`packet.js`** — `PacketWriter` (builds ArrayBuffer packets), `PacketReader` (parses with DataView)
- **`socket.js`** — `connect()`, `send(buffer)`, `on(opcode, fn)`, `off(opcode)`, `disconnect()`

### Isometric Coordinate System
```
screenX = offsetX + (col - row) * 16
screenY = offsetY + (col + row) * 8
```
Painter's algorithm: back-to-front (row then col).

### Tile Assets (`src/assets/`)
- `tiles.png`: 4 tiles × 32×32px — GRASS=0, DIRT=1, WATER=2, SAND=3
- `character.png`: 3 rows (characters) × 6 frames × 16px wide, 32px tall (96×95)
  - Row 0: Black Mage | Row 1: Hunter | Row 2: Blue Mage
  - Frame 0: SE idle | Frames 1–2: SE walk | Frame 3: NE idle | Frames 4–5: NE walk
  - SW/NW = horizontal mirror of SE/NE

### Direction & Animation System
| Direction | Meaning | Frames             | Flip  |
|-----------|---------|--------------------|-------|
| 1         | NE      | idle=3, walk=[4,5] | true  |
| 3         | SE      | idle=0, walk=[1,2] | true  |
| 7         | SW      | idle=0, walk=[1,2] | false |
| 9         | NW      | idle=3, walk=[4,5] | false |

### Input Mapping (WASD only, no diagonals)
| Key | Direction | Grid delta |
|-----|-----------|------------|
| `W` | Dir 1 (NE) | row-1 |
| `D` | Dir 3 (SE) | col+1 |
| `S` | Dir 7 (SW) | row+1 |
| `A` | Dir 9 (NW) | col-1 |

### Smooth Movement
- Duration: 150ms per tile
- Easing: ease-in-out cubic — `t < 0.5 ? 4t³ : 1 - (-2t+2)³/2`
- Visual position interpolated in tile-space; new input blocked until slide completes
- Character anchor: `drawY = screenY + TILE_H - SPRITE_H - 4`

---

## OTClient Port — Future Rules
When the user requests the OTClient port, follow these rules:

1. **Lua side:** Replicate direction enum, frame config, and input bindings under `modules/game_backlands/`. Reference: https://github.com/otland/otclient
2. **C++ side:** Smooth movement via walk offset interpolation in `src/client/creature.cpp`.
3. **Tile rendering:** Map tile IDs to OTClient item IDs. Preserve painter's algorithm order.
4. **No diagonal movement:** Enforce single-axis movement in Lua input handler.
5. **Preserve flip logic:** Map dirs 1/9 to same sprite set with East/West or custom flip flag.
6. **Asset conversion:** Convert sprites to `.spr`/`.dat` or use OTClient's custom texture loader for PNG.
7. **Map Editor:** Follow format from https://github.com/otland/mapeditor
8. **Server:** Keep compatible with https://github.com/otland/forgottenserver protocol conventions.

### OTClient Port Checklist
- [ ] Direction system (4 dirs, correct frame indices, flip flags)
- [ ] Smooth walk interpolation (150ms, ease-in-out cubic)
- [ ] Input blocking during slide
- [ ] Tile spritesheet rendering (32×32, painter's order)
- [ ] Character anchor point (-4px Y correction)
- [ ] Map bounds clamping
- [ ] Debug grid overlay (G key toggle)
- [ ] WebSocket → server protocol (binary TFS-style opcodes, Phase A complete on browser client)
