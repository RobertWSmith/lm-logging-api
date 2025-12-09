use utoipa::OpenApi;

use crate::routes::health;
use crate::routes::lm;

#[derive(OpenApi)]
#[openapi(paths(health::health, lm::post_log, lm::get_log, lm::put_log, lm::patch_log))]
pub struct ApiDoc;
