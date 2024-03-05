use askama_axum::Response;
use axum::{extract::Request, middleware::Next};
use http::{HeaderMap, StatusCode};

use crate::data::context::AppState;

pub async fn check_auth(
    // app_state: AppState,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("Middleware hit");
    Ok(next.run(request).await)
}
