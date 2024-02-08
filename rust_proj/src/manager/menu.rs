use super::templates::DetailsPage;
use crate::{
    data::{self, context::AppState},
    data_models::BioItem,
};
use askama::Template;
use axum::{extract::State, response::Html, Form};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_menu_items(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let bio_details = data::details::get_details(&app_state).await;
    let editor_home = DetailsPage {
        title: "Editor Home for SC",
        id: bio_details.id,
        lang: bio_details.lang,
        name: bio_details.name,
        info: bio_details.info.unwrap_or("".to_string()),
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}

pub async fn post_details_home(
    State(app_state): State<AppState>,
    Form(bio_item): Form<BioItem>,
) -> (StatusCode, Html<String>) {
    let mut info: String = "Details updated successfully".to_string();
    let result = data::details::set_details(&app_state, bio_item).await;
    if !result {
        info = "Failed to updated details".to_string();
    }
    (StatusCode::OK, Html(info))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailsForm {
    blurb: String,
}
