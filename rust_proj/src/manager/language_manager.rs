use crate::{
    data::{
        context::AppState,
        manager::account::{get_account_details, set_account_details},
        references::get_languages,
    },
    manager::components::PrimaryLanguageList,
    models::data::reference_items::Language,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use http::StatusCode;
use std::str::FromStr;

pub async fn post_language(
    State(app_state): State<AppState>,
    body: String,
) -> (StatusCode, Html<String>) {
    let mut account_model = get_account_details(&app_state).await;
    let options: Vec<String> = match body.contains("&") && body.len() > 0 {
        true => body
            .split("&")
            .map(|item| item.to_string())
            .collect::<Vec<String>>(),
        false => vec![body],
    };

    let res: Vec<i32> = options
        .clone()
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
        user_selected_languages: Language::vec_from_int_vec(
            &languages,
            &account_model.languages.languages,
        ),
    };
    let page: String = primary_dropdown.render().unwrap().to_string();
    (StatusCode::OK, Html(page))
}
