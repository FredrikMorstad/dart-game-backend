use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{legs, sets};

use super::leg::Leg;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set {
    pub id: i32,
    pub number: i32,
    pub player1_points: i32,
    pub player2_points: i32,
    pub game_id: Uuid,
    pub opening: String,
    pub length: i32,
    pub legs: Vec<Leg>,
}

impl From<sets::Model> for Set {
    fn from(leg: sets::Model) -> Self {
        Set {
            id: leg.id,
            number: leg.number,
            opening: leg.opening,
            player1_points: leg.player1_points,
            player2_points: leg.player2_points,
            game_id: leg.game_id,
            length: leg.length,
            legs: vec![],
        }
    }
}

impl From<(sets::Model, Vec<legs::Model>)> for Set {
    fn from((set, legs): (sets::Model, Vec<legs::Model>)) -> Self {
        let legs: Vec<Leg> = legs
            .iter()
            .map(|leg_entity| Leg {
                id: leg_entity.id,
                number: leg_entity.number,
                player1_score: leg_entity.player1_score,
                player2_score: leg_entity.player2_score,
                set_id: leg_entity.set_id,
                next_player: leg_entity.next_player.clone(),
                opening: leg_entity.opening.clone(),
                throws: vec![],
            })
            .collect();
        Set {
            id: set.id,
            number: set.number,
            player1_points: set.player1_points,
            player2_points: set.player2_points,
            game_id: set.game_id,
            opening: set.opening.clone(),
            length: set.length,
            legs,
        }
    }
}
