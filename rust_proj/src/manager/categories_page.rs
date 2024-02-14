use super::{
    macro_templates::CategoryButton,
    templates::{CategoriesPage, DetailsPage},
};
use crate::{
    data::{self, categories, context::AppState},
    data_models::{reference_items::Language, DetailsModel},
    manager::components::ComponentDetailEditor,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_categories_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let account = data::account::get_account_details(&app_state).await;
    let languages = data::references::get_languages(&app_state).await;
    let mut menu_items = data::categories::get_category_list(&app_state, &account.id).await;
    let mut menu_item_buttons: Vec<CategoryButton> = vec![];
    menu_items.sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));
    for menu_item in menu_items.clone() {
        if menu_item_buttons.iter().any(|mi| mi.id == menu_item.id) {
            menu_item_buttons
                .iter_mut()
                .filter(|mi| mi.id == menu_item.id)
                .collect::<Vec<&mut CategoryButton>>()[0]
                .user_languages
                .push(Language::get_from_int(&languages, menu_item.lang));
        } else {
            menu_item_buttons.push(CategoryButton {
                id: menu_item.id,
                name: menu_item.clone().name,
                user_languages: vec![Language::get_from_int(&languages, menu_item.lang)],
            });
        }
    }

    let menu_editor = CategoriesPage {
        title: "Edit Menu",
        category_buttons: menu_item_buttons
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn get_details_data(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Html<String>) {
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
    let component_edit_details = ComponentDetailEditor {
        id: detail.id,
        lang: detail.lang,
        lang_name: detail.lang_name,
        blurb: detail.blurb.clone().unwrap_or("".to_string()),
    };

    let component_editor: String = component_edit_details.render().unwrap().to_string();
    (StatusCode::OK, Html(component_editor))
}

// pub async fn get_categories_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
//     let account = data::account::get_account_details(&app_state).await;
//     let all_langs = data::references::get_languages(&app_state).await;
//     let language_list = Language::vec_from_int_vec(&all_langs, &account.languages.languages);
//
//     let mut categories = data::categories::get_category_list(&app_state, &account.id).await;
//     let editor_home = CategoriesPage {
//         title: "Editor Home for SC",
//         languages: language_list,
//         // categories: categories
//     };
//
//     let editor_home: String = editor_home.render().unwrap().to_string();
//     (StatusCode::OK, Html(editor_home))
// }

pub async fn post_categories(
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
