use utoipa::OpenApi;

use crate::routes::health;

#[derive(OpenApi)]
#[openapi(paths(health::health))]
pub struct ApiDoc;
