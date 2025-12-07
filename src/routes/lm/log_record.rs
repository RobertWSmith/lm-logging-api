use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;
use super::prompt::Prompt;

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
    pub prompt_submit_ts: String,
    pub response_receipt_ts: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64
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
    pub prompt_submit_ts: String,
    pub response_receipt_ts: String,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, ToSchema, FromRow, Type)]
pub struct LogRecordResponse {
    pub id: i64,
}
