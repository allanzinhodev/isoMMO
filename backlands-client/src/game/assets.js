// assets.js - Loads and parses custom .spr and .dat files
export const ASSETS = {
  sprites: [], // Array of ImageBitmaps or offscreen canvases
  tiles: [],   // ID -> { spriteIndex }
  chars: [],   // ID -> { baseSpriteIndex, frames }
};

let loaded = false;

export async function loadAssets() {
  if (loaded) return;

  const [sprRes, datRes] = await Promise.all([
    fetch('src/assets/backlands.spr'),
    fetch('src/assets/backlands.dat')
  ]);

  const sprBuf = await sprRes.arrayBuffer();
  const datBuf = await datRes.arrayBuffer();

  await parseSpr(sprBuf);
  parseDat(datBuf);
  
  loaded = true;
  console.log(`[Assets] Loaded ${ASSETS.sprites.length} sprites, ${ASSETS.tiles.length} tiles, ${ASSETS.chars.length} characters.`);
}

async function parseSpr(buffer) {
  const view = new DataView(buffer);
  const sig = view.getUint32(0, true);
  if (sig !== 0x4241434B) throw new Error("Invalid SPR signature");

  const count = view.getUint16(4, true);
  let offset = 6;

  for (let i = 0; i < count; i++) {
    const w = view.getUint16(offset, true); offset += 2;
    const h = view.getUint16(offset, true); offset += 2;
    
    const byteLength = w * h * 4;
    const pixels = new Uint8ClampedArray(buffer, offset, byteLength);
    offset += byteLength;

    const imageData = new ImageData(pixels, w, h);
    
    // Create offscreen canvas for fast rendering
    const canvas = document.createElement('canvas');
    canvas.width = w;
    canvas.height = h;
    const ctx = canvas.getContext('2d');
    ctx.putImageData(imageData, 0, 0);
    
    ASSETS.sprites.push(canvas);
  }
}

function parseDat(buffer) {
  const view = new DataView(buffer);
  const sig = view.getUint32(0, true);
  if (sig !== 0x44415441) throw new Error("Invalid DAT signature");

  const count = view.getUint16(4, true);
  let offset = 6;

  for (let i = 0; i < count; i++) {
    const type = view.getUint8(offset++);
    
    if (type === 0) {
      // Tile
      const spriteIndex = view.getUint16(offset, true); offset += 2;
      ASSETS.tiles.push({ spriteIndex });
    } else if (type === 1) {
      // Character
      const baseSpriteIndex = view.getUint16(offset, true); offset += 2;
      const frames = view.getUint8(offset++);
      ASSETS.chars.push({ baseSpriteIndex, frames });
    }
  }
}

export function isLoaded() {
  return loaded;
}

export function getTileSprite(id) {
  const t = ASSETS.tiles[id];
  if (!t) return null;
  return ASSETS.sprites[t.spriteIndex];
}

export function getCharSprite(looktype, frame) {
  const c = ASSETS.chars[looktype];
  if (!c) return null;
  // Frame wrap around just in case
  return ASSETS.sprites[c.baseSpriteIndex + (frame % c.frames)];
}
