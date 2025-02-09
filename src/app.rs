use axum::routing::post;
use axum::{Error, Router};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;
use tower_http::trace::{self};
use tracing::Level;

use crate::api::game::create_game;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
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

        let app_state = AppState { db };
        let router = Router::new()
            .nest(
                "/api",
                Router::new().nest("/games", Router::new().route("/", post(create_game))),
            )
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .with_state(app_state.clone());
        Ok(Self { app_state, router })
    }
}
