use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub mod chat_role;
pub mod log_record;
pub mod prompt;

use crate::database::{
    AppState, get_log_record, insert_create_log_record, patch_log_record, update_log_record,
};
use log_record::{CreateLogRecord, ErrorMessage, LogRecord, LogRecordResponse, PatchLogRecord};

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
    match get_log_record(&pool.pool, id).await {
        Ok(_) => match update_log_record(&pool.pool, id, payload).await {
            Ok(result) => (StatusCode::OK, Json(result)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    message: format!("Failed to update record with id: {} - Error {:#?}", id, e),
                }),
            )
                .into_response(),
        },
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
