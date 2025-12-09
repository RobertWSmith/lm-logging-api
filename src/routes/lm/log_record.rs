use super::prompt::Prompt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, ToSchema, FromRow, Type)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, ToSchema, FromRow, Type)]
pub struct CreateLogRecord {
    pub model_provider: String,
    pub model_name: String,
    pub model_version: String,
    pub app_name: String,
    pub app_project: String,
    pub app_version: String,
    #[sqlx(json)]
    pub prompt: Vec<Prompt>,
    pub response: String,
    pub prompt_user_id: String,
    pub prompt_app_hostname: String,
    #[serde(with = "time::serde::iso8601")]
    pub prompt_submit_ts: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub response_receipt_ts: OffsetDateTime,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, ToSchema, FromRow, Type)]
pub struct LogRecordResponse {
    pub id: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, ToSchema, FromRow, Type)]
pub struct LogRecord {
    pub id: i64,
    pub model_provider: String,
    pub model_name: String,
    pub model_version: String,
    pub app_name: String,
    pub app_project: String,
    pub app_version: String,
    #[sqlx(json)]
    pub prompt: Vec<Prompt>,
    pub response: String,
    pub prompt_user_id: String,
    pub prompt_app_hostname: String,
    #[serde(with = "time::serde::iso8601")]
    pub prompt_submit_ts: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub response_receipt_ts: OffsetDateTime,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, ToSchema, FromRow, Type)]
pub struct PatchLogRecord {
    pub model_provider: Option<String>,
    pub model_name: Option<String>,
    pub model_version: Option<String>,
    pub app_name: Option<String>,
    pub app_project: Option<String>,
    pub app_version: Option<String>,
    #[sqlx(json)]
    pub prompt: Option<Vec<Prompt>>,
    pub response: Option<String>,
    pub prompt_user_id: Option<String>,
    pub prompt_app_hostname: Option<String>,
    #[serde(with = "time::serde::iso8601::option")]
    pub prompt_submit_ts: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    pub response_receipt_ts: Option<OffsetDateTime>,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub total_tokens: Option<i64>,
}

impl LogRecord {
    pub fn new(
        id: i64,
        model_provider: String,
        model_name: String,
        model_version: String,
        app_name: String,
        app_project: String,
        app_version: String,
        prompt: Vec<Prompt>,
        response: String,
        prompt_user_id: String,
        prompt_app_hostname: String,
        prompt_submit_ts: OffsetDateTime,
        response_receipt_ts: OffsetDateTime,
        input_tokens: i64,
        output_tokens: i64,
        total_tokens: i64,
    ) -> Self {
        Self {
            id,
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
            total_tokens,
        }
    }
}
