use uuid::Uuid;

use crate::entities::{games, legs, sets, throws};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameWithThrows {
    pub id: Uuid,
    pub mode: i32,
    pub player_1: String,
    pub player_2: String,
    pub player_1_score: i32,
    pub player_2_score: i32,
    pub length: i32,
    pub winner: Option<String>,
    pub sets: Vec<Set>,
}

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Throw {
    pub id: i32,
    pub value: String,
}

impl From<(games::Model, Vec<sets::Model>)> for GameWithThrows {
    fn from((game, sets): (games::Model, Vec<sets::Model>)) -> Self {
        let sets: Vec<Set> = sets
            .iter()
            .map(|set_model| Set {
                id: set_model.id,
                game_id: set_model.game_id,
                number: set_model.number,
                opening: set_model.opening.clone(),
                player1_points: set_model.player1_points,
                player2_points: set_model.player2_points,
                length: set_model.length,
                legs: vec![],
            })
            .collect();
        GameWithThrows {
            id: game.id,
            mode: game.mode,
            player_1: game.player1,
            player_2: game.player2,
            player_1_score: game.player1_score,
            player_2_score: game.player2_score,
            length: game.length,
            winner: game.winner,
            sets,
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
