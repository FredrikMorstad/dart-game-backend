use std::sync::Arc;

use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::routing::{any, get, post};
use axum::{Error, Router};
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::trace::{self};
use tracing::Level;

use crate::api::game::{create_game, get_game};
use crate::api::games::play::post_throw;
use crate::api::ws::game::game_ws;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub sender: Sender<String>,
    pub receiver: Arc<Receiver<String>>,
}

pub struct App {
    pub app_state: AppState,
    pub router: Router,
}

impl App {
    pub fn new(db: DatabaseConnection) -> Result<Self, Error> {
        tracing_subscriber::fmt()
            .with_target(false)
            .compact()
            .init();

        let cors_layer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT])
            .allow_credentials(true)
            .allow_headers([CONTENT_TYPE])
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());

        let (sender, recv) = broadcast::channel::<String>(16);

        let app_state = AppState {
            db,
            sender,
            receiver: Arc::new(recv),
        };
        let router = Router::new()
            .nest(
                "/api",
                Router::new().nest(
                    "/games",
                    Router::new()
                        .route("/", post(create_game))
                        .route("/{id}", get(get_game))
                        .route("/throw", post(post_throw))
                        .nest("/ws", Router::new().route("/", any(game_ws))),
                ),
            )
            .layer(cors_layer)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .with_state(app_state.clone());
        Ok(Self { app_state, router })
    }
}
