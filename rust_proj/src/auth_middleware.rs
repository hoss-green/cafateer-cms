use crate::{models::data::ClaimsModel, session::validate_jwt_and_get_claims};
use askama_axum::{IntoResponse, Response};
use axum::{extract::Request, middleware::Next, response::Redirect};
use http::{header::COOKIE, HeaderMap, StatusCode};

pub async fn check_auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match request.uri().path_and_query() {
        Some(uri) => {
            let path = uri.path();
            // println!("{}", path);
            if !path.starts_with("/manager") {
                return Ok(next.run(request).await);
            }
        }
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let jwt = get_jwt_from_header(headers);
    match jwt {
        Ok(jwt) => {
            // println!("Middleware hit {}", jwt);
            match validate_jwt_and_get_claims::<ClaimsModel>(jwt) {
                Ok(cms) => {
                    request.extensions_mut().insert(cms.clone());
                    return Ok(next.run(request).await);
                }
                Err(err) => println!("Could not validate jwt, err: {}", err),
            };
        }
        Err(err) => {
            println!("Could not decode jwt, err: {}", err);
        }
    };

    Ok(Redirect::to("/session/login").into_response())
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
        None => return Err("could not find token in header".to_string()),
    };

    Ok(jwt)
}
