use super::templates::AccountPage;
use crate::{
    data::{account::get_account_details, context::AppState, references::get_languages},
    data_models::reference_items::Language,
};
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;

pub async fn get_account_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let languages = get_languages(&app_state).await;
    let response = get_account_details(&app_state).await;
    let editor_home = AccountPage {
        language: Language::get_from_int(&languages, response.languages.main_language),
        selected_languages: response.languages.languages.iter().map(|language_id| Language::get_from_int(&languages, *language_id)).collect(),
        available_languages: languages,
        title: "Editor Home for SC",
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}
