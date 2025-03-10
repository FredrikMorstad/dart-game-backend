use std::str::FromStr;

use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::de::Error;
use serde_json::Value;
use uuid::Uuid;

use crate::{channels::Consumer, db::games::get_full_game};

fn parse_game_id_from_event(msg: &str) -> Result<Uuid, serde_json::Error> {
    let value: Value = serde_json::from_str(msg)?;

    if let Some(game_id) = value.get("game_id").and_then(|v| v.as_str()) {
        Uuid::parse_str(game_id).map_err(|_| serde_json::Error::custom("Invalid UUID format"))
    } else {
        Err(serde_json::Error::custom(
            "game_id not found or not a string",
        ))
    }
}

pub async fn game_ws(ws: WebSocketUpgrade, State(state): State<Consumer>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Consumer) {
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

    let game_serialized = serde_json::to_string(&valid_game).unwrap();

    let msg = Message::Text(Utf8Bytes::from(game_serialized));
    let _ = sender.send(msg).await;

    let mut rx = state.receiver.resubscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let event_game_id = parse_game_id_from_event(msg.as_str());
            match event_game_id {
                Ok(event_game_id) => {
                    if event_game_id != valid_game.id {
                        continue;
                    }
                    // forward updates from post
                    let _ = sender
                        .send(Message::Text(Utf8Bytes::from(msg.clone())))
                        .await;
                }
                Err(err) => {
                    println!("error parsing event: {}", err);
                }
            }
        }
    });
}
