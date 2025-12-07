#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Health Check OK", body=String)
    )
)]
pub async fn health() -> &'static str {
    "OK"
}
