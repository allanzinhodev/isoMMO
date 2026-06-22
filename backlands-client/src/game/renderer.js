import { TILE_W, TILE_H, MAP_DATA, MAP_COLS, MAP_ROWS } from './map.js';

const HALF_W = TILE_W / 2; // 32
const HALF_H = TILE_H / 2; // 16

// Sprite strip: 4 tiles × 32px wide, 32px tall — order matches TILE enum (0=GRASS,1=DIRT,2=WATER,3=SAND)
const tilesImage = new Image();
let tilesLoaded = false;
tilesImage.onload = () => { tilesLoaded = true; };
tilesImage.src = 'src/assets/tiles.png';

/**
 * Converts logical tile (col, row) to screen pixel position (top vertex of tile diamond).
 */
export function isoToScreen(col, row, offsetX, offsetY) {
  return {
    x: offsetX + (col - row) * HALF_W,
    y: offsetY + (col + row) * HALF_H,
  };
}

function drawTile(ctx, col, row, offsetX, offsetY, debugGrid) {
  const { x, y } = isoToScreen(col, row, offsetX, offsetY);
  const tileType = MAP_DATA[row][col];

  if (tilesLoaded) {
    // Each tile sprite is 32×32; anchor: top vertex of diamond at (x, y)
    ctx.drawImage(tilesImage, tileType * TILE_W, 0, TILE_W, TILE_W, x - HALF_W, y, TILE_W, TILE_W);
  }

  if (debugGrid) {
    ctx.beginPath();
    ctx.moveTo(x,          y);
    ctx.lineTo(x + HALF_W, y + HALF_H);
    ctx.lineTo(x,          y + TILE_H);
    ctx.lineTo(x - HALF_W, y + HALF_H);
    ctx.closePath();
    ctx.strokeStyle = 'rgba(255, 0, 0, 0.7)';
    ctx.lineWidth = 1;
    ctx.stroke();
  }
}

export function renderMap(ctx, canvasWidth, canvasHeight, debugGrid = false) {
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);

  const offsetX = canvasWidth / 2;
  const offsetY = canvasHeight / 4;

  // Painter's algorithm: back to front
  for (let row = 0; row < MAP_ROWS; row++) {
    for (let col = 0; col < MAP_COLS; col++) {
      drawTile(ctx, col, row, offsetX, offsetY, debugGrid);
    }
  }
}
