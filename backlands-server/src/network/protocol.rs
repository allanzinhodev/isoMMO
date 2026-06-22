use crate::utils::byte_buffer::{ByteBuffer, parse_packet};
use crate::db::Player;
use crate::game::map::WorldMap;
use super::opcodes::*;

// ── Incoming ────────────────────────────────────────────────────────────────

pub enum ClientMsg {
    Login      { username: String, password: String },
    EnterGame  { player_id: u32 },
    WalkNorth,
    WalkEast,
    WalkSouth,
    WalkWest,
    TurnNorth,
    TurnEast,
    TurnSouth,
    TurnWest,
    Ping,
    Logout,
    Attack { target_id: u32 },
    ChangeFightModes { mode: u8, chase: u8 },
    Unknown(u8),
}

pub fn decode(data: &[u8]) -> Option<ClientMsg> {
    let (opcode, mut p) = parse_packet(data)?;
    let msg = match opcode {
        C_LOGIN => ClientMsg::Login {
            username: p.read_string()?,
            password: p.read_string()?,
        },
        C_ENTER_GAME => ClientMsg::EnterGame { player_id: p.read_u32()? },
        C_WALK_NORTH => ClientMsg::WalkNorth,
        C_WALK_EAST  => ClientMsg::WalkEast,
        C_WALK_SOUTH => ClientMsg::WalkSouth,
        C_WALK_WEST  => ClientMsg::WalkWest,
        C_TURN_NORTH => ClientMsg::TurnNorth,
        C_TURN_EAST  => ClientMsg::TurnEast,
        C_TURN_SOUTH => ClientMsg::TurnSouth,
        C_TURN_WEST  => ClientMsg::TurnWest,
        C_PING       => ClientMsg::Ping,
        C_LOGOUT     => ClientMsg::Logout,
        C_ATTACK     => ClientMsg::Attack { target_id: p.read_u32()? },
        C_CHANGE_FIGHT_MODES => ClientMsg::ChangeFightModes { mode: p.read_u8()?, chase: p.read_u8()? },
        op           => ClientMsg::Unknown(op),
    };
    Some(msg)
}

// ── Outgoing ─────────────────────────────────────────────────────────────────

pub fn login_error(reason: &str) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_string(reason);
    b.into_packet(S_LOGIN_ERROR)
}

/// [u8 count] then per char: [u32 id][str name][u8 vocation][u8 looktype]
pub fn char_list(players: &[Player]) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u8(players.len() as u8);
    for p in players {
        b.write_u32(p.id);
        b.write_string(&p.name);
        b.write_u8(p.vocation as u8);
        b.write_u8(p.looktype as u8);
    }
    b.into_packet(S_CHAR_LIST)
}

/// [u32 id][str name][u8 vocation][u8 looktype][u16 pos_x][u16 pos_y]
pub fn enter_game(p: &Player) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u32(p.id);
    b.write_string(&p.name);
    b.write_u8(p.vocation as u8);
    b.write_u8(p.looktype as u8);
    b.write_u16(p.pos_x as u16);
    b.write_u16(p.pos_y as u16);
    b.into_packet(S_ENTER_GAME)
}

/// [u16 pos_x][u16 pos_y][u8 direction]  — movement confirmation
pub fn player_data(pos_x: i16, pos_y: i16, direction: u8) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u16(pos_x as u16);
    b.write_u16(pos_y as u16);
    b.write_u8(direction);
    b.into_packet(S_PLAYER_DATA)
}

/// [u8 type][str message]
pub fn text_message(msg_type: u8, message: &str) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u8(msg_type);
    b.write_string(message);
    b.into_packet(S_TEXT_MESSAGE)
}

pub fn ping_back() -> Vec<u8> {
    ByteBuffer::new().into_packet(S_PING_BACK)
}

/// Full map snapshot — [u8 cols][u8 rows][u8 ground * (cols*rows)], row-major.
/// Mirrors TFS ProtocolGame::sendMapDescription() / GetMapDescription().
/// Opcode 0x64 = S_FULL_MAP (game phase); same hex as S_CHAR_LIST (login phase).
pub fn full_map(world: &WorldMap) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u8(world.cols() as u8);
    b.write_u8(world.rows() as u8);
    for row in 0..world.rows() {
        for col in 0..world.cols() {
            b.write_u8(world.ground_at(col, row));
        }
    }
    b.into_packet(S_FULL_MAP)
}

/// Spawn a creature on all nearby clients.
/// Mirrors TFS ProtocolGame::sendAddCreature().
/// [u32 creature_id][u16 pos_x][u16 pos_y][u8 direction][str name][u8 looktype]
pub fn create_on_map(creature_id: u32, pos_x: i16, pos_y: i16, direction: u8, name: &str, looktype: u8) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u32(creature_id);
    b.write_u16(pos_x as u16);
    b.write_u16(pos_y as u16);
    b.write_u8(direction);
    b.write_string(name);
    b.write_u8(looktype);
    b.into_packet(S_CREATE_ON_MAP)
}

/// Remove a creature from all nearby clients.
/// Mirrors TFS ProtocolGame::sendRemoveCreature().
/// [u32 creature_id]
pub fn delete_on_map(creature_id: u32) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u32(creature_id);
    b.into_packet(S_DELETE_ON_MAP)
}

/// Broadcast a creature movement to nearby clients.
/// Mirrors TFS ProtocolGame::sendMoveCreature().
/// [u32 creature_id][u16 new_x][u16 new_y][u8 direction]
pub fn move_creature(creature_id: u32, pos_x: i16, pos_y: i16, direction: u8) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u32(creature_id);
    b.write_u16(pos_x as u16);
    b.write_u16(pos_y as u16);
    b.write_u8(direction);
    b.into_packet(S_MOVE_CREATURE)
}

// ── Combat ───────────────────────────────────────────────────────────────────

/// [u32 creature_id][u8 hp_percent]
pub fn creature_health(creature_id: u32, hp_percent: u8) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u32(creature_id);
    b.write_u8(hp_percent);
    b.into_packet(S_CREATURE_HEALTH)
}

/// [u16 pos_x][u16 pos_y][u8 effect_id]
pub fn graphical_effect(pos_x: i16, pos_y: i16, effect_id: u8) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u16(pos_x as u16);
    b.write_u16(pos_y as u16);
    b.write_u8(effect_id);
    b.into_packet(S_GRAPHICAL_EFFECT)
}

/// [u16 pos_x][u16 pos_y][u8 color][str text]
pub fn text_effect(pos_x: i16, pos_y: i16, color: u8, text: &str) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u16(pos_x as u16);
    b.write_u16(pos_y as u16);
    b.write_u8(color);
    b.write_string(text);
    b.into_packet(S_TEXT_EFFECT)
}

/// [u16 from_x][u16 from_y][u16 to_x][u16 to_y][u8 missile_id]
pub fn missile_effect(from_x: i16, from_y: i16, to_x: i16, to_y: i16, missile_id: u8) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u16(from_x as u16);
    b.write_u16(from_y as u16);
    b.write_u16(to_x as u16);
    b.write_u16(to_y as u16);
    b.write_u8(missile_id);
    b.into_packet(S_MISSILE_EFFECT)
}

/// [u32 creature_id]
pub fn death(creature_id: u32) -> Vec<u8> {
    let mut b = ByteBuffer::new();
    b.write_u32(creature_id);
    b.into_packet(S_DEATH)
}
