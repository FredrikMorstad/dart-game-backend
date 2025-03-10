use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionError, TransactionTrait, Unchanged};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{
        api_errors::{ApiError, NotationError},
        games::{
            throw::Point,
            utils::{check_score, leg_win_score_update},
        },
    },
    channels::Producer,
    db::{games::get_full_game, legs::create_new_leg, set::create_new_set_with_leg},
    entities::{games, legs, sets, throws},
    models::{leg::Leg, set::Set},
};

pub enum Status {
    Playing,
    Finished,
    SetFinished,
    GameFinished,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Throw {
    pub game_id: Uuid,
    pub throws: Vec<String>,
    pub player: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerScores {
    pub round_score: i32,
    pub leg_score: i32,
    pub set_score: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameEvent<T> {
    pub id: Uuid,
    pub event_type: String,
    pub data: T,
}

pub async fn post_throw(
    State(state): State<Producer>,
    Json(payload): Json<Throw>,
) -> Result<StatusCode, ApiError> {
    if payload.throws.len() > 3 {
        return Err(ApiError::BadRequest(String::from(
            "invalid round: too many throws",
        )));
    }

    let throws: Result<Vec<Point>, NotationError> = payload
        .throws
        .into_iter()
        .map(|throw| throw.parse())
        .collect();

    let points = throws?;

    let game = get_full_game(&state.db, payload.game_id).await?;

    if game.winner.is_some() {
        return Err(ApiError::BadRequest(String::from("game finished")));
    }

    if payload.player != game.player_1 && payload.player != game.player_2 {
        return Err(ApiError::BadRequest(format!(
            "player: {} is not a player in this game",
            payload.player
        )));
    }

    let active_set = game
        .clone()
        .sets
        .into_iter()
        .max_by_key(|set| set.number)
        .ok_or(ApiError::UnknownError(String::from("no active set")))?;

    let leg: Leg = active_set
        .clone()
        .legs
        .into_iter()
        .max_by_key(|leg| leg.number)
        .ok_or(ApiError::UnknownError(String::from("no active leg")))?;

    if leg.next_player != payload.player {
        return Err(ApiError::BadRequest(String::from(
            "player throws before turn",
        )));
    }

    let player_1 = game.player_1.clone();
    let player_2 = game.player_2.clone();

    let mut update_leg = legs::ActiveModel {
        id: Unchanged(leg.id),
        ..Default::default()
    };

    let mut update_set = sets::ActiveModel {
        id: Unchanged(active_set.id),
        ..Default::default()
    };

    let mut update_game = games::ActiveModel {
        id: Unchanged(game.id),
        ..Default::default()
    };

    let mut scores = match payload.player.clone() {
        player if player == player_1 => PlayerScores {
            round_score: leg.player1_score,
            leg_score: active_set.player1_points,
            set_score: game.player_1_score,
        },
        player if player == player_2 => PlayerScores {
            round_score: leg.player2_score,
            leg_score: active_set.player2_points,
            set_score: game.player_2_score,
        },
        _ => return Err(ApiError::BadRequest(String::from("invalid player name"))),
    };

    let mut overshot = false;
    let mut leg_won = false;

    // filters out throws after overshoot, so the score is either valid or 0 i.e leg won
    let valid_points: Vec<Point> = points
        .clone()
        .iter()
        .scan(false, |has_overshot, point| {
            println!(
                "score: {}, has_overshot: {:?}",
                scores.round_score, has_overshot
            );
            if *has_overshot == true || leg_won == true {
                return None;
            }
            let new_score = check_score(scores.round_score, point.score);
            match new_score {
                Some(new_score) => {
                    scores.round_score = new_score;
                    if scores.round_score == 0 {
                        leg_won = true;
                    }
                }
                None => {
                    overshot = true;
                    *has_overshot = true;
                }
            };
            Some(point.clone())
        })
        .collect();

    let new_points: Vec<throws::ActiveModel> = valid_points
        .clone()
        .into_iter()
        .map(|p| throws::ActiveModel {
            game_id: Set(payload.game_id),
            leg_id: Set(i32::from(leg.id)),
            value: Set(p.notation.to_string()),
            ..Default::default()
        })
        .collect();

    if new_points.len() == 0 {
        return Ok(StatusCode::OK);
    }

    let mut should_create_new_leg = false;
    let mut should_create_new_set = false;
    let game_won;

    println!("new score: {}", scores.round_score);

    // works but maybe a refactor at some point
    // match the trows to the player and update the score to be updated in the transaction
    match payload.player {
        player if player == player_1 => {
            update_leg.next_player = Set(player_2.clone());
            if !overshot {
                update_leg.player1_score = Set(scores.round_score);
                if leg_won {
                    (should_create_new_set, should_create_new_leg, game_won) =
                        leg_win_score_update(&mut scores, active_set.length, game.length);
                    if game_won {
                        update_game.winner = Set(Some(game.player_1.clone()))
                    }
                    println!("after update score: {:?}", scores);
                    update_set.player1_points = Set(scores.leg_score);
                    update_game.player1_score = Set(scores.set_score);
                }
            }
        }
        player if player == player_2 => {
            update_leg.next_player = Set(player_1.clone());
            if !overshot {
                update_leg.player2_score = Set(scores.round_score);
                if leg_won {
                    (should_create_new_set, should_create_new_leg, game_won) =
                        leg_win_score_update(&mut scores, active_set.length, game.length);

                    if game_won {
                        update_game.winner = Set(Some(game.player_2.clone()))
                    }
                    println!("after update score: {:?}", scores);
                    update_game.player1_score = Set(scores.set_score);
                    update_set.player2_points = Set(scores.leg_score);
                }
            }
        }
        _ => return Err(ApiError::BadRequest(String::from("invalid player name"))),
    };

    println!(
        "running update, create_new_leg: {}, create_new_set: {}",
        should_create_new_leg, should_create_new_leg
    );

    state
        .db
        .transaction::<_, (), ApiError>(|tx| {
            Box::pin(async move {
                let mut events: Vec<String> = vec![];
                // inserts the trow and updates scores as it should always
                // be updated regardless of win, overshoot or regular throw
                throws::Entity::insert_many(new_points)
                    .exec_with_returning(tx)
                    .await?;

                // fetches the updated leg with the new throws with their id
                let update = legs::Entity::update(update_leg).exec(tx).await?;

                let updated_leg = legs::Entity::find_by_id(update.id)
                    .find_with_related(throws::Entity)
                    .all(tx)
                    .await?
                    .first()
                    .cloned()
                    .ok_or(ApiError::UnknownError("a".to_string()))?;

                let leg = Leg::from(updated_leg);

                let leg_update = GameEvent {
                    id: game.id,
                    event_type: "update".to_string(),
                    data: leg.clone(),
                };

                let point_event_serialized = serde_json::to_string(&leg_update);
                match point_event_serialized {
                    Ok(point_event_serialized) => {
                        events.push(point_event_serialized);
                    }
                    Err(_) => (),
                }

                if update_set.is_changed() {
                    let set = sets::Entity::update(update_set).exec(tx).await?;

                    let set_model = Set::from(set);

                    let set_update = GameEvent {
                        id: game.id,
                        event_type: "update".to_string(),
                        data: set_model,
                    };

                    let set_event_serialized = serde_json::to_string(&set_update);
                    match set_event_serialized {
                        Ok(set_event_serialized) => {
                            events.push(set_event_serialized);
                        }
                        Err(_) => (),
                    }
                }

                if update_game.is_changed() {
                    let game = games::Entity::update(update_game).exec(tx).await?;
                }
                // handles creating new leg or set based on the score of the player
                if should_create_new_set {
                    // negates the previous opening player for last set
                    let mut next_opening_player = player_1.clone();
                    if active_set.opening == player_1 {
                        next_opening_player = player_2.clone();
                    }

                    create_new_set_with_leg(
                        tx,
                        game.id,
                        game.mode,
                        active_set.number + 1,
                        next_opening_player.clone(),
                        active_set.length,
                    )
                    .await?;
                } else if should_create_new_leg {
                    // negates previous opening player
                    let mut next_opening_player = player_1.clone();
                    if leg.opening == player_1 {
                        next_opening_player = player_2;
                    }

                    create_new_leg(
                        tx,
                        active_set.id,
                        game.mode,
                        leg.number + 1,
                        next_opening_player.clone(),
                    )
                    .await?;
                }

                events.iter().for_each(|msg| {
                    let _ = state.sender.send(msg.to_string());
                });

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
