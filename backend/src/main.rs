use std::io;

use actix_web::{App, HttpServer, main, web};
use backend::state::AppState;
use backend::handlers;
use backend::routes;
#[main]
async fn main() -> io::Result<()> {
    println!("Starting API at http://localhost:8080 ...");

    let game_state = AppState::new().data();

    HttpServer::new(move || {
        App::new()
            .app_data(game_state.clone())
            .service(handlers::base)
            .service(
                web::scope("/players")
                    .configure(routes::register_player_routes)
            )
            .service(
                web::scope("/games")
                    .configure(routes::register_game_routes)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
