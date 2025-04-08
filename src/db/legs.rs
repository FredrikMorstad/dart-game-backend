use sea_orm::{ActiveValue::NotSet, DatabaseTransaction, DbErr, EntityTrait, Set};

use crate::{entities::legs, models::leg::Leg};

pub async fn create_new_leg(
    tx: &DatabaseTransaction,
    set_id: i32,
    mode: i32,
    number: i32,
    next_player: String,
) -> Result<Leg, DbErr> {
    let model = legs::ActiveModel {
        id: NotSet,
        player1_score: Set(mode),
        player2_score: Set(mode),
        number: Set(number),
        set_id: Set(set_id),
        opening: Set(next_player.clone()),
        next_player: Set(next_player.clone()),
    };

    let leg = legs::Entity::insert(model).exec_with_returning(tx).await?;

    Ok(Leg::from(leg))
}
