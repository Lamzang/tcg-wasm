use crate::command::Command;
use crate::event::GameEvent;
use crate::model::{Card, GameState, PlayerState};


pub struct CoreEngine {
    pub state: GameState,
}

impl CoreEngine {
    pub fn new() -> Self {
        let player1 = PlayerState {
            id: "p1".to_string(),
            hand: vec![
                Card{id: "c1".to_string(),
                name: "Soldier".to_string(),
                cost:1,
                },
                Card{id: "c2".to_string(),
                name: "Shield".to_string(),
                cost:1,
                },
            ],
            field: vec![],
        };

        let player2 = PlayerState {
            id: "p2".to_string(),
            hand: vec![
                Card{id: "c3".to_string(),
                name: "Archer".to_string(),
                cost:1,
                },
                Card{id: "c4".to_string(),
                name: "Mage".to_string(),
                cost:1,
                },
            ],
            field: vec![],
        };

        Self {
            state: GameState { current_player: 0, players: vec![player1, player2], turn: 1, events: vec![GameEvent::GameStarted], },
        }


    }

    pub fn dispatch(&mut self, command:Command) -> Result<Vec<GameEvent>, String> {
        match command {
            Command::PlayCard { player_id, card_id } => {
                self.handle_play_card(player_id, card_id)
            }
            Command::EndTurn { player_id } => self.handle_end_turn(player_id)
        }
    }

    fn handle_play_card(&mut self, player_id:String, card_id: String) -> Result<Vec<GameEvent>, String> {
        let current_player = self.state.players.get(self.state.current_player).ok_or("Invalid current player index")?;

        if current_player.id != player_id {
            let event = GameEvent::CommandRejected { reason: "It is not this player's turn".to_string() };
            self.state.events.push(event.clone());
            return Ok(vec![event]);
        };

        let player = self.state.players.iter_mut().find(|p| p.id == player_id).ok_or("Player not found")?;

        let card_index = player.hand.iter().position(|card| card.id == card_id).ok_or("Card not found in hand")?;

        let card = player.hand.remove(card_index);
        let event = GameEvent::CardPlayed { player_id: player.id.clone(), card_id: card.id.clone(), card_name: card.name.clone() };

        player.field.push(card);
        self.state.events.push(event.clone());

        Ok(vec![event])

    }

    fn handle_end_turn(&mut self, player_id: String,) -> Result<Vec<GameEvent>,String> {
        let current_player = self
            .state
            .players
            .get(self.state.current_player)
            .ok_or("Invalid current player index")?;

        if current_player.id != player_id {
            let event = GameEvent::CommandRejected {
                reason: "Only the current player can end the turn".to_string(),
            };
            self.state.events.push(event.clone());
            return Ok(vec![event]);
        }

        self.state.current_player = (self.state.current_player+1) % self.state.players.len();
        self.state.turn+=1;
        let next_player_id = self.state.players[self.state.current_player].id.clone();

        let event = GameEvent::TurnEnded { player_id, next_player_id, turn: self.state.turn };
        self.state.events.push(event.clone());
        Ok(vec![event])
    }

}