use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    entities::{games, legs, rounds, sets, throws},
    models::{game::GameWithThrows, leg::Leg, rounds::Round},
};

// should probably create a custom query over joining each query manually
pub async fn get_full_game(db: &DatabaseConnection, id: Uuid) -> Result<GameWithThrows, DbErr> {
    let mut game: GameWithThrows = games::Entity::find_by_id(id)
        .find_with_related(sets::Entity)
        .all(db)
        .await?
        .first()
        .ok_or(DbErr::RecordNotFound(String::from("game not found")))?
        .clone()
        .into();

    let set_ids: Vec<i32> = game.sets.clone().iter().map(|set| set.id).collect();

    let mut legs_with_rounds_res: Vec<Leg> = legs::Entity::find()
        .filter(legs::Column::SetId.is_in(set_ids))
        .find_with_related(rounds::Entity)
        .all(db)
        .await?
        .iter()
        .map(|leg| Leg::from(leg.clone()))
        .collect();

    println!("legs: {:?}", legs_with_rounds_res);

    let leg_ids: Vec<i32> = legs_with_rounds_res
        .clone()
        .into_iter()
        .map(|leg| leg.id)
        .collect();

    let rounds: Vec<Round> = rounds::Entity::find()
        .filter(rounds::Column::LegId.is_in(leg_ids))
        .find_with_related(throws::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(|round| round.into())
        .collect();

    legs_with_rounds_res.iter_mut().for_each(|leg| {
        let correct_round: Vec<Round> = rounds
            .clone()
            .into_iter()
            .filter(|round| round.leg_id == leg.id)
            .collect();

        println!("correct_round: {:?}", correct_round);

        leg.rounds = correct_round;
    });

    println!("legs: {:?}", legs_with_rounds_res);

    game.sets.iter_mut().for_each(|set| {
        let mut legs: Vec<Leg> = legs_with_rounds_res
            .clone()
            .into_iter()
            .filter(|leg| leg.set_id == set.id)
            .collect();
        set.legs.append(&mut legs);
    });
    Ok(game)
}
