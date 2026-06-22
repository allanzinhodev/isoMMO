import { state } from '../state.js';
import { send, on } from '../network/socket.js';

const SPRITE_W = 16;
const SPRITE_H = 32;
const SCALE    = 3;
const IDLE_FRAME = { 0: 3, 1: 0, 2: 3 }; // vocation → idle frame index

const VOCATION_NAME = { 0: 'Black Mage', 1: 'Hunter', 2: 'Blue Mage' };

function drawCharPreview(canvas, looktype) {
  const sheet = new Image();
  sheet.src = 'src/assets/character.png';
  const draw = () => {
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.imageSmoothingEnabled = false;
    const frame = IDLE_FRAME[looktype] ?? 0;
    const dw = SPRITE_W * SCALE;
    const dh = SPRITE_H * SCALE;
    const dx = (canvas.width - dw) / 2;
    const dy = (canvas.height - dh) / 2;
    ctx.drawImage(sheet, frame * SPRITE_W, looktype * SPRITE_H, SPRITE_W, SPRITE_H, dx, dy, dw, dh);
  };
  if (sheet.complete) draw();
  else sheet.onload = draw;
}

export function showCharSelect(container) {
  const players = state.get('charList') || [];
  const currentUser = state.get('currentUser') || 'Jogador';

  const wrap = document.createElement('div');
  wrap.className = 'card';
  wrap.innerHTML = `
    <h2>Bem-vindo, ${currentUser}</h2>
    <p>Selecione o seu personagem:</p>
    <div id="char-list" class="char-list"></div>
    <br>
    <button id="btn-back" style="background-color: var(--color-blue); margin-top: 10px;">Voltar</button>
  `;
  container.appendChild(wrap);

  const listContainer = document.getElementById('char-list');

  players.forEach(char => {
    const card = document.createElement('div');
    card.className = 'char-card';
    card.innerHTML = `
      <canvas width="80" height="80" class="char-preview" data-looktype="${char.looktype}"></canvas>
      <div class="char-name">${char.name}</div>
      <div class="char-class">${VOCATION_NAME[char.vocation] ?? 'Desconhecido'}</div>
      <button class="btn-select" data-id="${char.id}">Selecionar</button>
    `;
    listContainer.appendChild(card);

    const canvas = card.querySelector('.char-preview');
    drawCharPreview(canvas, char.looktype);
  });

  document.getElementById('btn-back').addEventListener('click', () => {
    location.hash = '#login';
  });

  on('enter_game', (data) => {
    state.set('selectedCharacter', data.player);
    location.hash = '#game';
  });

  container.querySelectorAll('.btn-select').forEach(btn => {
    btn.addEventListener('click', (e) => {
      const playerId = parseInt(e.target.getAttribute('data-id'));
      send('select_char', { player_id: playerId });
    });
  });
}
