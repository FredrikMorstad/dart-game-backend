use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionError, TransactionTrait, Unchanged};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{
        api_errors::ApiError,
        games::{throw::Point, utils::check_score},
    },
    app::AppState,
    entities::{
        games::{self},
        legs::{self},
        throws,
    },
};

enum Status {
    Playing,
    Finished,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Throw {
    pub game_id: Uuid,
    pub throw: String,
    pub player: String,
}

pub async fn post_throw(
    State(state): State<AppState>,
    Json(payload): Json<Throw>,
) -> Result<StatusCode, ApiError> {
    let throw: &str = &payload.throw.clone();
    let point = Point::new(throw)?;

    let games = games::Entity::find_by_id(payload.game_id)
        .find_with_related(legs::Entity)
        .all(&state.db)
        .await?;

    // should only be one game based on id
    let active_game = games
        .first()
        .cloned()
        .ok_or(ApiError::UnknownError(String::from("error finding game")))?;

    let active_legs: Vec<legs::Model> = active_game
        .1
        .clone()
        .iter()
        .filter(|leg| leg.player1_score != 0 && leg.player2_score != 0)
        .cloned()
        .collect();

    let leg = active_legs
        .first()
        .cloned()
        .ok_or(ApiError::UnknownError(String::from(
            "could not find an active leg in this game",
        )))?;

    println!("legs: {:?}", leg);

    let player_1 = active_game.0.player1;
    let player_2 = active_game.0.player2;

    let mut update_leg = legs::ActiveModel {
        id: Unchanged(leg.id),
        ..Default::default()
    };

    state
        .db
        .transaction::<_, (), ApiError>(|tx| {
            Box::pin(async move {
                let throw = throws::ActiveModel {
                    game_id: Set(payload.game_id),
                    leg_id: Set(leg.id),
                    value: Set(point.notation.to_string()),
                    ..Default::default()
                };

                throws::Entity::insert(throw).exec(tx).await?;

                println!("Throw: {}", point.score);

                let status = match payload.player {
                    player if player == player_1 => {
                        let mut status = Status::Playing;
                        if let Some(new_score) = check_score(leg.player1_score, point.score) {
                            if new_score == 0 {
                                status = Status::Finished;
                            }
                            update_leg.player1_score = Set(new_score);
                            println!(
                                "Player: {}, old_score: {}, new_score: {:?}",
                                player_1, leg.player1_score, new_score
                            );
                        }
                        Ok::<Status, ApiError>(status)
                    }
                    player if player == player_2 => {
                        let mut status = Status::Playing;
                        if let Some(new_score) = check_score(leg.player2_score, point.score) {
                            if new_score == 0 {
                                status = Status::Finished;
                            }
                            update_leg.player2_score = Set(new_score);
                            println!(
                                "Player: {}, old_score: {}, new_score: {:?}",
                                player_1, leg.player2_score, new_score
                            );
                        }
                        Ok(status)
                    }
                    _ => return Err(ApiError::BadRequest(String::from("Invalid player name"))),
                }?;

                if matches!(status, Status::Finished) {
                    println!("Should create new leg");
                    legs::ActiveModel {
                        player1_score: Set(active_game.0.mode),
                        player2_score: Set(active_game.0.mode),
                        number: Set(leg.number + 1),
                        set: Set(leg.set + 1),
                        game_id: Set(active_game.0.id),
                        ..Default::default()
                    }
                    .save(tx)
                    .await?;
                }

                legs::Entity::update(update_leg).exec(tx).await?;

                Ok(())
            })
        })
        .await
        .map_err(|err| match err {
            TransactionError::Connection(db_err) => ApiError::DatabaseError(db_err),
            TransactionError::Transaction(api_error) => api_error,
        })?;

    Ok(StatusCode::OK)
}
