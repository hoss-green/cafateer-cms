use crate::{
    data_context::{self, context::AppState}, manager::templates::{components::ComponentDetailEditorVm, pages::DetailsPageVm}, models::data::{reference_items::Language, ClaimsModel, DetailsModel}, session::claims::Claims
};
use askama::Template;
use axum::{
    extract::{Path, State}, response::Html, Extension, Form
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_details_data(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let account_languages = data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    if !account_languages
        // .iter().map(|al| al.language).collect::<Vec<i32>>()
        .iter()
        .any(|&lang_id| lang_id == id)
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Language not found".to_string()),
        );
    }

    let mut detail = data_context::manager::details::get_detail(database_pool, &claims.sub, id).await;
    if detail.lang_name.is_empty() {
        let all_langs = data_context::references::get_languages(database_pool).await;
        let current_language = Language::get_from_int(&all_langs, id);
        detail.lang = id;
        detail.lang_code = current_language.code;
        detail.lang_name = current_language.name;
    }
    let component_edit_details = ComponentDetailEditorVm {
        id: detail.id,
        lang: detail.lang,
        lang_name: detail.lang_name,
        blurb: detail.blurb.clone().unwrap_or("".to_string()),
    };

    let component_editor: String = component_edit_details.render().unwrap().to_string();
    (StatusCode::OK, Html(component_editor))
}

pub async fn get_details_home(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let account_languages = data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    let all_langs = data_context::references::get_languages(database_pool).await;
    let language_list = Language::vec_from_int_vec(&all_langs, &account_languages);
        // .iter().map(|ml| ml.language).collect::<Vec<i32>>());

    let editor_home = DetailsPageVm {
        title: "Editor Home for SC",
        languages: language_list,
    };

    let editor_home: String = editor_home.render().unwrap().to_string();
    (StatusCode::OK, Html(editor_home))
}

pub async fn post_details_home(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Form(details_item): Form<DetailsModel>,
) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    match data_context::manager::details::set_details(database_pool, &claims.sub, details_item).await {
        true => (StatusCode::OK, Html(String::from("Saved"))),
        false => (StatusCode::OK, Html(String::from("Error")))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailsForm {
    blurb: String,
}
