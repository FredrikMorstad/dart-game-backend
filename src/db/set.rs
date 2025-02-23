use sea_orm::{ActiveValue::NotSet, DatabaseTransaction, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::{db::legs::create_new_leg, entities::sets};

pub async fn create_new_set_with_leg(
    tx: &DatabaseTransaction,
    game_id: Uuid,
    game_mode: i32,
    number: i32,
    next_player: String,
    length: i32,
) -> Result<(), DbErr> {
    let new_set = sets::ActiveModel {
        id: NotSet,
        player1_points: Set(0),
        player2_points: Set(0),
        number: Set(number),
        game_id: Set(game_id),
        opening: Set(next_player.clone()),
        length: Set(length),
    };

    let set = sets::Entity::insert(new_set)
        .exec_with_returning(tx)
        .await?;

    create_new_leg(tx, set.id, game_mode, 1, next_player).await?;
    Ok(())
}
