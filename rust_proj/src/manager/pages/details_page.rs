use crate::{
    data_context::{self, context::AppState},
    manager::templates::pages::DetailsPageVm,
    models::data::{reference_items::Language, ClaimsModel}, session::claims::Claims,
};
use askama::Template;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_details_home(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let all_langs = data_context::references::get_languages(database_pool).await;
    let account_languages = crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    // let languages = account_languages.iter().map(|ac_lang_model| ac_lang_model.language).collect::<Vec<i32>>();
    let language_list = Language::vec_from_int_vec(&all_langs, &account_languages);

    let editor_home = DetailsPageVm {
        title: "Editor Home for SC",
        languages: language_list,
    };

    let editor_home: String = editor_home.render().unwrap().to_string();
    (StatusCode::OK, Html(editor_home))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailsForm {
    blurb: String,
}
