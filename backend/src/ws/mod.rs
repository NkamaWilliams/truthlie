use actix::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::ws::player_ws::PlayerWS;

pub mod player_ws;

/// Sent when a player wants to connect to a game
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub addr: Addr<PlayerWS>,
}

/// Sent when a player disconnects
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub player_id: Uuid,
    pub game_id: Uuid,
}

/// A message from the client (over WS) → server
#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize)]
pub struct ClientMessage {
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub text: String, // later we’ll parse JSON → enums
}

/// Message from manager to broadcast (server → client)
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);