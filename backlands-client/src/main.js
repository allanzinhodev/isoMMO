import { showLogin } from './ui/login.js';
import { showCharSelect } from './ui/char-select.js';
import { renderMap } from './game/renderer.js';
import { Character } from './game/character.js';
import { MAP_COLS, MAP_ROWS } from './game/map.js';
import { state } from './state.js';
import { send, disconnect } from './network/socket.js';

let debugGrid = false;
let activeLoopId = null;
const keys = {};

function startGame(container) {
  container.innerHTML = '';

  const serverPlayer = state.get('selectedCharacter');
  const startCol = serverPlayer?.pos_x ?? 5;
  const startRow = serverPlayer?.pos_y ?? 5;
  const looktype = serverPlayer?.looktype ?? 0;

  const canvas = document.getElementById('game-canvas');
  canvas.style.display = 'block';
  canvas.style.background = '#1a1a2e';
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  const ctx = canvas.getContext('2d');
  ctx.imageSmoothingEnabled = false;

  const player = new Character(startCol, startRow, looktype);
  player.direction = 1;

  const onResize = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    ctx.imageSmoothingEnabled = false;
  };
  window.addEventListener('resize', onResize);

  const onKeyDown = (e) => {
    keys[e.key] = true;
    if (e.key === 'g' || e.key === 'G') debugGrid = !debugGrid;
  };
  const onKeyUp = (e) => { keys[e.key] = false; };
  window.addEventListener('keydown', onKeyDown);
  window.addEventListener('keyup', onKeyUp);

  activeLoopId = { running: true, onResize, onKeyDown, onKeyUp };

  function updateMovement(now) {
    player.update(now);
    if (player.isMoving()) return;

    const movingRight = keys['ArrowRight'] || keys['d'] || keys['D'];
    const movingLeft  = keys['ArrowLeft']  || keys['a'] || keys['A'];
    const movingDown  = keys['ArrowDown']  || keys['s'] || keys['S'];
    const movingUp    = keys['ArrowUp']    || keys['w'] || keys['W'];

    if (!movingRight && !movingLeft && !movingDown && !movingUp) return;

    let newCol = player.col;
    let newRow = player.row;
    let dir    = player.direction;

    if      (movingRight) { newCol++; dir = 3; }
    else if (movingLeft)  { newCol--; dir = 9; }
    else if (movingDown)  { newRow++; dir = 7; }
    else if (movingUp)    { newRow--; dir = 1; }

    newCol = Math.max(0, Math.min(MAP_COLS - 1, newCol));
    newRow = Math.max(0, Math.min(MAP_ROWS - 1, newRow));

    player.moveTo(newCol, newRow, dir);
    send('move', { direction: dir });
  }

  function loop(now) {
    if (!activeLoopId?.running) return;
    updateMovement(now);
    const w = canvas.width, h = canvas.height;
    renderMap(ctx, w, h, debugGrid);
    player.draw(ctx, w / 2, h / 4);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
}

function stopGame() {
  if (activeLoopId) {
    activeLoopId.running = false;
    window.removeEventListener('resize', activeLoopId.onResize);
    window.removeEventListener('keydown', activeLoopId.onKeyDown);
    window.removeEventListener('keyup', activeLoopId.onKeyUp);
    activeLoopId = null;
  }
  const canvas = document.getElementById('game-canvas');
  canvas.style.display = 'none';
}

const routes = {
  '#login':       showLogin,
  '#select-char': showCharSelect,
  '#game':        startGame,
};

window.addEventListener('hashchange', () => {
  if (location.hash === '#login') disconnect();
  if (location.hash !== '#game') stopGame();
  render(location.hash);
});

function render(hash) {
  const uiRoot = document.getElementById('ui-root');
  uiRoot.innerHTML = '';
  const handler = routes[hash] ?? routes['#login'];
  handler(uiRoot);
}

render(location.hash || '#login');
