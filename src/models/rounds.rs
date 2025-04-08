use serde::{Deserialize, Serialize};

use crate::entities::{rounds, throws};

use super::throw::Throw;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Round {
    pub id: i32,
    pub number: i32,
    pub leg_id: i32,
    pub throws: Vec<Throw>,
}

impl From<(rounds::Model, Vec<throws::Model>)> for Round {
    fn from((round, throws): (rounds::Model, Vec<throws::Model>)) -> Self {
        let round_throws: Vec<Throw> = throws
            .into_iter()
            .map(|throw_model| Throw {
                id: throw_model.id,
                value: throw_model.value,
                thrower: throw_model.thrower,
            })
            .collect();

        Round {
            id: round.id,
            number: round.number,
            leg_id: round.leg_id,
            throws: round_throws,
        }
    }
}
