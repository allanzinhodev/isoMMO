import { Character } from './character.js';

// creature_id → Character instance
const _creatures = new Map();

export function addCreature(id, col, row, dir, looktype) {
  const c = new Character(col, row, looktype);
  c.direction = dir;
  c.id = id;
  _creatures.set(id, c);
}

export function removeCreature(id) {
  _creatures.delete(id);
}

export function getCreature(id) {
  return _creatures.get(id);
}

export function moveCreature(id, newCol, newRow, dir) {
  const c = _creatures.get(id);
  if (c) c.moveTo(newCol, newRow, dir);
}

export function updateAll(now) {
  _creatures.forEach(c => c.update(now));
}

export function getCreatures() {
  return Array.from(_creatures.values());
}

export function clear() {
  _creatures.clear();
}
