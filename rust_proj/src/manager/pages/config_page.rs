use crate::{
    data_context::context::AppState,
    manager::templates::pages::ConfigPageVm,
    manager::templates::view_models::AccountLanguageVm,
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
    let account_home: String = ConfigPageVm {
        primary_language: Language::get_from_int(&languages, claims.body.lang),
        user_languages: account_languages
            .iter()
            .map(|lm| AccountLanguageVm {
                id: lm.id,
                title: Language::get_from_int(&languages, lm.language).name,
                code: lm.language,
                published: lm.published,
            })
            .collect::<Vec<AccountLanguageVm>>(),
        title: "Config"
    }
    // let editor_home: String = editor_home
    .render()
    .unwrap()
    .to_string();

    (StatusCode::OK, Html(account_home)).into_response()
}
