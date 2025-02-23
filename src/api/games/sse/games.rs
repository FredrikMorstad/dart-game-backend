use axum::{
    extract::Path,
    response::sse::{Event, Sse},
};
use axum_extra::{headers, TypedHeader};
use futures_util::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;
use uuid::Uuid;

pub enum EventType {
    Insert,
    Update,
}

pub struct GameEvent<T> {
    pub even_type: EventType,
    pub data: T,
}

pub async fn sse_game_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    Path(id): Path<Uuid>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("id: {}", id);
    println!("`{}` connected", user_agent.as_str());

    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
