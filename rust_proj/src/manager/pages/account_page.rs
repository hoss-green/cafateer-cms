use crate::{
    data_context::context::AppState, manager::templates::AccountPage,
    models::data::reference_items::Language, session::claims::Claims,
};
use askama::Template;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;

pub async fn get_account_page(
    Extension(claims): Extension<Claims>,
    State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let languages = crate::data_context::references::get_languages(database_pool).await;
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    let account_languages = account_languages
        .iter()
        .map(|ac_lang_model| ac_lang_model.language)
        .collect::<Vec<i32>>();
    let editor_home = AccountPage {
        primary_language: Language::get_from_int(&languages, claims.language),
        user_languages: Language::vec_from_int_vec(&languages, &account_languages),
        system_languages: languages,
        title: "Editor Home for SC",
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}
