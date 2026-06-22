// Client → Server
export const C_LOGIN       = 0x01;
export const C_ENTER_GAME  = 0x0F;
export const C_LOGOUT      = 0x14;
export const C_PING        = 0x1D;
export const C_WALK_NORTH  = 0x65;
export const C_WALK_EAST   = 0x66;
export const C_WALK_SOUTH  = 0x67;
export const C_WALK_WEST   = 0x68;
export const C_TURN_NORTH  = 0x6F;
export const C_TURN_EAST   = 0x70;
export const C_TURN_SOUTH  = 0x71;
export const C_TURN_WEST   = 0x72;
export const C_ATTACK             = 0xA1;
export const C_CHANGE_FIGHT_MODES = 0xA0;
export const C_TALK               = 0x96;

// Server → Client
export const S_LOGIN_ERROR    = 0x0A;
export const S_ENTER_GAME     = 0x17;
export const S_PING_BACK      = 0x1D;
export const S_PING           = 0x1E;
// 0x64: S_CHAR_LIST in login phase / S_FULL_MAP in game phase (TFS dual-port convention)
export const S_CHAR_LIST      = 0x64;
export const S_FULL_MAP       = 0x64;
export const S_MAP_TOP_ROW    = 0x65;
export const S_MAP_RIGHT_COL  = 0x66;
export const S_MAP_BOTTOM_ROW = 0x67;
export const S_MAP_LEFT_COL   = 0x68;
export const S_CREATE_ON_MAP  = 0x6A;
export const S_DELETE_ON_MAP  = 0x6C;
export const S_MOVE_CREATURE  = 0x6D;
export const S_PLAYER_DATA    = 0xA0;
export const S_TEXT_MESSAGE   = 0xB4;
export const S_TALK             = 0xAA;
export const S_CREATURE_HEALTH  = 0x8C;
export const S_GRAPHICAL_EFFECT = 0x83;
export const S_TEXT_EFFECT      = 0x84;
export const S_MISSILE_EFFECT   = 0x85;
export const S_DEATH            = 0x28;
