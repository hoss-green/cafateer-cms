use crate::{
    data_context::context::AppState,
    manager::templates::pages::LanguagesPageVm,
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
        user_languages: Language::vec_from_int_vec(&languages, &account_languages),
        system_languages: languages,
        title: "Languages Editor",
    }
    .render()
    .unwrap()
    .to_string();

    (StatusCode::OK, Html(languages_page)).into_response()
}
