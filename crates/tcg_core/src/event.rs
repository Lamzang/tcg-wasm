use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone, Debug)]
#[serde(tag = "type")]
pub enum GameEvent {
    GameStarted,

    CardPlayed {
        player_id: String,
        card_id: String,
        card_name: String
    },

    TurnEnded {
        player_id: String,
        next_player_id: String,
        turn:u32
    },
    CommandRejected {
        reason: String
    },
}