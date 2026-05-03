use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum GameEvent {
    GameStarted,

    CardPlayed {
        player_id: String,
        card_instance_id: String,
        card_definition_id: String,
        card_name: String,
        cost: u32,
    },

    UnitAttackedUnit {
        attacker_id: String,
        target_id: String,
        attacker_damage: i32,
        target_damage: i32,
    },

    UnitAttackedPlayer {
        attacker_id: String,
        target_player_id: String,
        damage: i32,
        remaining_hp: i32,
    },

    UnitDamaged {
        card_instance_id: String,
        damage: i32,
        remaining_health: i32,
    },

    UnitDied {
        card_instance_id: String,
        owner_id: String,
    },

    TurnEnded {
        player_id: String,
        next_player_id: String,
        turn: u32,
    },

    CommandRejected {
        reason: String,
    },
    CardDrawn {
    player_id: String,
    card_instance_id: String,
},

UnitSummoned {
    player_id: String,
    card_instance_id: String,
    card_definition_id: String,
},

UnitBuffed {
    card_instance_id: String,
    attack_delta: i32,
    health_delta: i32,
    new_attack: i32,
    new_health: i32,
},

UnitHealed {
    card_instance_id: String,
    amount: i32,
    new_health: i32,
},

PlayerDamaged {
    player_id: String,
    damage: i32,
    remaining_hp: i32,
},

PlayerHealed {
    player_id: String,
    amount: i32,
    new_hp: i32,
},
}