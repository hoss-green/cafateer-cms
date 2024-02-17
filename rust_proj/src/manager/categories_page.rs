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
    let languages = data::references::get_languages(&app_state).await;
    let mut menu_items = data::manager::categories::get_category_list(&app_state, &account.id).await;
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
                title: menu_item.clone().title.unwrap_or(String::new()),
                user_languages: vec![Language::get_from_int(&languages, menu_item.lang)],
            });
        }
    }

    let menu_editor = CategoriesPage {
        title: "Edit Menu",
        category_buttons: menu_item_buttons,
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn get_category_item(
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let category = data::manager::categories::get_category(&app_state, (id, lang), &account.id).await;
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
            lang_name: None
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
