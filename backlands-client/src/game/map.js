export const TILE_W = 32;
export const TILE_H = 16;

export const TILE = {
  GRASS: 0,
  DIRT:  1,
  WATER: 2,
  SAND:  3,
};

// ── Dynamic map state (populated by S_FULL_MAP from server) ─────────────────

const _state = {
  data: [
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
  ],
  cols: 10,
  rows: 10,
};

/** Called by S_FULL_MAP handler with a 2D row-major array from the server. */
export function setMapData(rows2d) {
  _state.data = rows2d;
  _state.rows = rows2d.length;
  _state.cols = rows2d[0]?.length ?? 0;
}

export function getTile(row, col) {
  return _state.data[row]?.[col] ?? 0;
}

export function getMapCols() { return _state.cols; }
export function getMapRows() { return _state.rows; }

// Legacy constants — kept so character.js import still works without changes
export const MAP_COLS = 10;
export const MAP_ROWS = 10;
