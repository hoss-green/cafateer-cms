use crate::{
    data_context::context::AppState,
    manager::templates::{
        pages::LanguagesPageVm, view_models::SelectableLanguageVm,
    },
    models::data::{reference_items::Language, ClaimsModel},
    session::claims::Claims,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;

pub async fn get(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let languages = crate::data_context::references::get_languages(database_pool).await;
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    let languages_page: String = LanguagesPageVm {
        primary_language: Language::get_from_int(&languages, claims.body.lang),
        title: "Languages Editor",
        languages: languages
            .iter()
            .map(|lang| SelectableLanguageVm {
                lang_id: lang.id,
                title: lang.name.clone(),
                code: lang.code.clone(),
                user_lang_id: match &account_languages.iter().find(|acl| acl.language == lang.id) {
                    Some(value) => Some(value.id),
                    None => None,
                },
                published: match &account_languages.iter().find(|acl| acl.language == lang.id) {
                    Some(value) => value.published,
                    None => false,
                },
            })
            .collect::<Vec<SelectableLanguageVm>>()
    }
    .render()
    .unwrap()
    .to_string();

    (StatusCode::OK, Html(languages_page)).into_response()
}
