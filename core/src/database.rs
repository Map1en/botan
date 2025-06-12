use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub static GLOBAL_DB_CONNECTION: LazyLock<RwLock<Option<DatabaseConnection>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn init_database() -> Result<(), DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| -> Result<String, std::env::VarError> {
            let db_host =
                std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
            let db_port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
            let db_name = std::env::var("POSTGRES_DB").unwrap_or_else(|_| "botan".to_string());
            let db_user =
                std::env::var("POSTGRES_USER").unwrap_or_else(|_| "botan_user".to_string());
            let db_password =
                std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "botan_password".to_string());

            Ok(format!(
                "postgresql://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            ))
        })
        .unwrap_or_else(|_| {
            log::warn!("No database configuration found, using SQLite as fallback");
            // rm
            "sqlite://./botan.db?mode=rwc".to_string()
        });

    log::info!("Connecting to database: {}", &database_url);

    let db = Database::connect(&database_url).await?;

    match db.ping().await {
        Ok(_) => log::info!("Database connection established and tested"),
        Err(e) => {
            log::error!("Database connection test failed: {}", e);
            return Err(e);
        }
    }

    Migrator::up(&db, None).await?;

    {
        let mut global_db = GLOBAL_DB_CONNECTION.write().await;
        *global_db = Some(db);
    }

    Ok(())
}

pub async fn get_db_connection() -> Option<DatabaseConnection> {
    let db_guard = GLOBAL_DB_CONNECTION.read().await;
    db_guard.clone()
}
