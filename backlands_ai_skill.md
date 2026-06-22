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

### WebSocket Protocol (JSON)
All messages: `{ "op": "<name>", "data": { ... } }`

**Client → Server:**
| op | data | description |
|----|------|-------------|
| `login` | `{ username, password }` | Authenticate — responds with `char_list` |
| `select_char` | `{ player_id }` | Enter game — responds with `enter_game` |
| `move` | `{ direction }` | Move player (1=NE, 3=SE, 7=SW, 9=NW) |
| `logout` | — | End session |

**Server → Client:**
| op | data | description |
|----|------|-------------|
| `char_list` | `{ players: [{id, name, vocation, looktype}] }` | After login |
| `enter_game` | `{ player: {id, name, vocation, looktype, pos_x, pos_y} }` | After char select |
| `player_moved` | `{ pos_x, pos_y, direction }` | Movement confirmation |
| `login_fail` | `{ reason }` | Bad credentials |
| `error` | `{ reason }` | Generic error |

### File Structure
```
backlands-server/
  Cargo.toml          — tokio, tokio-tungstenite, sqlx (mysql), argon2, serde_json, dotenvy
  schema.sql          — CREATE TABLE accounts + players (runs on startup)
  seed.sql            — reference only, seed runs in code on first boot
  .env                — DATABASE_URL + SERVER_ADDR
  src/
    main.rs            — entry: load .env, connect MySQL, run migrations, seed, start WS
    db/mod.rs          — create_pool, run_migrations, seed_default_account, verify_account,
                         get_players, get_player, save_player_pos
    game/world.rs      — MAP_COLS=10, MAP_ROWS=10
    network/mod.rs     — WS accept loop, per-connection session state machine (async)
    network/protocol.rs — ClientMsg / ServerMsg enums (serde)
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
      socket.js          — WebSocket wrapper: connect(), send(op, data), on(op, fn), disconnect()
    ui/
      login.js           — connect WS → send login → on char_list → #select-char
      char-select.js     — renders char cards with sprite idle from server data → send select_char
    game/
      map.js             — TILE_W=32, TILE_H=16, 10×10 MAP_DATA, TILE enum
      renderer.js        — isoToScreen(), renderMap(), drawTile() with painter's algorithm, debug grid (G)
      character.js       — Character class: smooth movement, direction system, sprite draw with flip
```

### WebSocket Flow
1. **Login** → `connect()` to `ws://localhost:7171` → `send('login', {username, password})` → receive `char_list`
2. **Char Select** → renders server players with idle sprite preview (canvas per card) → `send('select_char', {player_id})` → receive `enter_game`
3. **Game** → spawns character at `pos_x/pos_y` from server, WASD moves → `send('move', {direction})` each step

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
- [ ] WebSocket → server protocol (JSON op/data format)
