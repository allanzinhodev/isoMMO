pub mod opcodes;
pub mod protocol;

use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use sqlx::MySqlPool;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use tungstenite::Message;

use crate::db;
use crate::game::map::WorldMap;
use crate::game::state::{GameWorld, PlayerState};
use protocol::{ClientMsg, decode};

pub async fn run(pool: MySqlPool, addr: &str, world_map: Arc<WorldMap>, game: Arc<RwLock<GameWorld>>) {
    let listener = TcpListener::bind(addr).await.expect("failed to bind");
    println!("[server] listening on ws://{addr}");

    loop {
        let (stream, peer) = listener.accept().await.expect("accept failed");
        println!("[server] connection from {peer}");
        let pool      = pool.clone();
        let world_map = world_map.clone();
        let game      = game.clone();
        tokio::spawn(handle_connection(stream, pool, world_map, game));
    }
}

async fn handle_connection(
    stream:    TcpStream,
    pool:      MySqlPool,
    world_map: Arc<WorldMap>,
    game:      Arc<RwLock<GameWorld>>,
) {
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => { eprintln!("[ws] handshake error: {e}"); return; }
    };
    let (ws_tx, mut ws_rx) = ws.split();

    // All outgoing bytes flow through this channel so the forward task owns ws_tx.
    let (btx, brx) = mpsc::unbounded_channel::<Vec<u8>>();

    // Forward task: mpsc receiver → WebSocket writer
    let fwd = tokio::spawn(forward_task(ws_tx, brx));

    // Per-connection session state
    let mut account_id:  Option<u32> = None;
    let mut player_id:   Option<u32> = None;
    let mut creature_id: Option<u32> = None;
    let mut pos_x:     i16 = 5;
    let mut pos_y:     i16 = 5;
    let mut direction:  u8 = 1;
    let mut looktype:   u8 = 0;
    let mut my_name = String::new();

    while let Some(msg) = ws_rx.next().await {
        let data = match msg {
            Ok(Message::Binary(b)) => b,
            Ok(Message::Close(_))  => break,
            _                      => continue,
        };

        let client_msg = match decode(&data) {
            Some(m) => m,
            None    => continue,
        };

        match client_msg {
            // ── Login ────────────────────────────────────────────────────────
            ClientMsg::Login { username, password } => {
                match db::verify_account(&pool, &username, &password).await {
                    Some(aid) => {
                        account_id = Some(aid);
                        let players = db::get_players(&pool, aid).await;
                        let _ = btx.send(protocol::char_list(&players));
                    }
                    None => { let _ = btx.send(protocol::login_error("Usuario ou senha invalidos.")); }
                }
            }

            // ── Enter game ───────────────────────────────────────────────────
            // TFS sequence: LoginSuccess(0x17) → FullMap(0x64) → AddCreature(0x6A)*
            ClientMsg::EnterGame { player_id: pid } => {
                let Some(aid) = account_id else { continue; };
                let Some(p) = db::get_player(&pool, pid, aid).await else { continue; };

                player_id = Some(p.id);
                pos_x     = p.pos_x;
                pos_y     = p.pos_y;
                looktype  = p.looktype as u8;
                my_name   = p.name.clone();

                // Send game state to this client
                let _ = btx.send(protocol::enter_game(&p));
                let _ = btx.send(protocol::full_map(&world_map));

                // Register in world and exchange creature lists
                let cid = {
                    let mut g = game.write().await;
                    let cid = g.next_creature_id();

                    // Notify existing players of our arrival
                    let spawn_pkt = protocol::create_on_map(cid, pos_x, pos_y, direction, &my_name, looktype);
                    g.broadcast_all(spawn_pkt);

                    // Send us the list of players already in-world
                    for state in g.players.values() {
                        let pkt = protocol::create_on_map(
                            state.creature_id, state.pos_x, state.pos_y,
                            state.direction, &state.name, state.looktype,
                        );
                        let _ = btx.send(pkt);
                    }

                    g.add(PlayerState::new(
                        cid, p.id, my_name.clone(), looktype,
                        pos_x, pos_y, direction,
                        btx.clone(),
                    ));
                    cid
                };
                creature_id = Some(cid);
            }

            // ── Movement ─────────────────────────────────────────────────────
            // Validate against WorldMap, update GameWorld, broadcast to others.
            ClientMsg::WalkNorth => {
                let Some(cid) = creature_id else { continue; };
                direction = 1;
                let ny = pos_y - 1;
                if ny >= 0 && world_map.is_walkable(pos_x as usize, ny as usize) { pos_y = ny; }
                commit_move(&game, cid, &btx, pos_x, pos_y, direction).await;
            }
            ClientMsg::WalkEast => {
                let Some(cid) = creature_id else { continue; };
                direction = 3;
                let nx = pos_x + 1;
                if world_map.is_walkable(nx as usize, pos_y as usize) { pos_x = nx; }
                commit_move(&game, cid, &btx, pos_x, pos_y, direction).await;
            }
            ClientMsg::WalkSouth => {
                let Some(cid) = creature_id else { continue; };
                direction = 7;
                let ny = pos_y + 1;
                if world_map.is_walkable(pos_x as usize, ny as usize) { pos_y = ny; }
                commit_move(&game, cid, &btx, pos_x, pos_y, direction).await;
            }
            ClientMsg::WalkWest => {
                let Some(cid) = creature_id else { continue; };
                direction = 9;
                let nx = pos_x - 1;
                if nx >= 0 && world_map.is_walkable(nx as usize, pos_y as usize) { pos_x = nx; }
                commit_move(&game, cid, &btx, pos_x, pos_y, direction).await;
            }

            ClientMsg::TurnNorth => { direction = 1; }
            ClientMsg::TurnEast  => { direction = 3; }
            ClientMsg::TurnSouth => { direction = 7; }
            ClientMsg::TurnWest  => { direction = 9; }

            ClientMsg::Ping => { let _ = btx.send(protocol::ping_back()); }

            ClientMsg::Logout => {
                account_id = None;
                player_id  = None;
            }

            ClientMsg::Unknown(op) => eprintln!("[server] unknown opcode: 0x{op:02X}"),
        }
    }

    // ── Disconnect cleanup ────────────────────────────────────────────────────
    if let Some(cid) = creature_id {
        let mut g = game.write().await;
        g.remove(cid);
        let del_pkt = protocol::delete_on_map(cid);
        g.broadcast_all(del_pkt);
    }

    if let (Some(pid), Some(_)) = (player_id, account_id) {
        db::save_player_pos(&pool, pid, pos_x, pos_y).await;
    }

    fwd.abort();
}

/// Forward task: reads from the per-connection mpsc channel and writes to WebSocket.
async fn forward_task(
    mut ws_tx: futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>,
    mut brx:   mpsc::UnboundedReceiver<Vec<u8>>,
) {
    while let Some(pkt) = brx.recv().await {
        if ws_tx.send(Message::Binary(pkt.into())).await.is_err() { break; }
    }
}

/// Update position in GameWorld, send S_PLAYER_DATA to mover, broadcast S_MOVE_CREATURE to others.
async fn commit_move(
    game:      &Arc<RwLock<GameWorld>>,
    cid:       u32,
    btx:       &mpsc::UnboundedSender<Vec<u8>>,
    pos_x:     i16,
    pos_y:     i16,
    direction: u8,
) {
    let move_pkt = protocol::move_creature(cid, pos_x, pos_y, direction);

    let mut g = game.write().await;
    if let Some(state) = g.players.get_mut(&cid) {
        state.pos_x      = pos_x;
        state.pos_y      = pos_y;
        state.direction  = direction;
    }
    g.broadcast_except(cid, move_pkt);
    drop(g); // release write lock before sending

    let _ = btx.send(protocol::player_data(pos_x, pos_y, direction));
}
