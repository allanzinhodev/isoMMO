export const TILE_W = 32;
export const TILE_H = 16;

export const TILE = {
  GRASS: 0,
  DIRT:  1,
  WATER: 2,
  SAND:  3,
};

// 10x10 hardcoded map — row-major [row][col]
export const MAP_DATA = [
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
  [0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
  [0, 1, 1, 3, 3, 1, 1, 0, 0, 0],
  [0, 0, 1, 3, 3, 1, 0, 0, 2, 2],
  [0, 0, 0, 1, 1, 0, 0, 2, 2, 2],
  [0, 0, 0, 0, 0, 0, 2, 2, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

export const MAP_COLS = MAP_DATA[0].length;
export const MAP_ROWS = MAP_DATA.length;
