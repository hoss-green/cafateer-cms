use crate::{
    data_context::context::AppState,
    manager::templates::{pages::ConfigPageVm, view_models::PrimaryLanguageVm},
    models::data::{reference_items::Language, ClaimsModel, ProfileLanguagesModel},
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
    let account_languages: Vec<ProfileLanguagesModel> =
        crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub)
            .await
            .into_iter()
            .filter(|acl| acl.published)
            .collect::<Vec<ProfileLanguagesModel>>();
    let profile = crate::data_context::manager::profile::get(database_pool, &claims.sub).await;
    let primary_lang = Language::get_from_int(&languages, profile.primary_language);
    let account_home: String = ConfigPageVm {
        primary_language: primary_lang.clone(),
        user_languages: account_languages
            .iter()
            .map(|lm| PrimaryLanguageVm {
                id: lm.id,
                title: Language::get_from_int(&languages, lm.language).name,
                code: lm.language,
                published: lm.published,
                primary: lm.language == primary_lang.id,
            })
            .collect::<Vec<PrimaryLanguageVm>>(),
        title: "Config",
    }
    .render()
    .unwrap()
    .to_string();

    (StatusCode::OK, Html(account_home)).into_response()
}
