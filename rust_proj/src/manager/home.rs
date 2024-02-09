use super::templates::StartPage;
use crate::data::context::AppState;
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;

pub async fn get_manager_home(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let editor_home = StartPage {
        title: "Editor Home for SC",
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}
