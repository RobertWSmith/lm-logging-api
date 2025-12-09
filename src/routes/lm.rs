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
    let mut result = get_log_record(pool, id).await?;

    if let Some(model_provider) = payload.model_provider {
        result.model_provider = model_provider;
    }
    if let Some(model_name) = payload.model_name {
        result.model_name = model_name;
    }
    if let Some(model_version) = payload.model_version {
        result.model_version = model_version;
    }
    if let Some(app_name) = payload.app_name {
        result.app_name = app_name;
    }
    if let Some(app_project) = payload.app_project {
        result.app_project = app_project;
    }
    if let Some(app_version) = payload.app_version {
        result.app_version = app_version;
    }
    if let Some(prompt) = payload.prompt {
        result.prompt = prompt;
    }
    if let Some(response) = payload.response {
        result.response = response;
    }
    if let Some(prompt_user_id) = payload.prompt_user_id {
        result.prompt_user_id = prompt_user_id;
    }
    if let Some(prompt_app_hostname) = payload.prompt_app_hostname {
        result.prompt_app_hostname = prompt_app_hostname;
    }
    if let Some(prompt_submit_ts) = payload.prompt_submit_ts {
        result.prompt_submit_ts = prompt_submit_ts;
    }
    if let Some(response_receipt_ts) = payload.response_receipt_ts {
        result.response_receipt_ts = response_receipt_ts;
    }
    if let Some(input_tokens) = payload.input_tokens {
        result.input_tokens = input_tokens;
    }
    if let Some(output_tokens) = payload.output_tokens {
        result.output_tokens = output_tokens;
    }
    if let Some(total_tokens) = payload.total_tokens {
        result.total_tokens = total_tokens;
    }

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
    .bind(&result.model_provider)
    .bind(&result.model_name)
    .bind(&result.model_version)
    .bind(&result.app_name)
    .bind(&result.app_project)
    .bind(&result.app_version)
    .bind(serde_json::to_string(&result.prompt).expect("Failed to serialize to JSON"))
    .bind(&result.response)
    .bind(&result.prompt_user_id)
    .bind(&result.prompt_app_hostname)
    .bind(&result.prompt_submit_ts)
    .bind(&result.response_receipt_ts)
    .bind(&result.input_tokens)
    .bind(&result.output_tokens)
    .bind(&result.total_tokens)
    .bind(&id)
    .execute(pool)
    .await?;

    Ok(get_log_record(&pool, id).await?)
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
