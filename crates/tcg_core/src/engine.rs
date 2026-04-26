use crate::command::Command;
use crate::event::GameEvent;
use crate::model::{
    CardDefinition, CardInstance, CardInstanceId, CardType, EffectDefinition, GameState,
    PlayerState, UnitStats, Zone,
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
                unit_stats: Some(UnitStats {
                    attack: 1,
                    health: 2,
                }),
                effects: vec![],
            },
            CardDefinition {
                id: "knight".to_string(),
                name: "Knight".to_string(),
                cost: 2,
                card_type: CardType::Unit,
                unit_stats: Some(UnitStats {
                    attack: 2,
                    health: 3,
                }),
                effects: vec![],
            },
            CardDefinition {
                id: "archer".to_string(),
                name: "Archer".to_string(),
                cost: 1,
                card_type: CardType::Unit,
                unit_stats: Some(UnitStats {
                    attack: 2,
                    health: 1,
                }),
                effects: vec![],
            },
            CardDefinition {
                id: "guard".to_string(),
                name: "Guard".to_string(),
                cost: 2,
                card_type: CardType::Unit,
                unit_stats: Some(UnitStats {
                    attack: 1,
                    health: 4,
                }),
                effects: vec![],
            },
            CardDefinition {
                id: "fireball".to_string(),
                name: "Fireball".to_string(),
                cost: 3,
                card_type: CardType::Spell,
                unit_stats: None,
                effects: vec![EffectDefinition::Damage { amount: 3 }],
            },
        ];

        for def in definitions {
            card_definitions.insert(def.id.clone(), def);
        }

        let mut card_instances = HashMap::new();

        let p1_hand = vec![
            Self::create_card_instance(&mut card_instances, "p1_c1", "soldier", "p1", Zone::Hand),
            Self::create_card_instance(&mut card_instances, "p1_c2", "knight", "p1", Zone::Hand),
            Self::create_card_instance(&mut card_instances, "p1_c3", "fireball", "p1", Zone::Hand),
        ];

        let p2_hand = vec![
            Self::create_card_instance(&mut card_instances, "p2_c1", "archer", "p2", Zone::Hand),
            Self::create_card_instance(&mut card_instances, "p2_c2", "guard", "p2", Zone::Hand),
            Self::create_card_instance(&mut card_instances, "p2_c3", "fireball", "p2", Zone::Hand),
        ];

        let player1 = PlayerState {
            id: "p1".to_string(),
            hp: 20,
            mana: 3,
            max_mana: 3,
            deck: vec![],
            hand: p1_hand,
            field: vec![],
            graveyard: vec![],
        };

        let player2 = PlayerState {
            id: "p2".to_string(),
            hp: 20,
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
            attack: None,
            health: None,
            max_health: None,
            exhausted: false,
        };

        card_instances.insert(instance.id.clone(), instance);
        instance_id.to_string()
    }

    pub fn dispatch(&mut self, command: Command) -> Result<Vec<GameEvent>, String> {
        match command {
            Command::PlayCard {
                player_id,
                card_instance_id,
            } => self.handle_play_card(player_id, card_instance_id),

            Command::EndTurn { player_id } => self.handle_end_turn(player_id),

            Command::AttackUnit {
                player_id,
                attacker_id,
                target_id,
            } => self.handle_attack_unit(player_id, attacker_id, target_id),

            Command::AttackPlayer {
                player_id,
                attacker_id,
                target_player_id,
            } => self.handle_attack_player(player_id, attacker_id, target_player_id),
        }
    }

    fn handle_play_card(
        &mut self,
        player_id: String,
        card_instance_id: String,
    ) -> Result<Vec<GameEvent>, String> {
        if !self.is_current_player(&player_id)? {
            return Ok(vec![self.reject("It is not this player's turn")]);
        }

        let player_index = self.get_player_index(&player_id)?;
        let hand_index = self.state.players[player_index]
            .hand
            .iter()
            .position(|id| id == &card_instance_id)
            .ok_or("Card is not in player's hand")?;

        let card_instance = self
            .state
            .card_instances
            .get(&card_instance_id)
            .ok_or("Card instance not found")?
            .clone();

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

                    if let Some(stats) = &card_definition.unit_stats {
                        instance.attack = Some(stats.attack);
                        instance.health = Some(stats.health);
                        instance.max_health = Some(stats.health);
                    }

                    instance.exhausted = true;
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

    fn handle_attack_unit(
        &mut self,
        player_id: String,
        attacker_id: String,
        target_id: String,
    ) -> Result<Vec<GameEvent>, String> {
        if !self.is_current_player(&player_id)? {
            return Ok(vec![self.reject("It is not this player's turn")]);
        }

        self.validate_attacker(&player_id, &attacker_id)?;

        let attacker = self
            .state
            .card_instances
            .get(&attacker_id)
            .ok_or("Attacker not found")?
            .clone();

        let target = self
            .state
            .card_instances
            .get(&target_id)
            .ok_or("Target not found")?
            .clone();

        if target.zone != Zone::Field {
            return Ok(vec![self.reject("Target is not on the field")]);
        }

        if target.controller_id == player_id {
            return Ok(vec![self.reject("Cannot attack your own unit")]);
        }

        let attacker_attack = attacker.attack.ok_or("Attacker has no attack value")?;
        let target_attack = target.attack.ok_or("Target has no attack value")?;

        let mut events = vec![GameEvent::UnitAttackedUnit {
            attacker_id: attacker_id.clone(),
            target_id: target_id.clone(),
            attacker_damage: target_attack,
            target_damage: attacker_attack,
        }];

        let target_damage_events = self.damage_unit(&target_id, attacker_attack)?;
        let attacker_damage_events = self.damage_unit(&attacker_id, target_attack)?;

        events.extend(target_damage_events);
        events.extend(attacker_damage_events);

        if let Some(attacker_instance) = self.state.card_instances.get_mut(&attacker_id) {
            attacker_instance.exhausted = true;
        }

        for event in events.clone() {
            self.state.events.push(event);
        }

        Ok(events)
    }

    fn handle_attack_player(
        &mut self,
        player_id: String,
        attacker_id: String,
        target_player_id: String,
    ) -> Result<Vec<GameEvent>, String> {
        if !self.is_current_player(&player_id)? {
            return Ok(vec![self.reject("It is not this player's turn")]);
        }

        self.validate_attacker(&player_id, &attacker_id)?;

        if player_id == target_player_id {
            return Ok(vec![self.reject("Cannot attack yourself")]);
        }

        let attacker = self
            .state
            .card_instances
            .get(&attacker_id)
            .ok_or("Attacker not found")?
            .clone();

        let damage = attacker.attack.ok_or("Attacker has no attack value")?;
        let target_player_index = self.get_player_index(&target_player_id)?;

        self.state.players[target_player_index].hp -= damage;
        let remaining_hp = self.state.players[target_player_index].hp;

        if let Some(attacker_instance) = self.state.card_instances.get_mut(&attacker_id) {
            attacker_instance.exhausted = true;
        }

        let event = GameEvent::UnitAttackedPlayer {
            attacker_id,
            target_player_id,
            damage,
            remaining_hp,
        };

        self.state.events.push(event.clone());
        Ok(vec![event])
    }

    fn handle_end_turn(&mut self, player_id: String) -> Result<Vec<GameEvent>, String> {
        if !self.is_current_player(&player_id)? {
            return Ok(vec![self.reject("Only the current player can end the turn")]);
        }

        self.state.current_player = (self.state.current_player + 1) % self.state.players.len();
        self.state.turn += 1;

        let next_player_index = self.state.current_player;
        self.state.players[next_player_index].mana =
            self.state.players[next_player_index].max_mana;

        for card_id in self.state.players[next_player_index].field.clone() {
            if let Some(instance) = self.state.card_instances.get_mut(&card_id) {
                instance.exhausted = false;
            }
        }

        let next_player_id = self.state.players[next_player_index].id.clone();

        let event = GameEvent::TurnEnded {
            player_id,
            next_player_id,
            turn: self.state.turn,
        };

        self.state.events.push(event.clone());
        Ok(vec![event])
    }

    fn validate_attacker(&mut self, player_id: &str, attacker_id: &str) -> Result<(), String> {
        let attacker = self
            .state
            .card_instances
            .get(attacker_id)
            .ok_or("Attacker not found")?;

        if attacker.zone != Zone::Field {
            return Err("Attacker is not on the field".to_string());
        }

        if attacker.controller_id != player_id {
            return Err("Player does not control attacker".to_string());
        }

        if attacker.exhausted {
            return Err("Attacker is exhausted".to_string());
        }

        if attacker.attack.is_none() || attacker.health.is_none() {
            return Err("Attacker is not a unit".to_string());
        }

        Ok(())
    }

    fn damage_unit(
        &mut self,
        card_instance_id: &str,
        damage: i32,
    ) -> Result<Vec<GameEvent>, String> {
        let mut events = vec![];

        let remaining_health;
        let owner_id;

        {
            let unit = self
                .state
                .card_instances
                .get_mut(card_instance_id)
                .ok_or("Unit not found")?;

            let current_health = unit.health.ok_or("Unit has no health")?;
            let new_health = current_health - damage;

            unit.health = Some(new_health);
            remaining_health = new_health;
            owner_id = unit.owner_id.clone();
        }

        events.push(GameEvent::UnitDamaged {
            card_instance_id: card_instance_id.to_string(),
            damage,
            remaining_health,
        });

        if remaining_health <= 0 {
            self.move_unit_to_graveyard(card_instance_id)?;
            events.push(GameEvent::UnitDied {
                card_instance_id: card_instance_id.to_string(),
                owner_id,
            });
        }

        Ok(events)
    }

    fn move_unit_to_graveyard(&mut self, card_instance_id: &str) -> Result<(), String> {
        let owner_id = self
            .state
            .card_instances
            .get(card_instance_id)
            .ok_or("Card instance not found")?
            .owner_id
            .clone();

        let owner_index = self.get_player_index(&owner_id)?;

        self.state.players[owner_index]
            .field
            .retain(|id| id != card_instance_id);

        if !self.state.players[owner_index]
            .graveyard
            .contains(&card_instance_id.to_string())
        {
            self.state.players[owner_index]
                .graveyard
                .push(card_instance_id.to_string());
        }

        if let Some(instance) = self.state.card_instances.get_mut(card_instance_id) {
            instance.zone = Zone::Graveyard;
            instance.exhausted = true;
        }

        Ok(())
    }

    fn get_player_index(&self, player_id: &str) -> Result<usize, String> {
        self.state
            .players
            .iter()
            .position(|p| p.id == player_id)
            .ok_or_else(|| "Player not found".to_string())
    }

    fn is_current_player(&self, player_id: &str) -> Result<bool, String> {
        let current_player = self
            .state
            .players
            .get(self.state.current_player)
            .ok_or("Invalid current player index")?;

        Ok(current_player.id == player_id)
    }

    fn reject(&mut self, reason: &str) -> GameEvent {
        let event = GameEvent::CommandRejected {
            reason: reason.to_string(),
        };

        self.state.events.push(event.clone());
        event
    }
}