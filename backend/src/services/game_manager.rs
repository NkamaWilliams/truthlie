use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use crate::ws::{player_ws::PlayerWS, BroadcastMessage, ClientMessage, Connect, Disconnect};

/// Global Game Manager
pub struct GameManager {
    // Maps Game to (Player, PlayerActor) mapping
    games: HashMap<Uuid, HashMap<Uuid, Addr<PlayerWS>>>
}

impl GameManager {
    pub fn new() -> Self {
        Self { games: HashMap::new() }
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

impl Handler<ClientMessage> for GameManager {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(players) = self.games.get(&msg.game_id) {
            for (pid, addr) in players {
                if pid != &msg.player_id {
                    addr.do_send(BroadcastMessage(msg.text.clone()));
                }
            }
        }
    }
}