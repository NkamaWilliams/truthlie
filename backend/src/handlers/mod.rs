use actix_web::{get, HttpResponse, Responder};

pub mod player;
pub mod game;
pub mod ws;

#[get("/")]
pub async fn base() -> impl Responder {
    HttpResponse::Ok().json("Welcome to Truth or Lie API! May the best liar win!")
}