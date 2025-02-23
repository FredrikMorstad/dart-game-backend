use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tracing_subscriber::fmt::format;
use uuid::Uuid;

use crate::{
    app::AppState,
    db::games::get_full_game,
    models::game::{Leg, Set, Throw},
};

pub async fn game_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    let mut game_id: Option<Uuid> = None;
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(id) = message {
            let id = Uuid::parse_str(id.as_str());
            match id {
                Ok(id) => {
                    game_id = Some(id);
                    break;
                }
                Err(_e) => {
                    let _ = sender
                        .send(Message::Text(Utf8Bytes::from_static("invalid id")))
                        .await;
                    return;
                }
            }
        }
    }

    let id = match game_id {
        Some(game_id) => game_id,
        None => return,
    };

    let game = get_full_game(&state.db, id).await;

    let valid_game = match game {
        Ok(game) => game,
        Err(_err) => {
            let _ = sender
                .send(Message::Text(Utf8Bytes::from_static("could not find game")))
                .await;
            return;
        }
    };

    let a = serde_json::to_string(&valid_game).unwrap();

    let msg = Message::Text(Utf8Bytes::from(a));
    let _ = sender.send(msg).await;

    println!("game id: {:?}", valid_game);

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(update) = message {
            let msg = update.as_str();
            let update_payload: Result<Throw, serde_json::Error> = serde_json::from_str(msg);
            let payload = match update_payload {
                Ok(update_payload) => update_payload,
                Err(err) => {
                    let msg = format!("error parsing payload: {}", err);
                    let _ = sender
                        .send(Message::Text(Utf8Bytes::from(msg.clone())))
                        .await;
                    continue;
                }
            };

            let active_leg: Option<Set> = valid_game
                .clone()
                .sets
                .into_iter()
                .max_by_key(|set| set.number);

            let set = match active_leg {
                Some(active_set) => active_set,
                None => {
                    let _ = sender
                        .send(Message::Text(Utf8Bytes::from_static("error finding set")))
                        .await;
                    continue;
                }
            };

            let active_leg: Option<Leg> = set.clone().legs.into_iter().max_by_key(|leg| leg.number);

            let leg = match active_leg {
                Some(active_leg) => active_leg,
                None => {
                    let _ = sender
                        .send(Message::Text(Utf8Bytes::from_static("error finding leg")))
                        .await;
                    continue;
                }
            };

            let last_throw = leg.throws.last();
            let throw = match last_throw {
                Some(last_throw) => last_throw,
                None => {
                    let _ = sender
                        .send(Message::Text(Utf8Bytes::from_static("error finding trow")))
                        .await;
                    continue;
                }
            };
            if throw.id != payload.id {
                let _ = sender
                    .send(Message::Text(Utf8Bytes::from_static("error finding trow")))
                    .await;
                continue;
            }
        }
    }
}
