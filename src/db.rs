use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use std::sync::OnceLock;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized — call init_db first")
}

pub async fn init_db() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment");

    let db = Database::connect(&url).await?;

    db.execute(Statement::from_string(
        DbBackend::Postgres,
        r#"
        CREATE TABLE IF NOT EXISTS entries (
            id    SERIAL PRIMARY KEY,
            field1 TEXT NOT NULL,
            field2 TEXT NOT NULL
        )
        "#
        .to_string(),
    ))
    .await?;

    let _ = DB.set(db);
    Ok(())
}
