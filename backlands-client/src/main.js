import { showLogin }     from './ui/login.js';
import { showCharSelect } from './ui/char-select.js';
import { renderMap }      from './game/renderer.js';
import { Character }      from './game/character.js';
import { MAP_COLS, MAP_ROWS, setMapData } from './game/map.js';
import { addCreature, removeCreature, moveCreature, updateAll, getCreatures, clear as clearCreatures } from './game/otherplayers.js';
import { state }          from './state.js';
import { send, on, disconnect } from './network/socket.js';
import { PacketWriter }   from './network/packet.js';
import {
  C_WALK_NORTH, C_WALK_EAST, C_WALK_SOUTH, C_WALK_WEST,
  S_FULL_MAP, S_PLAYER_DATA,
  S_CREATE_ON_MAP, S_DELETE_ON_MAP, S_MOVE_CREATURE,
} from './network/opcodes.js';

let debugGrid  = false;
let activeLoop = null;
const keys     = {};

function startGame(container) {
  container.innerHTML = '';

  const char     = state.get('selectedCharacter');
  const startCol = char?.pos_x   ?? 5;
  const startRow = char?.pos_y   ?? 5;
  const looktype = char?.looktype ?? 0;

  const canvas = document.getElementById('game-canvas');
  canvas.style.display    = 'block';
  canvas.style.background = '#1a1a2e';
  canvas.width  = window.innerWidth;
  canvas.height = window.innerHeight;

  const ctx = canvas.getContext('2d');
  ctx.imageSmoothingEnabled = false;

  clearCreatures();

  const player = new Character(startCol, startRow, looktype);
  player.direction = 1;

  // ── Server → Client handlers ─────────────────────────────────────────────

  // Full map from server
  on(S_FULL_MAP, (pkt) => {
    const cols = pkt.readU8();
    const rows = pkt.readU8();
    const data = [];
    for (let r = 0; r < rows; r++) {
      const row = [];
      for (let c = 0; c < cols; c++) row.push(pkt.readU8());
      data.push(row);
    }
    setMapData(data);
  });

  // Server-authoritative position — snap back if walk was rejected
  on(S_PLAYER_DATA, (pkt) => {
    const sx  = pkt.readU16();
    const sy  = pkt.readU16();
    const dir = pkt.readU8();
    if (player.col !== sx || player.row !== sy) {
      player.snapTo(sx, sy, dir);
    }
  });

  // Another player appeared in range
  on(S_CREATE_ON_MAP, (pkt) => {
    const id       = pkt.readU32();
    const x        = pkt.readU16();
    const y        = pkt.readU16();
    const dir      = pkt.readU8();
    /*const name  =*/ pkt.readString(); // reserved for nameplate (Phase D/E)
    const looktype = pkt.readU8();
    addCreature(id, x, y, dir, looktype);
  });

  // Another player left range or disconnected
  on(S_DELETE_ON_MAP, (pkt) => {
    removeCreature(pkt.readU32());
  });

  // Another player moved
  on(S_MOVE_CREATURE, (pkt) => {
    const id  = pkt.readU32();
    const x   = pkt.readU16();
    const y   = pkt.readU16();
    const dir = pkt.readU8();
    moveCreature(id, x, y, dir);
  });

  // ── Input ────────────────────────────────────────────────────────────────

  const onResize = () => {
    canvas.width  = window.innerWidth;
    canvas.height = window.innerHeight;
    ctx.imageSmoothingEnabled = false;
  };
  const onKeyDown = (e) => {
    keys[e.key] = true;
    if (e.key === 'g' || e.key === 'G') debugGrid = !debugGrid;
  };
  const onKeyUp = (e) => { keys[e.key] = false; };

  window.addEventListener('resize',  onResize);
  window.addEventListener('keydown', onKeyDown);
  window.addEventListener('keyup',   onKeyUp);

  activeLoop = { running: true, onResize, onKeyDown, onKeyUp };

  // ── Game loop ────────────────────────────────────────────────────────────

  function updateMovement(now) {
    player.update(now);
    if (player.isMoving()) return;

    const right = keys['ArrowRight'] || keys['d'] || keys['D'];
    const left  = keys['ArrowLeft']  || keys['a'] || keys['A'];
    const down  = keys['ArrowDown']  || keys['s'] || keys['S'];
    const up    = keys['ArrowUp']    || keys['w'] || keys['W'];

    if (!right && !left && !down && !up) return;

    let newCol = player.col;
    let newRow = player.row;
    let dir    = player.direction;
    let opcode;

    if      (right) { newCol++; dir = 3; opcode = C_WALK_EAST;  }
    else if (left)  { newCol--; dir = 9; opcode = C_WALK_WEST;  }
    else if (down)  { newRow++; dir = 7; opcode = C_WALK_SOUTH; }
    else            { newRow--; dir = 1; opcode = C_WALK_NORTH; }

    newCol = Math.max(0, Math.min(MAP_COLS - 1, newCol));
    newRow = Math.max(0, Math.min(MAP_ROWS - 1, newRow));

    player.moveTo(newCol, newRow, dir);
    send(new PacketWriter().build(opcode));
  }

  function loop(now) {
    if (!activeLoop?.running) return;

    updateMovement(now);
    updateAll(now); // animate other players

    const w = canvas.width;
    const h = canvas.height;
    const offsetX = w / 2;
    const offsetY = h / 4;

    renderMap(ctx, w, h, debugGrid);

    const entities = [player, ...getCreatures()];
    entities.sort((a, b) => (a._visualCol + a._visualRow) - (b._visualCol + b._visualRow));

    for (const ent of entities) {
      ent.draw(ctx, offsetX, offsetY);
    }

    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
}

function stopGame() {
  if (activeLoop) {
    activeLoop.running = false;
    window.removeEventListener('resize',  activeLoop.onResize);
    window.removeEventListener('keydown', activeLoop.onKeyDown);
    window.removeEventListener('keyup',   activeLoop.onKeyUp);
    activeLoop = null;
  }
  document.getElementById('game-canvas').style.display = 'none';
  clearCreatures();
}

const routes = {
  '#login':       showLogin,
  '#select-char': showCharSelect,
  '#game':        startGame,
};

window.addEventListener('hashchange', () => {
  if (location.hash === '#login') disconnect();
  if (location.hash !== '#game')  stopGame();
  render(location.hash);
});

function render(hash) {
  const uiRoot = document.getElementById('ui-root');
  uiRoot.innerHTML = '';
  (routes[hash] ?? routes['#login'])(uiRoot);
}

render(location.hash || '#login');
