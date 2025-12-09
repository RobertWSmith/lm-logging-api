use crate::routes::lm::log_record::{CreateLogRecord, LogRecord, PatchLogRecord};
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

pub async fn get_log_record(pool: &SqlitePool, id: i64) -> Result<LogRecord, sqlx::Error> {
    sqlx::query_as("SELECT * FROM log_records WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn insert_create_log_record(
    pool: &SqlitePool,
    payload: CreateLogRecord,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
INSERT INTO log_records (
    model_provider,
    model_name,
    model_version,
    app_name,
    app_project,
    app_version,
    prompt,
    response,
    prompt_user_id,
    prompt_app_hostname,
    prompt_submit_ts,
    response_receipt_ts,
    input_tokens,
    output_tokens,
    total_tokens
)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&payload.model_provider)
    .bind(&payload.model_name)
    .bind(&payload.model_version)
    .bind(&payload.app_name)
    .bind(&payload.app_project)
    .bind(&payload.app_version)
    .bind(serde_json::to_string(&payload.prompt).expect("Failed to serialize to JSON"))
    .bind(&payload.response)
    .bind(&payload.prompt_user_id)
    .bind(&payload.prompt_app_hostname)
    .bind(&payload.prompt_submit_ts)
    .bind(&payload.response_receipt_ts)
    .bind(&payload.input_tokens)
    .bind(&payload.output_tokens)
    .bind(&payload.total_tokens)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update_log_record(
    pool: &SqlitePool,
    id: i64,
    payload: CreateLogRecord,
) -> Result<LogRecord, sqlx::Error> {
    sqlx::query(
        r#"
UPDATE log_records SET
    model_provider = ?,
    model_name = ?,
    model_version = ?,
    app_name = ?,
    app_project = ?,
    app_version = ?,
    prompt = ?,
    response = ?,
    prompt_user_id = ?,
    prompt_app_hostname = ?,
    prompt_submit_ts = ?,
    response_receipt_ts = ?,
    input_tokens = ?,
    output_tokens = ?,
    total_tokens = ?
WHERE
    id = ? "#,
    )
    .bind(&payload.model_provider)
    .bind(&payload.model_name)
    .bind(&payload.model_version)
    .bind(&payload.app_name)
    .bind(&payload.app_project)
    .bind(&payload.app_version)
    .bind(serde_json::to_string(&payload.prompt).expect("Failed to serialize to JSON"))
    .bind(&payload.response)
    .bind(&payload.prompt_user_id)
    .bind(&payload.prompt_app_hostname)
    .bind(&payload.prompt_submit_ts)
    .bind(&payload.response_receipt_ts)
    .bind(&payload.input_tokens)
    .bind(&payload.output_tokens)
    .bind(&payload.total_tokens)
    .bind(&id)
    .execute(pool)
    .await?;

    Ok(get_log_record(&pool, id).await?)
}

pub async fn patch_log_record(
    pool: &SqlitePool,
    id: i64,
    payload: PatchLogRecord,
) -> Result<LogRecord, sqlx::Error> {
    let result = get_log_record(pool, id).await?;

    let patch_row = CreateLogRecord {
        model_provider: payload
            .model_provider
            .or(Some(result.model_provider))
            .unwrap(),
        model_name: payload.model_name.or(Some(result.model_name)).unwrap(),
        model_version: payload
            .model_version
            .or(Some(result.model_version))
            .unwrap(),
        app_name: payload.app_name.or(Some(result.app_name)).unwrap(),
        app_project: payload.app_project.or(Some(result.app_project)).unwrap(),
        app_version: payload.app_version.or(Some(result.app_version)).unwrap(),
        prompt: payload.prompt.or(Some(result.prompt)).unwrap(),
        response: payload.response.or(Some(result.response)).unwrap(),
        prompt_user_id: payload
            .prompt_user_id
            .or(Some(result.prompt_user_id))
            .unwrap(),
        prompt_app_hostname: payload
            .prompt_app_hostname
            .or(Some(result.prompt_app_hostname))
            .unwrap(),
        prompt_submit_ts: payload
            .prompt_submit_ts
            .or(Some(result.prompt_submit_ts))
            .unwrap(),
        response_receipt_ts: payload
            .response_receipt_ts
            .or(Some(result.response_receipt_ts))
            .unwrap(),
        input_tokens: payload.input_tokens.or(Some(result.input_tokens)).unwrap(),
        output_tokens: payload
            .output_tokens
            .or(Some(result.output_tokens))
            .unwrap(),
        total_tokens: payload.total_tokens.or(Some(result.total_tokens)).unwrap(),
    };

    Ok(update_log_record(&pool, id, patch_row).await?)
}
