use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter};

use crate::{
    entities::{legs, rounds, throws},
    models::{leg::Leg, rounds::Round},
};

pub async fn get_leg_with_rounds(db: &DatabaseTransaction, leg_id: i32) -> Result<Leg, DbErr> {
    let mut leg_with_rounds_res: Leg = legs::Entity::find_by_id(leg_id)
        .find_with_related(rounds::Entity)
        .all(db)
        .await?
        .first()
        .ok_or(DbErr::RecordNotFound("could not find leg".to_string()))?
        .clone()
        .into();

    let rounds: Vec<Round> = rounds::Entity::find()
        .filter(rounds::Column::LegId.eq(leg_with_rounds_res.id))
        .find_with_related(throws::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(|round| round.into())
        .collect();

    leg_with_rounds_res.rounds = rounds;

    Ok(leg_with_rounds_res)
}
