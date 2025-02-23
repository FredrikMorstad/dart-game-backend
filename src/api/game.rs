use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{ActiveValue::NotSet, DbErr, EntityTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::AppState,
    db::{games::get_full_game, legs::create_new_leg},
    entities::{
        games::{self},
        sets,
    },
    models::game::GameWithThrows,
};

use super::api_errors::ApiError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewGame {
    pub player_1: String,
    pub player_2: String,
    pub mode: u16,
    pub sets: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub id: Uuid,
    pub player_1: String,
    pub player_2: String,
    pub mode: u16,
    pub sets: u8,
}

const ALLOWED_MODES: &'static [u16] = &[201, 301, 501];

pub async fn get_game(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GameWithThrows>, ApiError> {
    let game = get_full_game(&state.db, id).await?;
    Ok(Json(game))
}

#[axum::debug_handler]
pub async fn create_game(
    State(state): State<AppState>,
    Json(payload): Json<NewGame>,
) -> Result<Json<Game>, ApiError> {
    if !ALLOWED_MODES.contains(&payload.mode) {
        return Err(ApiError::BadRequest(String::from("invalid game mode")));
    }

    if payload.sets > 10 {
        return Err(ApiError::BadRequest(String::from(
            "max sets per game is 10",
        )));
    }

    let mode = i32::from(payload.mode);
    let sets = i32::from(payload.sets);

    let game = state
        .db
        .transaction::<_, Game, DbErr>(|txn| {
            let next_player = payload.player_1.clone();
            Box::pin(async move {
                let new_game = games::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    player1: Set(payload.player_1.clone()),
                    player2: Set(payload.player_2.clone()),
                    player1_score: Set(0),
                    player2_score: Set(0),
                    mode: Set(mode),
                    length: Set(sets),
                    winner: Set(None),
                };

                let game = games::Entity::insert(new_game)
                    .exec_with_returning(txn)
                    .await?;

                let new_set = sets::ActiveModel {
                    id: NotSet,
                    game_id: Set(game.id),
                    number: Set(1),
                    opening: Set(next_player.clone()),
                    player1_points: Set(0),
                    player2_points: Set(0),
                    length: Set(sets),
                };

                let set = sets::Entity::insert(new_set)
                    .exec_with_returning(txn)
                    .await?;

                create_new_leg(txn, set.id, mode, 1, next_player.clone()).await?;

                return Ok(Game {
                    id: game.id,
                    player_1: game.player1,
                    player_2: game.player2,
                    mode: payload.mode,
                    sets: payload.sets,
                });
            })
        })
        .await;

    match game {
        Ok(game) => Ok(Json(game)),
        Err(e) => {
            println!("{}", e);
            Err(ApiError::UnknownError(String::from(
                "error creating game in databse",
            )))
        }
    }
}
