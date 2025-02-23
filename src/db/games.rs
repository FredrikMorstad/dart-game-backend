use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    entities::{games, legs, sets, throws},
    models::game::{GameWithThrows, Leg},
};

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

    let legs_with_throw_res: Vec<Leg> = legs::Entity::find()
        .filter(legs::Column::SetId.is_in(set_ids))
        .find_with_related(throws::Entity)
        .all(db)
        .await?
        .iter()
        .map(|leg| Leg::from(leg.clone()))
        .collect();

    game.sets.iter_mut().for_each(|set| {
        let mut legs: Vec<Leg> = legs_with_throw_res
            .clone()
            .into_iter()
            .filter(|leg| leg.set_id == set.id)
            .collect();
        set.legs.append(&mut legs);
    });
    Ok(game)
}
