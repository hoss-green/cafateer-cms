use askama_axum::Response;
use axum::{extract::Request, middleware::Next};
use http::{header::COOKIE, HeaderMap, HeaderValue, StatusCode};

use crate::data::context::AppState;

pub async fn check_auth(
    // app_state: AppState,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let jwt = get_jwt_from_header(headers);
    match jwt {
        Ok(_) => {
            println!(
                "Middleware hit {}",
                jwt.unwrap_or("Unauthorised".to_string())
            );
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

fn get_jwt_from_header(headers: HeaderMap) -> Result<String, String> {
    let cookie_headers = match headers.get(COOKIE) {
        Some(cookie_header) => cookie_header,
        None => return Err("Could not find header for cookie".to_string()),
    };

    let cookies = match cookie_headers.to_str() {
        Ok(cookie_string) => cookie_string,
        Err(_) => return Err("could not parse cookie header".to_string()),
    };

    let mut token_cookie: Option<String> = None;

    cookies.split(";").into_iter().for_each(|item| {
        if item.contains("token") {
            token_cookie = Some(item.split("=").collect::<Vec<&str>>()[1].to_string())
        }
    });

    let jwt = match token_cookie {
        Some(jwt) => jwt,
        None => return Err("could not parse jwt cookie".to_string()),
    };

    Ok(jwt)
}
