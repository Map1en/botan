use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub static GLOBAL_DB_CONNECTION: LazyLock<RwLock<Option<DatabaseConnection>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn init_database() -> Result<(), DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./botan.db?mode=rwc".to_string());

    log::info!("Connecting to database: {}", database_url);

    let db = Database::connect(&database_url).await?;

    {
        let mut global_db = GLOBAL_DB_CONNECTION.write().await;
        *global_db = Some(db);
    }

    log::info!("Database connection established");
    Ok(())
}

pub async fn get_db_connection() -> Option<DatabaseConnection> {
    let db_guard = GLOBAL_DB_CONNECTION.read().await;
    db_guard.clone()
}
