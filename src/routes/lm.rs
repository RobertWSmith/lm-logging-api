use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

pub mod chat_role;
pub mod log_record;
pub mod prompt;

use log_record::{CreateLogRecord, LogRecord, LogRecordResponse};

#[utoipa::path(
    post,
    path = "/api/v1/lm/log",
    responses(
        (status = 200, description = "Language Model Logged", body = LogRecordResponse)
    )
)]
pub async fn post_log(
    State(pool): State<crate::database::AppState>,
    Json(payload): Json<CreateLogRecord>,
) -> (StatusCode, Json<LogRecordResponse>) {
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
    .bind(payload.model_provider)
    .bind(payload.model_name)
    .bind(payload.model_version)
    .bind(payload.app_name)
    .bind(payload.app_project)
    .bind(payload.app_version)
    .bind(serde_json::to_string(&payload.prompt).expect("Failed to serialize to JSON"))
    .bind(payload.response)
    .bind(payload.prompt_user_id)
    .bind(payload.prompt_app_hostname)
    .bind(payload.prompt_submit_ts)
    .bind(payload.response_receipt_ts)
    .bind(payload.input_tokens)
    .bind(payload.output_tokens)
    .bind(payload.total_tokens)
    .execute(&pool.pool)
    .await;

    match result {
        Ok(result) => (
            StatusCode::CREATED,
            Json(LogRecordResponse {
                id: result.last_insert_rowid(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LogRecordResponse { id: -1 }),
        ),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/lm/log/{id}",
    responses(
        (status = 200, description = "Language Model Log Retrieved", body = LogRecord)
    )
)]
pub async fn get_log(
    State(pool): State<crate::database::AppState>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<LogRecord>) {
    match sqlx::query_as("SELECT * FROM log_records WHERE id = ?")
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(result)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LogRecord::new_error(id)),
        ),
    }
}
