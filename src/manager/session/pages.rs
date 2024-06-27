use crate::{
    data_context::{context::AppState, manager::profile},
    manager::session::create_cookie_header,
    session::{models::UserAccountModel, security},
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    response::{AppendHeaders, Html, Redirect},
    Form,
};
use chrono::Utc;
use http::{header::SET_COOKIE, StatusCode};
use serde::{Deserialize, Serialize};

use super::templates::{LoginPage, SignUpPage, SignUpSuccessPage};

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
        reg_code: None,
    };
    let sign_up_page: String = sign_up_page.render().unwrap().to_string();
    (StatusCode::OK, Html(sign_up_page))
}
pub async fn sign_up_success() -> (StatusCode, Html<String>) {
    let sign_up_success_page: String = SignUpSuccessPage {
        title: "Sign Up Success",
    }
        .render().unwrap().to_string();
    (StatusCode::OK, Html(sign_up_success_page))
}

pub async fn do_login(
    State(app_state): State<AppState>,
    Form(session_form): Form<SessionForm>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let normalised_email = session_form.email.to_uppercase();
    let user_account = match crate::session::data::get_account_by_email(
        &app_state.database_pool,
        &normalised_email,
    )
    .await
    {
        Some(user) => user,
        None => panic!("user not found for email {}", &session_form.email),
    };

    let password_hash = security::calculate_hash(&session_form.password, &user_account.salt);

    if password_hash == user_account.password_hash {
        let profile = profile::get(database_pool, &user_account.id).await;
        println!("User {} SUCCEEDED to log in", user_account.email);
        let mut redirect = Redirect::to("/manager").into_response();
        let header_val = create_cookie_header(&user_account, &profile);
        redirect.headers_mut().insert(SET_COOKIE, header_val);
        return redirect;
    }
    println!("User {} FAILED to log in", user_account.email);
    let login_page: String = LoginPage {
        title: "Login",
        email: Some(session_form.email.as_str()),
        message: Some("Username or Password Error"),
    }
    .render()
    .unwrap()
    .to_string();

    let headers: AppendHeaders<[(http::HeaderName, String); 1]> =
        AppendHeaders([(SET_COOKIE, String::new())]);
    (headers, Html(login_page)).into_response()
}

pub async fn do_signup(
    State(app_state): State<AppState>,
    Form(session_form): Form<SessionForm>,
) -> impl IntoResponse {

    let reg_code = &session_form.reg_code.unwrap_or("".to_string());
    if reg_code != "Canary24Food" {
        let sign_up_page: SignUpPage = SignUpPage {
            title: "Sign Up",
            email: Some(session_form.email.as_str()),
            message: Some("Invalid Registration Code"),
            reg_code: Some(reg_code),
        };
        let sign_up_page: String = sign_up_page.render().unwrap().to_string();
        return (StatusCode::OK, Html(sign_up_page)).into_response();
    };

    let creation_timestamp = Utc::now();
    let salt = security::generate_salt();
    let hash = security::calculate_hash(&session_form.password, &salt);
    match crate::session::data::save_sign_up(
        &app_state.database_pool,
        &UserAccountModel {
            id: match app_state.single_user_id {
                Some(id) => id,
                None => uuid::Uuid::new_v4(),
            },
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
                reg_code: None,
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
    reg_code: Option<String>,
}
