use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub rank: u32,
    pub correct_guesses: u32,
    pub wrong_guesses: u32,
    pub no_guesses: u32,
    pub max_correct_guesses_in_game: u32,
    pub wallet_address: Option<String>,
    pub current_game_id: Option<Uuid>,
    pub games_played: u32,
}


#[derive(Debug, Deserialize)]
pub struct CreatePlayerPayload {
    pub name: String
}