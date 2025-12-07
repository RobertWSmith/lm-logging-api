use super::prompt::Prompt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;

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
    pub total_tokens: i64,
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
        prompt_submit_ts: String,
        response_receipt_ts: String,
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

    pub fn new_error(id: i64) -> Self {
        Self::new(
            id,
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            vec![],
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            -1,
            -1,
            -2,
        )
    }
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
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, ToSchema, FromRow, Type)]
pub struct LogRecordResponse {
    pub id: i64,
}
