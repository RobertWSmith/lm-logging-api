use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, ToSchema)]
pub enum ChatRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "tool")]
    Tool,
}
