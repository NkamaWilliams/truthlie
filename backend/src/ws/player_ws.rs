use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;
use crate::{services::game_manager::GameManager, ws::{BroadcastMessage, ClientMessage, Connect, Disconnect, StartGame}};
use std::{str::FromStr, time::{Duration, Instant}};

// TODO: See if this can't be better optimized using zero-copy. Especially state
pub struct PlayerWS {
    pub id: Uuid,
    pub game_id: Uuid,
    pub server: Addr<GameManager>,
    pub heartbeat: Instant
}

impl Actor for PlayerWS {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        // Register with GameManager
        self.server.do_send(Connect {
            player_id: self.id,
            game_id: self.game_id,
            addr: ctx.address()
        });
        self.heartbeat(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Disconnect when stopped
        self.server.do_send(Disconnect {
            player_id: self.id,
            game_id: self.game_id
        });
    }
}

impl PlayerWS {
    pub fn new(id: Uuid, game_id: Uuid, server: Addr<GameManager>) -> Self {
        Self {
            id,
            game_id,
            server,
            heartbeat: Instant::now()
        }
    }

    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > Duration::from_secs(10) {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn update_heartbeat(&mut self) {
        self.heartbeat = Instant::now();
    }
}

impl Handler<BroadcastMessage> for PlayerWS {
    type Result = ();
    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::json!(msg.0).to_string());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerWS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                log::info!("Received message on server: {}", text);

                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::StartGame { game_id, player_id }) => {
                        log::info!("Starting game: {}...", game_id);

                        // Ensure valid uuid game_id and player_id
                        match (Uuid::from_str(&game_id), Uuid::from_str(&player_id)) {
                            (Ok(game_id), Ok(player_id)) => {
                                self.server.do_send(StartGame { game_id, player_id });
                            }
                            _ => log::error!("Invalid UUIDs in StartGame: {}, {}", game_id, player_id),
                        }
                    }

                    Err(e) => {
                        log::error!("Invalid client message: {}", e);
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => self.update_heartbeat(),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            Ok(other) => log::warn!("Unhandled WS message: {:?}", other),
            _ => ()
        }
    }
}