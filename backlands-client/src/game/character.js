import { isoToScreen } from './renderer.js';
import { TILE_H } from './map.js';

const SPRITE_W = 16;
const SPRITE_H = 32;
const SCALE    = 1;
const WALK_MS  = 300;
const MOVE_MS  = 150;

// Character types — each maps to a row in character.png
export const CHARACTER_TYPE = {
  BLACK_MAGE: 0,
  HUNTER:     1,
  BLUE_MAGE:  2,
};

const sheet = new Image();
let sheetLoaded = false;
sheet.onload = () => { sheetLoaded = true; };
sheet.src = 'src/assets/character.png';

const DIR_CONFIG = {
  1: { idle: 3, walk: [4, 5], flipH: true  }, // NE
  3: { idle: 0, walk: [1, 2], flipH: true  }, // SE
  7: { idle: 0, walk: [1, 2], flipH: false }, // SW
  9: { idle: 3, walk: [4, 5], flipH: false }, // NW
};

export class Character {
  constructor(col, row, type = CHARACTER_TYPE.BLACK_MAGE) {
    this.col  = col;
    this.row  = row;
    this.type = type;
    this.direction = 7;
    this.moving    = false;

    this._visualCol = col;
    this._visualRow = row;
    this._fromCol   = col;
    this._fromRow   = row;
    this._toCol     = col;
    this._toRow     = row;
    this._moveStart = null;
  }

  moveTo(col, row, direction) {
    this._fromCol   = this._visualCol;
    this._fromRow   = this._visualRow;
    this._toCol     = col;
    this._toRow     = row;
    this._moveStart = performance.now();
    this.col        = col;
    this.row        = row;
    this.direction  = direction;
    this.moving     = true;
  }

  update(now) {
    if (this._moveStart === null) return;

    const t    = Math.min(1, (now - this._moveStart) / MOVE_MS);
    const ease = t < 0.5 ? 4 * t ** 3 : 1 - (-2 * t + 2) ** 3 / 2;

    this._visualCol = this._fromCol + (this._toCol - this._fromCol) * ease;
    this._visualRow = this._fromRow + (this._toRow - this._fromRow) * ease;

    if (t >= 1) {
      this._visualCol = this._toCol;
      this._visualRow = this._toRow;
      this._moveStart = null;
      this.moving     = false;
    }
  }

  isMoving() {
    return this._moveStart !== null;
  }

  /** Snap immediately to server-authoritative position (no animation). */
  snapTo(col, row, direction) {
    this.col        = col;
    this.row        = row;
    this.direction  = direction;
    this._visualCol = col;
    this._visualRow = row;
    this._fromCol   = col;
    this._fromRow   = row;
    this._toCol     = col;
    this._toRow     = row;
    this._moveStart = null;
    this.moving     = false;
  }

  getFrame() {
    const cfg = DIR_CONFIG[this.direction] ?? DIR_CONFIG[1];
    if (!this.moving) return cfg.idle;
    return cfg.walk[Math.floor(performance.now() / WALK_MS) % 2];
  }

  draw(ctx, offsetX, offsetY) {
    if (!sheetLoaded) return;

    const { x, y } = isoToScreen(this._visualCol, this._visualRow, offsetX, offsetY);
    const frame = this.getFrame();
    const cfg   = DIR_CONFIG[this.direction] ?? DIR_CONFIG[1];

    const dw = SPRITE_W * SCALE;
    const dh = SPRITE_H * SCALE;

    const drawX  = Math.round(x - dw / 2);
    const drawY  = Math.round(y + TILE_H - dh - 4);
    const sheetY = this.type * SPRITE_H; // row in spritesheet

    ctx.save();
    ctx.imageSmoothingEnabled = false;

    if (cfg.flipH) {
      ctx.translate(drawX + dw, drawY);
      ctx.scale(-1, 1);
      ctx.drawImage(sheet, frame * SPRITE_W, sheetY, SPRITE_W, SPRITE_H, 0, 0, dw, dh);
    } else {
      ctx.drawImage(sheet, frame * SPRITE_W, sheetY, SPRITE_W, SPRITE_H, drawX, drawY, dw, dh);
    }

    ctx.restore();
  }
}
