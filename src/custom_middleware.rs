use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    println!("Received request: {}", req.uri());
    next.run(req).await
}
