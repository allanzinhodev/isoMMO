pub mod protocol;

use sqlx::MySqlPool;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use tungstenite::Message;

use crate::db;
use crate::game::world::{MAP_COLS, MAP_ROWS};
use protocol::{ClientMsg, ServerMsg, CharEntry, GamePlayer};

pub async fn run(pool: MySqlPool, addr: &str) {
    let listener = TcpListener::bind(addr).await.expect("failed to bind");
    println!("[server] listening on ws://{addr}");

    loop {
        let (stream, peer) = listener.accept().await.expect("accept failed");
        println!("[server] connection from {peer}");
        let pool = pool.clone();
        tokio::spawn(handle_connection(stream, pool));
    }
}

async fn handle_connection(stream: TcpStream, pool: MySqlPool) {
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => { eprintln!("[ws] handshake error: {e}"); return; }
    };

    let (mut tx, mut rx) = ws.split();

    let mut account_id: Option<u32> = None;
    let mut player_id:  Option<u32> = None;
    let mut pos_x: i16 = 5;
    let mut pos_y: i16 = 5;

    while let Some(msg) = rx.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(_) => break,
        };

        let text = match msg {
            Message::Text(t) => t,
            Message::Close(_) => break,
            _ => continue,
        };

        let client_msg: ClientMsg = match serde_json::from_str(&text) {
            Ok(m) => m,
            Err(e) => {
                let _ = send(&mut tx, &ServerMsg::Error { reason: format!("bad message: {e}") }).await;
                continue;
            }
        };

        let response = handle_msg(
            client_msg, &pool,
            &mut account_id, &mut player_id,
            &mut pos_x, &mut pos_y,
        ).await;

        if send(&mut tx, &response).await.is_err() { break; }
    }

    if let (Some(pid), Some(_)) = (player_id, account_id) {
        db::save_player_pos(&pool, pid, pos_x, pos_y).await;
    }
}

async fn handle_msg(
    msg: ClientMsg,
    pool: &MySqlPool,
    account_id: &mut Option<u32>,
    player_id:  &mut Option<u32>,
    pos_x: &mut i16,
    pos_y: &mut i16,
) -> ServerMsg {
    match msg {
        ClientMsg::Login { username, password } => {
            match db::verify_account(pool, &username, &password).await {
                Some(aid) => {
                    *account_id = Some(aid);
                    let players = db::get_players(pool, aid).await;
                    let entries = players.into_iter().map(|p| CharEntry {
                        id:       p.id,
                        name:     p.name,
                        vocation: p.vocation as i32,
                        looktype: p.looktype as i32,
                    }).collect();
                    ServerMsg::CharList { players: entries }
                }
                None => ServerMsg::LoginFail { reason: "Invalid username or password.".into() },
            }
        }

        ClientMsg::SelectChar { player_id: pid } => {
            let aid = match account_id {
                Some(a) => *a,
                None => return ServerMsg::Error { reason: "not logged in".into() },
            };
            match db::get_player(pool, pid, aid).await {
                Some(p) => {
                    *player_id = Some(p.id);
                    *pos_x = p.pos_x;
                    *pos_y = p.pos_y;
                    ServerMsg::EnterGame { player: GamePlayer {
                        id:       p.id,
                        name:     p.name,
                        vocation: p.vocation as i32,
                        looktype: p.looktype as i32,
                        pos_x:    p.pos_x as i32,
                        pos_y:    p.pos_y as i32,
                    }}
                }
                None => ServerMsg::Error { reason: "player not found".into() },
            }
        }

        ClientMsg::Move { direction } => {
            if player_id.is_none() {
                return ServerMsg::Error { reason: "not in game".into() };
            }
            let (dx, dy): (i16, i16) = match direction {
                1 => ( 0, -1), // NE
                3 => ( 1,  0), // SE
                7 => ( 0,  1), // SW
                9 => (-1,  0), // NW
                _ => return ServerMsg::Error { reason: "invalid direction".into() },
            };
            *pos_x = (*pos_x + dx).clamp(0, MAP_COLS as i16 - 1);
            *pos_y = (*pos_y + dy).clamp(0, MAP_ROWS as i16 - 1);
            ServerMsg::PlayerMoved { pos_x: *pos_x as i32, pos_y: *pos_y as i32, direction }
        }

        ClientMsg::Logout => {
            *account_id = None;
            *player_id  = None;
            ServerMsg::Error { reason: "logged out".into() }
        }
    }
}

async fn send<S>(tx: &mut S, msg: &ServerMsg) -> Result<(), ()>
where
    S: SinkExt<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let text = serde_json::to_string(msg).unwrap();
    tx.send(Message::Text(text.into())).await.map_err(|_| ())
}
