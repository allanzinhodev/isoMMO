/// WorldMap — TFS-inspired flat-array tile map.
///
/// TFS uses a quadtree (QTreeNode) for large map storage; here we use a simple
/// Vec<Tile> with row-major indexing: tile(col, row) = tiles[row * cols + col].
/// The API mirrors TFS's Map::getTile() / Tile::hasFlag().

// ── Tile flags (mirrors TFS TILESTATE_* / FLAG_* masks) ────────────────────

#[derive(Clone, Copy)]
pub struct TileFlags(pub u8);

impl TileFlags {
    pub const NONE:      u8 = 0x00;
    pub const BLOCKING:  u8 = 0x01; // solid, cannot walk through (TFS: FLAG_BLOCKSOLID)

    pub fn is_walkable(self) -> bool {
        self.0 & Self::BLOCKING == 0
    }
}

// ── Tile ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Tile {
    pub ground: u8,      // ground sprite index (0=grass, 1=dirt, 2=water, 3=sand)
    pub flags:  TileFlags,
}

impl Tile {
    const fn new(ground: u8, flags: u8) -> Self {
        Self { ground, flags: TileFlags(flags) }
    }
}

// ── WorldMap ──────────────────────────────────────────────────────────────────

pub struct WorldMap {
    cols:  usize,
    rows:  usize,
    tiles: Vec<Tile>,
}

// Ground IDs
const G: u8 = 0; // GRASS   — walkable
const D: u8 = 1; // DIRT    — walkable
const W: u8 = 2; // WATER   — BLOCKING
const S: u8 = 3; // SAND    — walkable

// Static 10×10 layout — matches backlands-client/src/game/map.js MAP_DATA
#[rustfmt::skip]
const MAP_GROUNDS: &[u8] = &[
    G, G, G, G, G, G, G, G, G, G,
    G, G, G, D, D, G, G, G, G, G,
    G, G, D, D, D, D, G, G, G, G,
    G, D, D, S, S, D, D, G, G, G,
    G, G, D, S, S, D, G, G, W, W,
    G, G, G, D, D, G, G, W, W, W,
    G, G, G, G, G, G, W, W, G, G,
    G, G, G, G, G, G, G, G, G, G,
    G, G, G, G, G, G, G, G, G, G,
    G, G, G, G, G, G, G, G, G, G,
];

pub const WORLD_COLS: usize = 10;
pub const WORLD_ROWS: usize = 10;

impl WorldMap {
    pub fn new() -> Self {
        let tiles = MAP_GROUNDS
            .iter()
            .map(|&g| {
                let flags = if g == W { TileFlags::BLOCKING } else { TileFlags::NONE };
                Tile::new(g, flags)
            })
            .collect();

        Self { cols: WORLD_COLS, rows: WORLD_ROWS, tiles }
    }

    // TFS: Map::getTile(Position)
    pub fn get_tile(&self, col: usize, row: usize) -> Option<&Tile> {
        if col < self.cols && row < self.rows {
            Some(&self.tiles[row * self.cols + col])
        } else {
            None
        }
    }

    // TFS: Tile::hasFlag(FLAG_BLOCKSOLID)
    pub fn is_walkable(&self, col: usize, row: usize) -> bool {
        self.get_tile(col, row).map_or(false, |t| t.flags.is_walkable())
    }

    pub fn ground_at(&self, col: usize, row: usize) -> u8 {
        self.get_tile(col, row).map_or(0, |t| t.ground)
    }

    pub fn cols(&self) -> usize { self.cols }
    pub fn rows(&self) -> usize { self.rows }
}
