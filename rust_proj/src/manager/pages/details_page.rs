use crate::{
    data_context::{self, context::AppState},
    manager::templates::DetailsPage,
    models::data::reference_items::Language,
};
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_details_home(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let profile = data_context::manager::profile::get(&app_state.database_pool).await;
    let all_langs = data_context::references::get_languages(&app_state).await;
    let account_languages = crate::data_context::manager::profile_languages::get_all(&app_state, profile.id).await;
    let languages = account_languages.iter().map(|ac_lang_model| ac_lang_model.language).collect::<Vec<i32>>();
    let language_list = Language::vec_from_int_vec(&all_langs, &languages);

    let editor_home = DetailsPage {
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
