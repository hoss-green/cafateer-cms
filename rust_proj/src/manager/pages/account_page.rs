// use super::templates::AccountPage;
use crate::{
    data_context::context::AppState, manager::templates::AccountPage,
    models::data::reference_items::Language,
};
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;

pub async fn get_account_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let languages = crate::data_context::references::get_languages(&app_state).await;
    let account = crate::data_context::manager::profile::get(&app_state.database_pool).await;
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(&app_state, account.id).await;
    let account_languages = account_languages
        .iter()
        .map(|ac_lang_model| ac_lang_model.language)
        .collect::<Vec<i32>>();
    let editor_home = AccountPage {
        primary_language: Language::get_from_int(&languages, account.primary_language),
        user_languages: Language::vec_from_int_vec(&languages, &account_languages),
        system_languages: languages,
        title: "Editor Home for SC",
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}
