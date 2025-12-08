use sqlx::Error as SqlxError;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::{Pool, Sqlite};
use std::path::Path;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

pub async fn connect_or_create_db(filename: impl AsRef<Path>) -> Result<SqlitePool, SqlxError> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    SqlitePool::connect_with(options).await
}

pub async fn database_setup(pool: &SqlitePool) -> Result<(), SqlxError> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS log_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            model_provider VARCHAR NOT NULL,
            model_name VARCHAR NOT NULL,
            model_version VARCHAR NOT NULL,
            app_name VARCHAR NOT NULL,
            app_project VARCHAR NOT NULL,
            app_version VARCHAR NOT NULL,
            prompt JSON NOT NULL,
            response TEXT NOT NULL,
            prompt_user_id VARCHAR NOT NULL,
            prompt_app_hostname VARCHAR NOT NULL,
            prompt_submit_ts DATETIME NOT NULL,
            response_receipt_ts DATETIME NOT NULL,
            input_tokens INTEGER NOT NULL,
            output_tokens INTEGER NOT NULL,
            total_tokens INTEGER NOT NULL
        )
        "#,
    )
        .execute(pool)
        .await?;

    Ok(())
}
