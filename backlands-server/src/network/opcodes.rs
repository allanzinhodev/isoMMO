// Client → Server opcodes (TFS-aligned)
pub const C_LOGIN:      u8 = 0x01;
pub const C_ENTER_GAME: u8 = 0x0F;
pub const C_LOGOUT:     u8 = 0x14;
pub const C_PING:       u8 = 0x1D;
pub const C_WALK_NORTH: u8 = 0x65;
pub const C_WALK_EAST:  u8 = 0x66;
pub const C_WALK_SOUTH: u8 = 0x67;
pub const C_WALK_WEST:  u8 = 0x68;
pub const C_TURN_NORTH: u8 = 0x6F;
pub const C_TURN_EAST:  u8 = 0x70;
pub const C_TURN_SOUTH: u8 = 0x71;
pub const C_TURN_WEST:  u8 = 0x72;

// Server → Client opcodes (TFS-aligned)
pub const S_LOGIN_ERROR:    u8 = 0x0A;
pub const S_ENTER_GAME:     u8 = 0x17;
pub const S_PING_BACK:      u8 = 0x1D;
pub const S_PING:           u8 = 0x1E;
// 0x64: S_CHAR_LIST in login phase / S_FULL_MAP in game phase (same opcode, different ports in TFS)
pub const S_CHAR_LIST:      u8 = 0x64;
pub const S_FULL_MAP:       u8 = 0x64;
pub const S_MAP_TOP_ROW:    u8 = 0x65; // player walked north  — send new top row
pub const S_MAP_RIGHT_COL:  u8 = 0x66; // player walked east   — send new right col
pub const S_MAP_BOTTOM_ROW: u8 = 0x67; // player walked south  — send new bottom row
pub const S_MAP_LEFT_COL:   u8 = 0x68; // player walked west   — send new left col
pub const S_CREATE_ON_MAP:  u8 = 0x6A; // TFS: GameServerCreateOnMap — spawn creature
pub const S_DELETE_ON_MAP:  u8 = 0x6C; // TFS: GameServerDeleteOnMap — despawn creature
pub const S_MOVE_CREATURE:  u8 = 0x6D; // TFS: GameServerMoveCreature — creature moved
pub const S_PLAYER_DATA:    u8 = 0xA0;
pub const S_TEXT_MESSAGE:   u8 = 0xB4;
