use sea_orm::{Database, DbConn, DbErr};

pub async fn connect_to_db() -> Result<DbConn, DbErr> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    // let database_url = format!("postgres://local_user:local_dev@dart_db:5432/dart");
    println!("Database url: {database_url}");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    Ok(db)
}
