use serde::{Deserialize, Serialize};

use crate::entities::{legs, rounds};

use super::rounds::Round;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Leg {
    pub id: i32,
    pub number: i32,
    pub player1_score: i32,
    pub player2_score: i32,
    pub set_id: i32,
    pub opening: String,
    pub next_player: String,
    pub rounds: Vec<Round>,
}

impl From<(legs::Model, Vec<rounds::Model>)> for Leg {
    fn from((leg, rounds): (legs::Model, Vec<rounds::Model>)) -> Self {
        let leg_rounds: Vec<Round> = rounds
            .iter()
            .map(|round_model| Round {
                id: round_model.id,
                number: round_model.number,
                leg_id: round_model.leg_id,
                throws: vec![],
            })
            .collect();
        Leg {
            id: leg.id,
            number: leg.number,
            player1_score: leg.player1_score,
            player2_score: leg.player2_score,
            set_id: leg.set_id,
            next_player: leg.next_player.clone(),
            opening: leg.opening,
            rounds: leg_rounds,
        }
    }
}

impl From<legs::Model> for Leg {
    fn from(leg: legs::Model) -> Self {
        Leg {
            id: leg.id,
            number: leg.number,
            set_id: leg.set_id,
            opening: leg.opening,
            next_player: leg.next_player,
            player1_score: leg.player1_score,
            player2_score: leg.player2_score,
            rounds: vec![],
        }
    }
}
