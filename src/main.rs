use std::error::Error;
use axum::{middleware, Router};
use axum::routing::{get, post};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_doc;
mod database;
mod routes;
mod custom_middleware;

use crate::routes::health;
use crate::routes::lm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openapi = api_doc::ApiDoc::openapi();
    let addr = tokio::net::TcpListener::bind("127.0.0.1:5000").await?;
    let pool = database::connect_or_create_db("rust-database.db").await?;
    database::database_setup(&pool).await?;

    println!("Listening on: {}", addr.local_addr().unwrap());

    let app = Router::new()
        .route("/api/v1/health", get(health::health))
        .route("/api/v1/lm/log", post(lm::post_log))
        .route("/api/v1/lm/log/{id}", get(lm::get_log))
        .merge(SwaggerUi::new("/swagger/").url("/api/openapi.json", openapi))
        .layer(middleware::from_fn(custom_middleware::logging_middleware))
        .with_state(database::AppState{ pool });

    axum::serve(addr, app).await?;
    Ok(())
}
