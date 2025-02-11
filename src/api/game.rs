use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::AppState,
    entities::{
        games::{self},
        legs,
    },
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

// struct GameWithThrows {
//     pub id: Uuid,
//     pub player_1: String,
//     pub player_2: String,
//     pub mode: u16,
//     pub sets: u8,
//     legs: Vec<Leg>
// }

pub async fn get_game(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Game, ApiError> {
    let game = games::Entity::find_by_id(id)
        .find_with_related(legs::Entity)
        .all(&state.db)
        .await?;

    for legs in game {
        println!("model: {:?}, legs: {:?}", legs.0, legs.1);
    }

    Ok(StatusCode::OK)
}

#[axum::debug_handler]
pub async fn create_game(
    State(state): State<AppState>,
    Json(payload): Json<NewGame>,
) -> Result<Json<Game>, ApiError> {
    if !ALLOWED_MODES.contains(&payload.mode) {
        return Err(ApiError::BadRequest(String::from("Invalid game mode")));
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
            Box::pin(async move {
                let new_game = games::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    player1: Set(payload.player_1.clone()),
                    player2: Set(payload.player_2.clone()),
                    mode: Set(mode),
                    sets: Set(sets),
                    ..Default::default()
                };

                let game = games::Entity::insert(new_game)
                    .exec_with_returning(txn)
                    .await?;

                legs::ActiveModel {
                    player1_score: Set(mode),
                    player2_score: Set(mode),
                    number: Set(1),
                    set: Set(1),
                    game_id: Set(game.id),
                    ..Default::default()
                }
                .save(txn)
                .await?;

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
