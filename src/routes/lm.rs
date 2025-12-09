use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::SqlitePool;

pub mod chat_role;
pub mod log_record;
pub mod prompt;

use crate::database::AppState;
use log_record::{CreateLogRecord, ErrorMessage, LogRecord, LogRecordResponse, PatchLogRecord};

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

#[utoipa::path(
    post,
    path = "/api/v1/lm/log",
    responses(
        (status = StatusCode::CREATED, description = "Language Model Logged", body = LogRecordResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Language Model Logging Failed", body = ErrorMessage),
    )
)]
pub async fn post_log(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateLogRecord>,
) -> impl IntoResponse {
    let result = insert_create_log_record(&app_state.pool, payload).await;

    match result {
        Ok(id) => (StatusCode::CREATED, Json(LogRecordResponse { id })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: format!("Failed to insert row with error: {:#?}", e),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/lm/log/{id}",
    responses(
        (status = StatusCode::OK, description = "Language Model Log Updated", body = LogRecord),
        (status = StatusCode::NOT_FOUND, description = "Language Model Log Not Found", body = ErrorMessage),
    )
)]
pub async fn get_log(State(pool): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let result = get_log_record(&pool.pool, id).await;
    match result {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorMessage {
                message: format!("Log record with id {} not found.", id),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/lm/log/{id}",
    responses(
        (status = StatusCode::OK, description = "Language Model Log Updated", body = LogRecord),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Language Model Log Updated Failed", body = ErrorMessage),
        (status = StatusCode::NOT_FOUND, description = "Language Model Log Not Found", body = ErrorMessage),
    )
)]
pub async fn put_log(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateLogRecord>,
) -> impl IntoResponse {
    let original = get_log_record(&pool.pool, id).await;

    match original {
        Ok(_) => {
            let result = update_log_record(&pool.pool, id, payload).await;

            match result {
                Ok(_) => (
                    StatusCode::OK,
                    Json(get_log_record(&pool.pool, id).await.unwrap()),
                )
                    .into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorMessage {
                        message: format!(
                            "Failed to update record with id: {} - Error {:#?}",
                            id, e
                        ),
                    }),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorMessage {
                message: format!("Could not find record to overwrite with id: {}", id),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/lm/log/{id}",
    responses(
        (status = 200, description = "Language Model Log Updated", body = LogRecord)
    )
)]
pub async fn patch_log(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<PatchLogRecord>,
) -> impl IntoResponse {
    let original = get_log_record(&pool.pool, id).await;

    match original {
        Ok(_) => {
            let result = patch_log_record(&pool.pool, id, payload).await;

            match result {
                Ok(_) => (
                    StatusCode::OK,
                    Json(get_log_record(&pool.pool, id).await.unwrap()),
                )
                    .into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorMessage {
                        message: format!(
                            "Failed to update record with id: {} - Error {:#?}",
                            id, e
                        ),
                    }),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorMessage {
                message: format!("Could not find record to overwrite with id: {}", id),
            }),
        )
            .into_response(),
    }
}
