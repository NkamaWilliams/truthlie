use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

use crate::{config, error::AppError, models::game::{CreateGameSessionPayload, GameSession}, AppStateData};

// When calling routes must include trailing / e.g. 'URL/games/'

#[post("/")]
pub async fn create_game(
    config: web::Json<CreateGameSessionPayload>,
    data: AppStateData
) -> Result<impl Responder, AppError>{
    let id = Uuid::new_v4();
    let host = config.host_id;

    // Verify that if with_staking is true then a stake_amount > 0 must be provided
    if let Some(staking) = config.with_staking {
        if staking && config.stake_amount.unwrap_or_else(|| 0) == 0 {
            return Err(AppError::Validation("Must provide a staking amount greater than 0 when staking is enabled!".into()));
        }
    }

    let session = GameSession {
        id,
        name: config.name.clone().unwrap_or_else(|| "Truth or Lie".into()),
        round_statements: vec![],
        players: vec![host],
        host_id: host,
        has_started: false,
        is_private: config.is_private.unwrap_or_else(|| false),
        with_staking: config.with_staking.unwrap_or_else(|| false),
        stake_amount: config.stake_amount.unwrap_or_else(|| 0),
        max_players: config.max_players.unwrap_or_else(|| config::MIN_PLAYERS),
        current_round: 0,
        max_rounds: config.max_rounds.unwrap_or_else(|| config::MIN_ROUNDS),
        current_guesses: 0,
        round_duration: 0,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    data.sessions.insert(id, session.clone());
    Ok(HttpResponse::Created().json(session))
}

#[get("/")]
pub async fn get_games(
    data: AppStateData
) -> Result<impl Responder, AppError> {
    let games = data.sessions
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(games))
}