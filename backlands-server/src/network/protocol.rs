use serde::{Deserialize, Serialize};

// Client → Server
#[derive(Debug, Deserialize)]
#[serde(tag = "op", content = "data", rename_all = "snake_case")]
pub enum ClientMsg {
    Login     { username: String, password: String },
    SelectChar{ player_id: u32 },
    Move      { direction: u8 },
    Logout,
}

// Server → Client
#[derive(Debug, Serialize)]
#[serde(tag = "op", content = "data", rename_all = "snake_case")]
pub enum ServerMsg {
    LoginFail { reason: String },
    CharList  { players: Vec<CharEntry> },
    EnterGame { player: GamePlayer },
    PlayerMoved { pos_x: i32, pos_y: i32, direction: u8 },
    Error     { reason: String },
}

#[derive(Debug, Serialize)]
pub struct CharEntry {
    pub id:       u32,
    pub name:     String,
    pub vocation: i32,
    pub looktype: i32,
}

#[derive(Debug, Serialize)]
pub struct GamePlayer {
    pub id:       u32,
    pub name:     String,
    pub vocation: i32,
    pub looktype: i32,
    pub pos_x:    i32,
    pub pos_y:    i32,
}
