use super::templates::DetailsPage;
use crate::{
    data::{self, context::AppState},
    data_models::DetailsModel,
    view_models::components::DetailsViewModel,
};
use askama::Template;
use axum::{extract::State, response::Html, Form};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_details_home(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let details_list = data::details::get_details_list(&app_state).await;
    let editor_home = DetailsPage {
        title: "Editor Home for SC",
        details: details_list
            .iter()
            .map(|item| DetailsViewModel {
                id: item.id,
                lang: item.lang,
                blurb: item.blurb.clone().unwrap_or("".to_string()),
                code: item.code.clone(),
            })
            .collect(),
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}

pub async fn post_details_home(
    State(app_state): State<AppState>,
    Form(details_item): Form<DetailsModel>,
) -> (StatusCode, Html<String>) {
    let mut info: String = "Details updated successfully".to_string();
    let result = data::details::set_details(&app_state, details_item).await;
    if !result {
        info = "Failed to updated details".to_string();
    }
    (StatusCode::OK, Html(info))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailsForm {
    blurb: String,
}
