// Global State Defined here until DB is created

use actix_web::web;
use dashmap::DashMap;
use uuid::Uuid;
use crate::models::{game::GameSession, player::Player};

pub struct AppState {
    pub sessions: DashMap<Uuid, GameSession>,
    pub players: DashMap<Uuid, Player>
}

pub type AppStateData = web::Data<AppState>;

impl AppState {
    pub fn new() -> Self {
        AppState {
            sessions: DashMap::new(),
            players: DashMap::new()
        }
    }

    pub fn data(self) -> AppStateData {
        web::Data::new(self)
    }
}