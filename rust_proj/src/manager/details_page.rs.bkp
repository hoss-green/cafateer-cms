use super::templates::DetailsPage;
use crate::{
    data::{self, context::AppState},
    data_models::{reference_items::Language, DetailLangModel, DetailsModel},
    view_models::components::DetailsViewModel,
};
use askama::Template;
use axum::{extract::State, response::Html, Form};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_details_home(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let account = data::account::get_account_details(&app_state).await;
    let language_list = data::references::get_languages(&app_state).await;
    let mut details_list = data::details::get_details_list(&app_state).await;
    let id = match details_list.len() {
        0 => uuid::Uuid::new_v4(),
        _ => details_list[0].id,
    };
    let mut user_selected_languages = account
        .languages
        .languages
        .iter()
        .filter(|&lang_id| !details_list.iter().any(|d| d.lang == *lang_id))
        .collect::<Vec<&i32>>()
        .iter()
        .map(|&lang_id| {
            let language:Language = Language::get_from_int(&language_list, *lang_id);
            DetailLangModel::new(id, *lang_id, None, language.code, language.name)
        })
        .collect::<Vec<DetailLangModel>>();
    details_list.retain(|detail| {
        account
            .languages
            .languages
            .iter()
            .any(|lang| detail.lang == *lang)
    });
    details_list.append(&mut user_selected_languages);
    let available_details: Vec<DetailsViewModel> = details_list 
        .iter()
        .map(|item| DetailsViewModel {
            id: item.id,
            lang: item.lang,
            blurb: item.blurb.clone().unwrap_or("".to_string()),
            lang_code: item.lang_code.clone(),
            lang_name: item.lang_name.clone(),
        })
        .collect::<Vec<DetailsViewModel>>();

    let editor_home = DetailsPage {
        title: "Editor Home for SC",
        languages: language_list,
        details: available_details,
    };

    let editor_home: String = editor_home.render().unwrap().to_string();

    (StatusCode::OK, Html(editor_home))
}

pub async fn post_details_home(
    State(app_state): State<AppState>,
    Form(details_item): Form<DetailsModel>,
) -> (StatusCode, Html<String>) {
    println!("hit");
    let mut info: String = "Saved!".to_string();
    let result = data::details::set_details(&app_state, details_item).await;
    if !result {
        info = "Error".to_string();
    }
    (StatusCode::OK, Html(info))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailsForm {
    blurb: String,
}
