use super::{
    data,
    models::AccountModel,
    templates::{LoginPage, SignUpPage, SignUpSuccessPage},
};
use crate::{
    data_context::context::AppState,
    session::{security, tokens::account_to_jwt},
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    response::{AppendHeaders, Html, Redirect},
    Form,
};
use chrono::Utc;
use http::{header::SET_COOKIE, HeaderValue, StatusCode};
use serde::{Deserialize, Serialize};

pub async fn login() -> impl IntoResponse {
    let login_page: LoginPage = LoginPage {
        title: "Login",
        email: None,
        message: None,
    };
    let login_page: String = login_page.render().unwrap().to_string();
    (StatusCode::OK, Html(login_page))
}

pub async fn sign_up() -> (StatusCode, Html<String>) {
    let sign_up_page: SignUpPage = SignUpPage {
        title: "Sign Up",
        email: None,
        message: None,
    };
    let sign_up_page: String = sign_up_page.render().unwrap().to_string();
    (StatusCode::OK, Html(sign_up_page))
}
pub async fn sign_up_success() -> (StatusCode, Html<String>) {
    let sign_up_success_page: SignUpSuccessPage = SignUpSuccessPage {
        title: "Sign Up Success",
    };
    let sign_up_page: String = sign_up_success_page.render().unwrap().to_string();
    (StatusCode::OK, Html(sign_up_page))
}

pub async fn do_login(
    State(app_state): State<AppState>,
    Form(session_form): Form<SessionForm>,
) -> impl IntoResponse {
    println!("{:#?}", session_form);

    let normalised_email = session_form.email.to_uppercase();

    let user_account = match data::get_account_by_email(&app_state, &normalised_email).await {
        Some(user) => user,
        None => panic!("user not found for email {}", &session_form.email),
    };
    let password_hash = security::calculate_hash(&session_form.password, &user_account.salt);

    if password_hash == user_account.password_hash {
        println!("User {} SUCCEEDED to log in", user_account.email);
        match HeaderValue::from_str(&get_cookie(&user_account).await) {
            Ok(header_val) => {
                let mut redirect = Redirect::to("/manager").into_response();
                redirect.headers_mut().insert(SET_COOKIE, header_val);
                return redirect;
            }
            Err(_) => println!("could not parse header"),
        };
    }

    let login_page: LoginPage = LoginPage {
        title: "Login",
        email: Some(session_form.email.as_str()),
        message: Some("Username or Password Error"),
    };
    let login_page: String = login_page.render().unwrap().to_string();
    println!("User {} FAILED to log in", user_account.email);
    let headers: AppendHeaders<[(http::HeaderName, String); 1]> =
        AppendHeaders([(SET_COOKIE, String::new())]);
    (headers, Html(login_page)).into_response()
}

pub async fn do_signup(
    State(app_state): State<AppState>,
    Form(session_form): Form<SessionForm>,
) -> impl IntoResponse {
    let creation_timestamp = Utc::now();
    let salt = security::generate_salt();
    let hash = security::calculate_hash(&session_form.password, &salt);
    match data::save_sign_up(
        &app_state,
        &AccountModel {
            id: uuid::Uuid::new_v4(),
            email: session_form.email.clone(),
            email_normalised: session_form.email.to_uppercase(),
            password_hash: hash,
            sign_up: creation_timestamp.naive_utc(),
            salt,
            status: 0,
        },
    )
    .await
    {
        true => Redirect::to("/session/sign_up_success").into_response(),
        false => {
            let sign_up_page: SignUpPage = SignUpPage {
                title: "Sign Up",
                email: Some(session_form.email.as_str()),
                message: Some("Email Taken"),
            };
            let sign_up_page: String = sign_up_page.render().unwrap().to_string();
            (StatusCode::OK, Html(sign_up_page)).into_response()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionForm {
    email: String,
    password: String,
    remember: Option<bool>,
}

async fn get_cookie<'a>(account: &AccountModel) -> String {
    let cookie_string = account_to_jwt(&account);
    format!("token={}; same-site=Lax; path=/;", cookie_string)
}
