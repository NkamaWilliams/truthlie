use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{config::{self, MIN_PLAYERS}, error::AppError, models::game::{CreateGameSessionPayload, GameSession, JoinGameSessionPayload}, AppStateData};

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
        if staking && config.stake_amount.unwrap_or(0) == 0 {
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
        is_private: config.is_private.unwrap_or(false),
        with_staking: config.with_staking.unwrap_or(false),
        stake_amount: config.stake_amount.unwrap_or(0),
        max_players: config.max_players.unwrap_or(config::MIN_PLAYERS),
        current_round: 0,
        max_rounds: config.max_rounds.unwrap_or(config::MIN_ROUNDS),
        current_guesses: 0,
        round_duration: 0,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    data.sessions.insert(id, session.clone());
    Ok(HttpResponse::Created().json(session))
}

#[post("/{game_id}/join")]
pub async fn join_game(
    path: web::Path<Uuid>,
    payload: web::Json<JoinGameSessionPayload>,
    data: AppStateData,
) -> Result<impl Responder, AppError> {
    let game_id = path.into_inner();
    let mut game = data.sessions.get_mut(&game_id)
        .ok_or_else(|| AppError::NotFound(format!("Game {} not found", game_id)))?;

    if game.max_players == game.players.len() as u32 {
        return Err(AppError::Validation("Game lobby full! Please join another game".into()));
    }
    else if game.has_started {
        return Err(AppError::Validation("Game already started! Please join another game".into()));
    }
    else if game.players.contains(&payload.player_id) {
        return Err(AppError::Validation("Player already in game".into()));
    }

    game.players.push(payload.player_id);

    Ok(HttpResponse::Ok().json(json!({"message": "Joined game successfully", "body": game.clone()})))
}

#[post("/{game_id}/start")]
pub async fn start_game(
    path: web::Path<Uuid>,
    payload: web::Json<JoinGameSessionPayload>,
    data: AppStateData,
) -> Result<impl Responder, AppError> {
    let game_id = path.into_inner();
    let mut game = data.sessions.get_mut(&game_id)
        .ok_or_else(|| AppError::NotFound(format!("Game {} not found", game_id)))?;

    if (game.players.len() as u32) < MIN_PLAYERS {
        return Err(AppError::Validation(format!(
            "Game needs at least {} players to start!",
            MIN_PLAYERS
        )));
    }

    if game.has_started {
        return Err(AppError::Validation("Game has already started!".into()));
    }

    if game.host_id != payload.player_id {
        return  Err(AppError::Unauthorized("Only host can start game!".into()));
    }
    game.has_started = true;
    game.updated_at = Utc::now().to_rfc3339();
    game.current_round = 1;

    Ok(HttpResponse::Ok().json(json!({"message": "Started game successfully", "body": game.clone()})))
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