use actix::prelude::*;
use serde::{Deserialize, Serialize};
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

/// A message from the client to start a game
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct StartGame {
    pub player_id: Uuid,
    pub game_id: Uuid,
}

/// Message from manager to broadcast (server → client)
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub ServerMessage);

/// A message from the client (over WS) → server
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    StartGame {game_id: String, player_id: String},
}

/// A message from the server (over WS) → client
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    StartedGame {game_id: String},
    Error {message: String}
}