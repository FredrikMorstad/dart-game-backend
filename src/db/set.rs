use sea_orm::{ActiveValue::NotSet, DatabaseTransaction, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::{db::legs::create_new_leg, entities::sets, models::set::Set};

pub async fn create_new_set_with_leg(
    tx: &DatabaseTransaction,
    game_id: Uuid,
    game_mode: i32,
    number: i32,
    next_player: String,
    length: i32,
) -> Result<Set, DbErr> {
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

    let leg = create_new_leg(tx, set.id, game_mode, 1, next_player).await?;

    let new_set = Set {
        id: set.id,
        number: set.number,
        length: set.length,
        opening: set.opening,
        game_id: set.game_id,
        player1_points: set.player1_points,
        player2_points: set.player2_points,
        legs: [leg].to_vec(),
    };

    Ok(new_set)
}
