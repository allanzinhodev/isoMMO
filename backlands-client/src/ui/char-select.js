import { state } from '../state.js';
import { send, on } from '../network/socket.js';
import { PacketWriter } from '../network/packet.js';
import { C_ENTER_GAME, S_ENTER_GAME } from '../network/opcodes.js';

const SPRITE_W = 16;
const SPRITE_H = 32;
const SCALE    = 3;
const IDLE_FRAME   = { 0: 3, 1: 0, 2: 3 }; // vocation → idle frame
const VOCATION_NAME = { 0: 'Black Mage', 1: 'Hunter', 2: 'Blue Mage' };

import { getCharSprite } from '../game/assets.js';

function drawCharPreview(canvas, looktype) {
  const ctx = canvas.getContext('2d');
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.imageSmoothingEnabled = false;
  
  const frame = IDLE_FRAME[looktype] ?? 0;
  const sprite = getCharSprite(looktype, frame);
  if (!sprite) return;
  
  const dw = SPRITE_W * SCALE;
  const dh = SPRITE_H * SCALE;
  const dx = (canvas.width  - dw) / 2;
  const dy = (canvas.height - dh) / 2;
  
  ctx.drawImage(sprite, dx, dy, dw, dh);
}

export function showCharSelect(container) {
  const players     = state.get('charList') || [];
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
      <canvas width="80" height="80" class="char-preview"></canvas>
      <div class="char-name">${char.name}</div>
      <div class="char-class">${VOCATION_NAME[char.vocation] ?? 'Desconhecido'}</div>
      <button class="btn-select" data-id="${char.id}">Selecionar</button>
    `;
    listContainer.appendChild(card);
    drawCharPreview(card.querySelector('.char-preview'), char.looktype);
  });

  document.getElementById('btn-back').addEventListener('click', () => {
    location.hash = '#login';
  });

  on(S_ENTER_GAME, (pkt) => {
    state.set('selectedCharacter', {
      id:       pkt.readU32(),
      name:     pkt.readString(),
      vocation: pkt.readU8(),
      looktype: pkt.readU8(),
      pos_x:    pkt.readU16(),
      pos_y:    pkt.readU16(),
    });
    location.hash = '#game';
  });

  container.querySelectorAll('.btn-select').forEach(btn => {
    btn.addEventListener('click', (e) => {
      const pw = new PacketWriter();
      pw.writeU32(parseInt(e.target.getAttribute('data-id')));
      send(pw.build(C_ENTER_GAME));
    });
  });
}
