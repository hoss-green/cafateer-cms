use super::templates::DetailsPage;
use crate::{
    data::{self, context::AppState},
    data_models::{reference_items::Language, DetailsModel},
    manager::components::ComponentEditDetails,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_details_data(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Html<String>) {
    println!("id {}", id);
    let account = data::account::get_account_details(&app_state).await;
    if !account
        .languages
        .languages
        .iter()
        .any(|&lang_id| lang_id == id)
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Language not found".to_string()),
        );
    }

    let mut detail = data::details::get_detail(&app_state, &account.id, id).await;
    if detail.lang_name.is_empty() {
        let all_langs = data::references::get_languages(&app_state).await;
        let current_language = Language::get_from_int(&all_langs, id);
        detail.lang = id;
        detail.lang_code = current_language.code;
        detail.lang_name = current_language.name;
    }
    let component_edit_details = ComponentEditDetails {
        id: detail.id,
        lang: detail.lang,
        lang_name: detail.lang_name,
        blurb: detail.blurb.clone().unwrap_or("".to_string()),
    };

    let component_editor: String = component_edit_details.render().unwrap().to_string();
    (StatusCode::OK, Html(component_editor))
}

pub async fn get_details_home(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let account = data::account::get_account_details(&app_state).await;
    let all_langs = data::references::get_languages(&app_state).await;
    let language_list = Language::vec_from_int_vec(&all_langs, &account.languages.languages);

    let editor_home = DetailsPage {
        title: "Editor Home for SC",
        languages: language_list,
    };

    let editor_home: String = editor_home.render().unwrap().to_string();
    (StatusCode::OK, Html(editor_home))
}

pub async fn post_details_home(
    State(app_state): State<AppState>,
    Form(details_item): Form<DetailsModel>,
) -> (StatusCode, Html<String>) {
    let account = data::account::get_account_details(&app_state).await;
    let mut info: String = "Saved!".to_string();
    let result = data::details::set_details(&app_state, &account.id, details_item).await;
    if !result {
        info = "Error".to_string();
    }
    (StatusCode::OK, Html(info))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailsForm {
    blurb: String,
}
