use uuid::Uuid;

use crate::entities::{games, sets};
use serde::{Deserialize, Serialize};

use super::set::Set;

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
