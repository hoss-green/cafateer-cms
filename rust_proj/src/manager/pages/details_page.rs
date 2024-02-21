use crate::{
    data::{self, context::AppState},
    manager::templates::DetailsPage,
    models::data::reference_items::Language,
};
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_details_home(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let all_langs = data::references::get_languages(&app_state).await;
    let language_list = Language::vec_from_int_vec(&all_langs, &account.languages.languages);

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
