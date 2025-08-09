use actix_web::{ web };

use crate::handlers::{player, game};

pub fn register_player_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(player::create_player)
        .service(player::get_all_players);
}

pub fn register_game_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(game::create_game)
        .service(game::get_games)
        .service(game::join_game)
        .service(game::start_game);
}