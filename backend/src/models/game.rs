use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameSession {
    pub id: Uuid,
    pub name: String,
    pub round_statements: Vec<Statements>,
    pub players: Vec<Uuid>,
    pub host_id: Uuid,
    pub has_started: bool,
    pub is_private: bool,
    pub with_staking: bool,
    pub stake_amount: u32,
    pub max_players: u32,
    pub current_round: u32,
    pub max_rounds: u32,
    pub current_guesses: u32,
    pub round_duration: u32, // in seconds
    pub created_at: String, // ISO 8601 format
    pub updated_at: String, // ISO 8601 format
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Statements {
    pub id: Uuid,
    pub game_id: Uuid,
    pub creator: Uuid,
    pub true_statement: String,
    pub false_statement: String,
    pub right_guesses: Vec<Uuid>,
    pub wrong_guesses: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameSessionPayload {
    pub name: Option<String>,
    pub host_id: Uuid,
    pub is_private: Option<bool>,
    pub with_staking: Option<bool>,
    pub max_rounds: Option<u32>,
    pub max_players: Option<u32>,
    pub stake_amount: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct JoinGameSessionPayload {
    pub player_id: Uuid
}