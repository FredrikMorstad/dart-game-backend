use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::app::AppState;

pub struct Producer {
    pub db: DatabaseConnection,
    pub sender: Sender<String>,
}

pub struct Consumer {
    pub db: DatabaseConnection,
    pub receiver: Arc<Receiver<String>>,
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(app_state: &AppState) -> DatabaseConnection {
        app_state.db.clone()
    }
}

impl FromRef<AppState> for Producer {
    fn from_ref(app_state: &AppState) -> Producer {
        Producer {
            db: app_state.db.clone(),
            sender: app_state.sender.clone(),
        }
    }
}

impl FromRef<AppState> for Consumer {
    fn from_ref(app_state: &AppState) -> Consumer {
        Consumer {
            db: app_state.db.clone(),
            receiver: app_state.receiver.clone(),
        }
    }
}
