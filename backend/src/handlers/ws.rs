use actix::Addr;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{services::game_manager::GameManager, ws::player_ws::PlayerWS};

#[get("/{game_id}/{player_id}")]
pub async fn player_ws_connect(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(Uuid, Uuid)>,
    manager: web::Data<Addr<GameManager>>
) -> Result<HttpResponse, Error> {
    let (game_id, player_id) = path.into_inner();

    let ws_actor = PlayerWS::new(player_id, game_id, manager.get_ref().clone());

    ws::start(ws_actor, &req, stream)
}