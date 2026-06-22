/// GameWorld — shared multiplayer state.
///
/// Mirrors TFS's Game singleton + CreatureMap pattern:
///   Game::addCreature() / Game::removeCreature() / Game::getCreatureByID()
/// Here simplified to a HashMap with per-player MPSC senders for broadcast.

use std::collections::HashMap;
use tokio::sync::mpsc;

// ── Per-player state stored in the world ─────────────────────────────────────

pub struct PlayerState {
    pub creature_id: u32,
    pub player_id:   u32,
    pub name:        String,
    pub looktype:    u8,
    pub pos_x:       i16,
    pub pos_y:       i16,
    pub direction:   u8,
    pub hp:          u32,
    pub max_hp:      u32,
    sender: mpsc::UnboundedSender<Vec<u8>>,
}

impl PlayerState {
    pub fn new(
        creature_id: u32, player_id: u32,
        name: String, looktype: u8,
        pos_x: i16, pos_y: i16, direction: u8,
        sender: mpsc::UnboundedSender<Vec<u8>>,
    ) -> Self {
        Self { creature_id, player_id, name, looktype, pos_x, pos_y, direction, hp: 100, max_hp: 100, sender }
    }

    pub fn send(&self, pkt: Vec<u8>) {
        let _ = self.sender.send(pkt);
    }
}

// ── Shared game world ─────────────────────────────────────────────────────────

pub struct GameWorld {
    pub players: HashMap<u32, PlayerState>, // creature_id → state
    next_id:     u32,
}

impl GameWorld {
    pub fn new() -> Self {
        Self { players: HashMap::new(), next_id: 1 }
    }

    /// TFS: Game::addCreature — assign a unique creature ID and register.
    pub fn add(&mut self, state: PlayerState) {
        self.players.insert(state.creature_id, state);
    }

    pub fn next_creature_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// TFS: Game::removeCreature
    pub fn remove(&mut self, creature_id: u32) -> Option<PlayerState> {
        self.players.remove(&creature_id)
    }

    /// Send a packet to every connected player.
    pub fn broadcast_all(&self, pkt: Vec<u8>) {
        for state in self.players.values() {
            state.send(pkt.clone());
        }
    }

    /// Send a packet to every connected player except one (e.g. the mover).
    pub fn broadcast_except(&self, exclude: u32, pkt: Vec<u8>) {
        for (id, state) in &self.players {
            if *id != exclude {
                state.send(pkt.clone());
            }
        }
    }
}
