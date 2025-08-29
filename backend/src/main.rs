use std::io;
use actix::prelude::*;
use actix_web::{App, HttpServer, main, web};
use backend::services::game_manager::GameManager;
use backend::state::AppState;
use backend::handlers;
use backend::routes;
#[main]
async fn main() -> io::Result<()> {
    println!("Starting API at http://localhost:8080 ...");

    let game_state = AppState::new().data();
    let manager = web::Data::new(GameManager::new(game_state.clone()).start());

    HttpServer::new(move || {
        App::new()
            .app_data(game_state.clone())
            .app_data(manager.clone())
            .service(handlers::base)
            .service(
                web::scope("/players")
                    .configure(routes::register_player_routes)
            )
            .service(
                web::scope("/games")
                    .configure(routes::register_game_routes)
            )
            .service(
                web::scope("/ws")
                    .configure(routes::register_ws_routes)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
