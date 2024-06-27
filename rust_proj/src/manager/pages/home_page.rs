use crate::{
    data_context::context::AppState, manager::templates::pages::StartPageVm, models::data::ClaimsModel,
    session::claims::Claims,
};
use askama::Template;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;

pub async fn get(
    Extension(_claims): Extension<Claims<ClaimsModel>>,
    State(_app_state): State<AppState>,
) -> (StatusCode, Html<String>) {
    let editor_home = StartPageVm {
        title: "Editor Home for SC",
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}
