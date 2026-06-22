import { isoToScreen } from './renderer.js';
import { TILE_H } from './map.js';

const texts = [];

export function addFloatingText(col, row, color, text) {
  texts.push({
    col,
    row,
    color,
    text,
    start: performance.now(),
    duration: 1500
  });
}

export function drawEffects(ctx, offsetX, offsetY, now) {
  for (let i = texts.length - 1; i >= 0; i--) {
    const t = texts[i];
    const elapsed = now - t.start;
    if (elapsed > t.duration) {
      texts.splice(i, 1);
      continue;
    }

    const progress = elapsed / t.duration;
    const { x, y } = isoToScreen(t.col, t.row, offsetX, offsetY);
    
    // Float upwards
    const drawY = y + TILE_H - 16 - (progress * 30);
    
    ctx.save();
    ctx.font = 'bold 16px Arial';
    ctx.textAlign = 'center';
    
    // Map TFS colors to CSS (roughly)
    let fillStyle = 'white';
    if (t.color === 180) fillStyle = '#e74c3c'; // red
    else if (t.color === 30) fillStyle = '#2ecc71'; // green
    
    // Fade out at the end
    if (progress > 0.7) {
      ctx.globalAlpha = 1 - ((progress - 0.7) / 0.3);
    }

    ctx.fillStyle = 'black';
    ctx.fillText(t.text, x + 1, drawY + 1); // shadow
    ctx.fillStyle = fillStyle;
    ctx.fillText(t.text, x, drawY);

    ctx.restore();
  }
}
