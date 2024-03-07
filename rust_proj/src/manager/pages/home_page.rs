use crate::{data_context::context::AppState, manager::templates::StartPage, session::claims::Claims};
use askama::Template;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;

pub async fn get_home_page(
    Extension(claims): Extension<Claims>,
    State(_app_state): State<AppState>) -> (StatusCode, Html<String>) {

    println!("{:#?}", claims);

    let editor_home = StartPage {
        title: "Editor Home for SC",
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}
