// use super::templates::AccountPage;
use crate::{
    data::{context::AppState, manager::account::get_account_details, references::get_languages},
    manager::templates::AccountPage,
    models::data::reference_items::Language,
};
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;

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
