use super::prompt::Prompt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use time::PrimitiveDateTime;
use time::macros::format_description;
use utoipa::ToSchema;

const FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]");

pub mod iso8601 {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(value: &PrimitiveDateTime, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let sdt = value.format(FORMAT).map_err(serde::ser::Error::custom)?;
        s.serialize_str(&sdt)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = String::deserialize(d)?;
        let dt = PrimitiveDateTime::parse(&opt, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }

    pub mod option {
        use super::*;
        use serde::{Deserializer, Serializer};

        pub fn serialize<S>(value: &Option<PrimitiveDateTime>, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(dt) => {
                    let sdt = dt.format(FORMAT).map_err(serde::ser::Error::custom)?;
                    s.serialize_str(&sdt)
                }
                None => s.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(d: D) -> Result<Option<PrimitiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let opt = Option::<String>::deserialize(d)?;
            match opt {
                Some(s) => {
                    let dt =
                        PrimitiveDateTime::parse(&s, FORMAT).map_err(serde::de::Error::custom)?;
                    Ok(Some(dt))
                }
                None => Ok(None),
            }
        }
    }
}

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
    #[serde(with = "iso8601")]
    pub prompt_submit_ts: PrimitiveDateTime,
    #[serde(with = "iso8601")]
    pub response_receipt_ts: PrimitiveDateTime,
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
    #[serde(with = "iso8601")]
    pub prompt_submit_ts: PrimitiveDateTime,
    #[serde(with = "iso8601")]
    pub response_receipt_ts: PrimitiveDateTime,
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
    #[serde(with = "iso8601::option")]
    pub prompt_submit_ts: Option<PrimitiveDateTime>,
    #[serde(with = "iso8601::option")]
    pub response_receipt_ts: Option<PrimitiveDateTime>,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub total_tokens: Option<i64>,
}
