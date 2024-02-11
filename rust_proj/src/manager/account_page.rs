use super::templates::AccountPage;
use crate::{
    data::{
        account::{get_account_details, set_account_details},
        context::AppState,
        references::get_languages,
    },
    data_models::reference_items::Language, manager::components::PrimaryLanguageList,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use http::StatusCode;
use std::str::FromStr;

pub async fn get_account_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let languages = get_languages(&app_state).await;
    let response = get_account_details(&app_state).await;
    let editor_home = AccountPage {
        primary_language: Language::get_from_int(&languages, response.languages.main_language),
        user_languages: Language::vec_from_int_vec(&languages, &response.languages.languages),
        system_languages: languages,
        title: "Editor Home for SC",
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}

pub async fn post_language(
    State(app_state): State<AppState>,
    body: String,
) -> (StatusCode, Html<String>) {
    let mut account_model = get_account_details(&app_state).await;
    let message = format!("Details updated successfully with body: {}", body.clone());
    println!("{}", message);
    let res: Vec<i32> = body
        .split("&")
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|item| FromStr::from_str(item.split("=").next().unwrap()).unwrap_or(0))
        .collect();

    if res
        .clone()
        .iter()
        .any(|&item| item == account_model.languages.main_language)
    {

    } else {
        account_model.languages.main_language = res[0];
    }

    account_model.languages.languages = res.clone();
    set_account_details(&app_state, &account_model).await;
    let languages = get_languages(&app_state).await;
    let primary_dropdown = PrimaryLanguageList {
        primary_language_id: account_model.languages.main_language,
        user_selected_languages: Language::vec_from_int_vec(&languages, &res),
    };
    let page: String = primary_dropdown.render().unwrap().to_string();
    (StatusCode::OK, Html(page))
}

pub async fn post_primary_language(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Html<String>) {
    let languages = get_languages(&app_state).await;
    let mut account_model = get_account_details(&app_state).await;
    account_model.languages.main_language = id;
    set_account_details(&app_state, &account_model).await;
    let primary_dropdown = PrimaryLanguageList {
        primary_language_id: account_model.languages.main_language,
        user_selected_languages: Language::vec_from_int_vec(&languages, &account_model.languages.languages),
    };
    let page: String = primary_dropdown.render().unwrap().to_string();
    (StatusCode::OK, Html(page))
}
