use std::collections::HashMap;

use super::{
    components::ComponentCategoryEditor, macro_templates::CategoryButton, templates::CategoriesPage,
};
use crate::{
    data::{self, context::AppState},
    models::data::{reference_items::Language, CategoryModel},
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
    let account = data::manager::account::get_account_details(&app_state).await;
    let languages = Language::vec_from_int_vec(
        &data::references::get_languages(&app_state).await,
        &account.languages.languages,
    );
    let mut fetched_categories =
        data::manager::categories::get_category_list(&app_state, &account.id).await;
    fetched_categories
        .sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));

    let mut unique_category_ids: HashMap<uuid::Uuid, bool> = HashMap::new();
    fetched_categories.clone().into_iter().for_each(|cat| {
        unique_category_ids.insert(cat.id, true);
    });

    println!("{:#?}", fetched_categories);
    let category_item_buttons: Vec<CategoryButton> = unique_category_ids.iter().map(|unique_cat| {
        let button_title = match fetched_categories.iter().find(|cat| cat.id == *unique_cat.0 && cat.lang == account.languages.main_language) {
            Some(cat) => cat.clone().title.unwrap_or("No title".to_string()),
            None => "No title".to_string()
        };
        CategoryButton { id: *unique_cat.0, title: button_title, user_languages: languages.clone() }
    }).collect();

    println!("{:#?}", category_item_buttons);
    let menu_editor = CategoriesPage {
        title: "Edit Menu",
        category_buttons: category_item_buttons,
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn get_category_item(
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let category =
        data::manager::categories::get_category(&app_state, (id, lang), &account.id).await;
    let category_editor = ComponentCategoryEditor {
        id: category.id,
        title: category.title.unwrap_or("".to_string()),
        lang: category.lang,
    };
    let menu_editor: String = category_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn post_category_item(
    State(app_state): State<AppState>,
    Form(details_item): Form<CategoryForm>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let result = data::manager::categories::set_category(
        &app_state,
        &account.id,
        CategoryModel {
            id: details_item.id,
            lang: details_item.lang,
            owner_id: account.id,
            title: details_item.title,
            lang_name: None,
        },
    )
    .await;
    if result {
        return (StatusCode::OK, Html("Saved!".to_string()));
    }
    (StatusCode::OK, Html("Error".to_string()))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryForm {
    id: uuid::Uuid,
    title: Option<String>,
    lang: i32,
}
