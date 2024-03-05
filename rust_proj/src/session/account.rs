use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    response::{AppendHeaders, Html},
    Form,
};
use chrono::Utc;
use http::{header::SET_COOKIE, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{data::context::AppState, session::security};

use super::{
    data,
    models::AccountModel,
    templates::{LoginPage, SignUpPage},
};

pub async fn login(State(app_state): State<AppState>) -> impl IntoResponse {
    let login_page: LoginPage = LoginPage { title: "Login" };
    let login_page: String = login_page.render().unwrap().to_string();
    (StatusCode::OK, Html(login_page))
}

pub async fn sign_up(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let sign_up_page: SignUpPage = SignUpPage { title: "Sign Up" };
    let sign_up_page: String = sign_up_page.render().unwrap().to_string();
    (StatusCode::OK, Html(sign_up_page))
}

pub async fn do_login(
    State(app_state): State<AppState>,
    Form(session_form): Form<SessionForm>,
) -> impl IntoResponse {
    println!("{:#?}", session_form);
    let login_page: LoginPage = LoginPage { title: "Login" };
    let login_page: String = login_page.render().unwrap().to_string();

    let normalised_email = session_form.email.to_uppercase();

    let user_account = match data::get_account_by_email(&app_state, &normalised_email).await {
        Some(user) => user,
        None => panic!("user not found for email {}", session_form.email),
    };
    let password_hash = security::calculate_hash(&session_form.password, &user_account.salt);

    if password_hash == user_account.password_hash {
        println!("User {} SUCCEEDED to log in", user_account.email);
        let headers: AppendHeaders<[(http::HeaderName, String); 1]> =
            AppendHeaders([(SET_COOKIE, get_cookie().await)]);
        return (headers, Html(login_page));
    }

        println!("User {} FAILED to log in", user_account.email);
    let headers: AppendHeaders<[(http::HeaderName, String); 1]> =
        AppendHeaders([(SET_COOKIE, String::new())]);
    (headers, Html(login_page))
    // (headers, Html(login_page))
}

pub async fn do_signup(
    State(app_state): State<AppState>,
    Form(session_form): Form<SessionForm>,
) -> (StatusCode, Html<String>) {
    let sign_up_page: SignUpPage = SignUpPage { title: "Sign Up" };
    println!("{:#?}", session_form);

    let creation_timestamp = Utc::now();
    let salt = security::generate_salt();
    let hash = security::calculate_hash(&session_form.password, &salt);
    let success = data::save_sign_up(
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
    .await;
    let sign_up_page: String = sign_up_page.render().unwrap().to_string();
    (StatusCode::OK, Html(sign_up_page))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionForm {
    email: String,
    password: String,
    remember: Option<bool>,
}

async fn get_cookie<'a>() -> String {
    format!("token={}; same-site=Lax, path=/;", "tooookkkeeennnn")
}
