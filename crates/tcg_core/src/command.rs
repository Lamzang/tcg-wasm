use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Command {
    PlayCard {
        player_id: String,
        card_instance_id: String,
    },
    EndTurn {
        player_id:String
    }
}