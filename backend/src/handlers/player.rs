use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{models::player::{CreatePlayerPayload, Player}, state::AppStateData};

// When calling routes must include trailing / e.g. 'URL/players/'

#[post("/")]
pub async fn create_player(
    payload: web::Json<CreatePlayerPayload>,
    data: AppStateData
) -> impl Responder {
    let id = Uuid::new_v4();
    let player = Player {
        id,
        name: payload.name.clone(),
        rank: 0,
        correct_guesses: 0,
        wrong_guesses: 0,
        no_guesses: 0,
        max_correct_guesses_in_game: 0,
        wallet_address: None,
        current_game_id: None,
        games_played: 0,
    };
    data.players.insert(id, player.clone());
    log::info!("Created player: {player:?}");
    HttpResponse::Created().json(player)
}

#[get("/")]
pub async fn get_all_players(
    data: AppStateData
) -> impl Responder {
    let players = data.players.clone()
        .into_iter()
        .map(|(_, k)| k)
        .collect::<Vec<_>>();
    HttpResponse::Ok().json(players)
}