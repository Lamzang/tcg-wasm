use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
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
    pub log: Vec<String>,
}

#[wasm_bindgen]
pub struct Engine {
    state: GameState,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        console_error_panic_hook::set_once();

        let player1 = PlayerState {
            id: "p1".to_string(),
            hand: vec![Card{
                id: "attack1".to_string(),
                name: "공격하라".to_string(),
                cost: 1,
            }],
            field: vec![],
        };

        let player2 = PlayerState {
            id: "p1".to_string(),
            hand: vec![Card{
                id: "shield1".to_string(),
                name: "수비하라".to_string(),
                cost: 1,
            }],
            field: vec![],
        };

        Engine {
            state: GameState { current_player: 0, players: vec![player1,player2], turn: 1, log: vec!["Game Start".to_string()] }
        }

    }

    pub fn get_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.state).unwrap()
    }

    pub fn play_first_card(&mut self) -> Result<JsValue, JsValue> {
        let idx = self.state.current_player;
        let player = &mut self.state.players[idx];

        if player.hand.is_empty() {
            return Err(JsValue::from_str("No card in hand"));
        }

        let card = player.hand.remove(0);
        let card_name = card.name.clone();
        player.field.push(card);

        self.state.log.push(format!("{} played {}", player.id, card_name));

        Ok(serde_wasm_bindgen::to_value(&self.state).unwrap())
    }

    pub fn end_turn(&mut self) -> JsValue {
        self.state.current_player = {self.state.current_player + 1} % self.state.players.len();
        self.state.turn += 1;

        let next_player_id = self.state.players[self.state.current_player].id.clone();
        self.state.log.push(format!("Turn Ended. Now {}'s turn",next_player_id));
        
        serde_wasm_bindgen::to_value(&self.state).unwrap()
    }
}