use actix_web::{get, HttpResponse, Responder};

pub mod player;
pub mod game;

#[get("/")]
pub async fn base() -> impl Responder {
    HttpResponse::Ok().json("Welcome to Truth or Lie API! May the best liar win!")
}