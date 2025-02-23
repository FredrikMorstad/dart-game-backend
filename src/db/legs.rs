use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseTransaction, DbErr, Set};

use crate::entities::legs;

pub async fn create_new_leg(
    db: &DatabaseTransaction,
    set_id: i32,
    mode: i32,
    number: i32,
    next_player: String,
) -> Result<(), DbErr> {
    legs::ActiveModel {
        id: NotSet,
        player1_score: Set(mode),
        player2_score: Set(mode),
        number: Set(number),
        set_id: Set(set_id),
        opening: Set(next_player.clone()),
        next_player: Set(next_player.clone()),
    }
    .save(db)
    .await?;
    Ok(())
}
