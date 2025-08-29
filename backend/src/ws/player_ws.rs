use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;
use crate::{services::game_manager::GameManager, ws::{BroadcastMessage, ClientMessage, Connect, Disconnect}};
use std::time::{Duration, Instant};

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
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerWS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received guess on server: {}", text);
                self.server.do_send(ClientMessage {
                    player_id: self.id,
                    game_id: self.game_id,
                    text: text.into()
                });
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => self.update_heartbeat(),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            Ok(other) => log::warn!("Unhandled WS message: {:?}", other),
            _ => ()
        }
    }
}