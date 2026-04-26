use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub cost: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub id: String,
    pub hand: Vec<Card>,
    pub field: Vec<Card>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub current_player: usize,
    pub players: Vec<PlayerState>,
    pub turn: u32,
    pub events: Vec<crate::event::GameEvent>,
}