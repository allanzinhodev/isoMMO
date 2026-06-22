import { TILE_W, TILE_H, getTile, getMapCols, getMapRows } from './map.js';

const HALF_W = TILE_W / 2;
const HALF_H = TILE_H / 2;

import { getTileSprite } from './assets.js';

export function isoToScreen(col, row, offsetX, offsetY) {
  return {
    x: offsetX + (col - row) * HALF_W,
    y: offsetY + (col + row) * HALF_H,
  };
}

export function screenToIso(screenX, screenY, offsetX, offsetY) {
  const dx = (screenX - offsetX) / HALF_W;
  const dy = (screenY - offsetY) / HALF_H;
  return {
    col: Math.floor((dy + dx) / 2),
    row: Math.floor((dy - dx) / 2),
  };
}

function drawTile(ctx, col, row, offsetX, offsetY, debugGrid) {
  const { x, y } = isoToScreen(col, row, offsetX, offsetY);
  const tileType  = getTile(row, col);

  const sprite = getTileSprite(tileType);
  if (sprite) {
    ctx.drawImage(sprite, x - HALF_W, y, TILE_W, TILE_W);
  }

  if (debugGrid) {
    ctx.beginPath();
    ctx.moveTo(x,          y);
    ctx.lineTo(x + HALF_W, y + HALF_H);
    ctx.lineTo(x,          y + TILE_H);
    ctx.lineTo(x - HALF_W, y + HALF_H);
    ctx.closePath();
    ctx.strokeStyle = 'rgba(255, 0, 0, 0.7)';
    ctx.lineWidth   = 1;
    ctx.stroke();
  }
}

export function renderMap(ctx, canvasWidth, canvasHeight, debugGrid = false) {
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);

  const offsetX = canvasWidth  / 2;
  const offsetY = canvasHeight / 4;

  // Painter's algorithm: back to front (row-major)
  const rows = getMapRows();
  const cols = getMapCols();
  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      drawTile(ctx, col, row, offsetX, offsetY, debugGrid);
    }
  }
}
