use dart_game_backend::{app::App, db_utils::connect_to_db};
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5050").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let db = connect_to_db().await.unwrap();
    let app = App::new(db).unwrap();

    axum::serve(listener, app.router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
