use serde::{Deserialize, Serialize};

use crate::entities::{legs, throws};

use super::throw::Throw;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Leg {
    pub id: i32,
    pub number: i32,
    pub player1_score: i32,
    pub player2_score: i32,
    pub set_id: i32,
    pub opening: String,
    pub next_player: String,
    pub throws: Vec<Throw>,
}

impl From<(legs::Model, Vec<throws::Model>)> for Leg {
    fn from((leg, throws): (legs::Model, Vec<throws::Model>)) -> Self {
        let throws: Vec<Throw> = throws
            .iter()
            .map(|throw_model| Throw {
                id: throw_model.id,
                value: throw_model.value.clone(),
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
            throws,
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
            throws: vec![],
        }
    }
}
