use crate::{models::data::ClaimsModel, session::validate_jwt_and_get_claims};
use askama_axum::IntoResponse;
use axum::{extract::Request, middleware::Next, response::Redirect};
use http::{header::COOKIE, HeaderMap, HeaderName, HeaderValue, StatusCode};
use std::str::FromStr;

pub async fn check_auth(headers: HeaderMap, mut request: Request, next: Next) -> impl IntoResponse {
    match request.uri().path_and_query() {
        Some(uri) => {
            let path = uri.path();
            if !path.starts_with("/manager") {
                return next.run(request).await.into_response();
            }
        }
        None => return (StatusCode::UNAUTHORIZED).into_response(),
    };
    let jwt = get_jwt_from_header(&headers);
    match jwt {
        Ok(jwt) => {
            // println!("Middleware hit {}", jwt);
            match validate_jwt_and_get_claims::<ClaimsModel>(jwt) {
                Ok(cms) => {
                    request.extensions_mut().insert(cms.clone());
                    return (next.run(request).await).into_response();
                }
                Err(err) => println!("Could not validate jwt, err: {}", err),
            };
        }
        Err(err) => {
            println!("Could not decode jwt, err: {}", err);
        }
    };


    if !is_hx_request(&headers) {
        return Redirect::to("/session/login").into_response();
    }

    let response = "redirect".into_response();
    let mut red = (StatusCode::OK, response).into_response();
    let _ = red.headers_mut().append(
        HeaderName::from_str("HX-Redirect").unwrap(),
        HeaderValue::from_str("/session/login").unwrap(),
    );
    let _ = red.headers_mut().append(
        HeaderName::from_str("HX-Refresh").unwrap(),
        HeaderValue::from_str("true").unwrap(),
    );
    red
}

fn is_hx_request(headers: &HeaderMap) -> bool {
    match headers.get("HX-Request") {
        Some(hx_header) => match hx_header.to_str().unwrap_or("false").to_lowercase().as_str() {
            "true" => true,
            _ => false,
        },
        None => false,
    }
}

fn get_jwt_from_header(headers: &HeaderMap) -> Result<String, String> {
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
        None => return Err("could not find token in header".to_string()),
    };

    Ok(jwt)
}
