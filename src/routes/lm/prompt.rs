use super::chat_role::ChatRole;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, ToSchema, FromRow, Type)]
pub struct Prompt {
    pub role: ChatRole,
    pub content: String,
}
