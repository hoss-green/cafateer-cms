use crate::{
    data_context::context::AppState,
    manager::templates::pages::AccountPageVm,
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
        crate::data_context::manager::profile_languages::get_all_ids(database_pool, &claims.sub).await;
    // let account_languages = account_languages
    //     .iter()
    //     .map(|ac_lang_model| ac_lang_model.language)
    //     .collect::<Vec<i32>>();
    let account_home: String = AccountPageVm {
        primary_language: Language::get_from_int(&languages, claims.body.lang),
        user_languages: Language::vec_from_int_vec(&languages, &account_languages),
        system_languages: languages,
        title: "Editor Home for SC",
    }
    // let editor_home: String = editor_home
    .render()
    .unwrap()
    .to_string();

    (StatusCode::OK, Html(account_home)).into_response()
}
