use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PlayerId = String;
pub type CardDefinitionId = String;
pub type CardInstanceId = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CardType {
    Unit,
    Spell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EffectDefinition {
    Draw { amount: u32 },
    Damage { amount: u32 },
    Heal { amount: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitStats {
    pub attack: i32,
    pub health: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardDefinition {
    pub id: CardDefinitionId,
    pub name: String,
    pub cost: u32,
    pub card_type: CardType,
    pub unit_stats: Option<UnitStats>,
    pub effects: Vec<EffectDefinition>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Zone {
    Deck,
    Hand,
    Field,
    Graveyard,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardInstance {
    pub id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub owner_id: PlayerId,
    pub controller_id: PlayerId,
    pub zone: Zone,
    pub attack: Option<i32>,
    pub health: Option<i32>,
    pub max_health: Option<i32>,
    pub exhausted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
    pub id: PlayerId,
    pub hp: i32,
    pub mana: u32,
    pub max_mana: u32,
    pub deck: Vec<CardInstanceId>,
    pub hand: Vec<CardInstanceId>,
    pub field: Vec<CardInstanceId>,
    pub graveyard: Vec<CardInstanceId>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub current_player: usize,
    pub players: Vec<PlayerState>,
    pub turn: u32,
    pub card_definitions: HashMap<CardDefinitionId, CardDefinition>,
    pub card_instances: HashMap<CardInstanceId, CardInstance>,
    pub events: Vec<crate::event::GameEvent>,
}