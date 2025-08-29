use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use crate::{models::game::GameSession, ws::{player_ws::PlayerWS, BroadcastMessage, Connect, Disconnect, ServerMessage, StartGame}, AppStateData};

/// Global Game Manager
/// 
/// Actors process messages sequentially so using HashMap is safe
pub struct GameManager {
    // Maps Game to (Player, PlayerActor) mapping
    games: HashMap<Uuid, HashMap<Uuid, Addr<PlayerWS>>>,
    sessions: HashMap<Uuid, GameSession>, // temporary live game state (will settle once game ends)
    global_data: AppStateData // Reference to global state
}

impl GameManager {
    pub fn new(global_data: AppStateData) -> Self {
        Self { games: HashMap::new(), sessions: HashMap::new(), global_data }
    }
}

impl Actor for GameManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for GameManager {
    type Result = ();
    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let players = self.games.entry(msg.game_id).or_default();
        players.insert(msg.player_id, msg.addr);
        println!("Player {:?} joined game {:?}", msg.player_id, msg.game_id);
    }
}

impl Handler<Disconnect> for GameManager {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        let players = self.games.entry(msg.game_id).or_default();
        if players.contains_key(&msg.player_id) {
            players.remove(&msg.player_id);
            println!("Player {:?} left game {:?}", msg.player_id, msg.game_id);
        }
    }
}

impl Handler<StartGame> for GameManager {
    type Result = ();
    fn handle(&mut self, msg: StartGame, _ctx: &mut Self::Context) -> Self::Result {
        let mut none = false;
        
        let message = if let Some(session) = self.global_data.sessions.get(&msg.game_id) {
            if session.host_id != msg.player_id {
                none = true;
                "Only the host can start a game"
            } else {
                self.sessions.insert(msg.game_id, session.clone());
                ""
            }
        } else {
            none = true;
            "Failed to find game session!"

        };

        if let Some(players) = self.games.get(&msg.game_id) {
            for (pid, addr) in players {
                if pid != &msg.player_id {
                    let message = if none { 
                        ServerMessage::Error { message: message.into() } 
                    } else {
                        ServerMessage::StartedGame { game_id: msg.game_id.to_string() }
                    };
                    addr.do_send(BroadcastMessage(message));
                }
            }
        }
    }
}