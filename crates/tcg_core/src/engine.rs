use crate::command::Command;
use crate::event::GameEvent;
use crate::model::{
    CardDefinition, CardInstance, CardInstanceId, CardType, EffectDefinition, GameState,
    PlayerState, Zone,
};
use std::collections::HashMap;

pub struct CoreEngine {
    pub state: GameState,
}

impl CoreEngine {
    pub fn new() -> Self {
        let mut card_definitions = HashMap::new();

        let definitions = vec![
            CardDefinition {
                id: "soldier".to_string(),
                name: "Soldier".to_string(),
                cost: 1,
                card_type: CardType::Unit,
                effects: vec![],
            },
            CardDefinition {
                id: "knight".to_string(),
                name: "Knight".to_string(),
                cost: 2,
                card_type: CardType::Unit,
                effects: vec![],
            },
            CardDefinition {
                id: "archer".to_string(),
                name: "Archer".to_string(),
                cost: 1,
                card_type: CardType::Unit,
                effects: vec![],
            },
            CardDefinition {
                id: "fireball".to_string(),
                name: "Fireball".to_string(),
                cost: 3,
                card_type: CardType::Spell,
                effects: vec![EffectDefinition::Damage { amount: 3 }],
            },
            CardDefinition {
                id: "healing_light".to_string(),
                name: "Healing Light".to_string(),
                cost: 2,
                card_type: CardType::Spell,
                effects: vec![EffectDefinition::Heal { amount: 2 }],
            },
        ];

        for def in definitions {
            card_definitions.insert(def.id.clone(), def);
        }

         let mut card_instances = HashMap::new();

        let p1_hand = vec![
            Self::create_card_instance(
                &mut card_instances,
                "p1_c1",
                "soldier",
                "p1",
                Zone::Hand,
            ),
            Self::create_card_instance(
                &mut card_instances,
                "p1_c2",
                "knight",
                "p1",
                Zone::Hand,
            ),
            Self::create_card_instance(
                &mut card_instances,
                "p1_c3",
                "fireball",
                "p1",
                Zone::Hand,
            ),
        ];

        let p2_hand = vec![
            Self::create_card_instance(
                &mut card_instances,
                "p2_c1",
                "archer",
                "p2",
                Zone::Hand,
            ),
            Self::create_card_instance(
                &mut card_instances,
                "p2_c2",
                "healing_light",
                "p2",
                Zone::Hand,
            ),
        ];

        let player1 = PlayerState {
            id: "p1".to_string(),
            mana: 3,
            max_mana: 3,
            deck: vec![],
            hand: p1_hand,
            field: vec![],
            graveyard: vec![],
        };

        let player2 = PlayerState {
            id: "p2".to_string(),
            mana: 3,
            max_mana: 3,
            deck: vec![],
            hand: p2_hand,
            field: vec![],
            graveyard: vec![],
        };

        Self {
            state: GameState {
                current_player: 0,
                players: vec![player1, player2],
                turn: 1,
                card_definitions,
                card_instances,
                events: vec![GameEvent::GameStarted],
            },
        }


    }

    fn create_card_instance(
        card_instances: &mut HashMap<CardInstanceId, CardInstance>,
        instance_id: &str,
        definition_id: &str,
        owner_id: &str,
        zone: Zone,
    ) -> CardInstanceId {
        let instance = CardInstance {
            id: instance_id.to_string(),
            definition_id: definition_id.to_string(),
            owner_id: owner_id.to_string(),
            controller_id: owner_id.to_string(),
            zone,
        };

        card_instances.insert(instance.id.clone(), instance);
        instance_id.to_string()
    }

    pub fn dispatch(&mut self, command:Command) -> Result<Vec<GameEvent>, String> {
        match command {
            Command::PlayCard { player_id, card_instance_id } => {
                self.handle_play_card(player_id, card_instance_id)
            }
            Command::EndTurn { player_id } => self.handle_end_turn(player_id)
        }
    }

    fn handle_play_card(&mut self, player_id:String, card_instance_id: String) -> Result<Vec<GameEvent>, String> {
        let current_player = self.state.players.get(self.state.current_player).ok_or("Invalid current player index")?;

        if current_player.id != player_id {
            let event = GameEvent::CommandRejected { reason: "It is not this player's turn".to_string() };
            self.state.events.push(event.clone());
            return Ok(vec![event]);
        };

        let player_index = self.state.players.iter().position(|p| p.id == player_id).ok_or("Player not found")?;

        let hand_index = self.state.players[player_index].hand.iter().position(|id| id == &card_instance_id).ok_or("Card is not player's hand")?;

        let card_instance = self.state.card_instances.get(&card_instance_id).ok_or("Card Instance not found").clone()?;

        if card_instance.controller_id != player_id {
            return Ok(vec![self.reject("Player does not control this card")]);
        }

        let card_definition = self
            .state
            .card_definitions
            .get(&card_instance.definition_id)
            .ok_or("Card definition not found")?
            .clone();

        if self.state.players[player_index].mana < card_definition.cost {
            return Ok(vec![self.reject("Not enough mana")]);
        }

        self.state.players[player_index].mana -= card_definition.cost;

        self.state.players[player_index].hand.remove(hand_index);

        match card_definition.card_type {
            CardType::Unit => {
                self.state.players[player_index]
                    .field
                    .push(card_instance_id.clone());

                if let Some(instance) = self.state.card_instances.get_mut(&card_instance_id) {
                    instance.zone = Zone::Field;
                }
            }
            CardType::Spell => {
                self.state.players[player_index]
                    .graveyard
                    .push(card_instance_id.clone());

                if let Some(instance) = self.state.card_instances.get_mut(&card_instance_id) {
                    instance.zone = Zone::Graveyard;
                }
            }
        }

        let event = GameEvent::CardPlayed {
            player_id,
            card_instance_id,
            card_definition_id: card_definition.id,
            card_name: card_definition.name,
            cost: card_definition.cost,
        };

        self.state.events.push(event.clone());

        Ok(vec![event])

    }

    fn handle_end_turn(&mut self, player_id: String) -> Result<Vec<GameEvent>, String> {
        let current_player = self
            .state
            .players
            .get(self.state.current_player)
            .ok_or("Invalid current player index")?;

        if current_player.id != player_id {
            return Ok(vec![self.reject("Only the current player can end the turn")]);
        }

        self.state.current_player = (self.state.current_player + 1) % self.state.players.len();
        self.state.turn += 1;

        let next_player_index = self.state.current_player;
        self.state.players[next_player_index].mana =
            self.state.players[next_player_index].max_mana;

        let next_player_id = self.state.players[next_player_index].id.clone();

        let event = GameEvent::TurnEnded {
            player_id,
            next_player_id,
            turn: self.state.turn,
        };

        self.state.events.push(event.clone());

        Ok(vec![event])
    }

    fn reject(&mut self, reason: &str) -> GameEvent {
        let event = GameEvent::CommandRejected {
            reason: reason.to_string(),
        };

        self.state.events.push(event.clone());
        event
    }

}