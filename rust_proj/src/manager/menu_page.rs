use super::{components::MenuItemEditor, macro_templates::MenuItemButton, templates::MenuPage};
use crate::{
    data::{self, context::AppState},
    data_models::{reference_items::Language, MenuItemModel},
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_menu_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let languages = data::references::get_languages(&app_state).await;
    let mut menu_items = data::menu_items::get_items_for_account(&app_state).await;
    let mut menu_item_buttons: Vec<MenuItemButton> = vec![];
    menu_items.sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));
    for menu_item in menu_items.clone() {
        if menu_item_buttons.iter().any(|mi| mi.id == menu_item.id) {
            menu_item_buttons
                .iter_mut()
                .filter(|mi| mi.id == menu_item.id)
                .collect::<Vec<&mut MenuItemButton>>()[0]
                .user_languages
                .push(Language::get_from_int(&languages, menu_item.lang));
        } else {
            menu_item_buttons.push(MenuItemButton {
                id: menu_item.id,
                title: menu_item.clone().title,
                category: menu_item.category.unwrap().to_string(),
                user_languages: vec![Language::get_from_int(&languages, menu_item.lang)],
            });
        }
    }

    let menu_editor = MenuPage {
        title: "Edit Menu",
        menu_item_buttons,
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn get_menu_item(
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    let menu_item = data::menu_items::get_item(&app_state, id, lang).await;
    let menu_item_editor = MenuItemEditor {
        id: menu_item.id,
        title: menu_item.title,
        description: menu_item.description.unwrap_or(String::new()),
        lang: menu_item.lang,
        price: menu_item.price.unwrap_or(0.0),
        category: menu_item.category.unwrap_or(uuid::Uuid::nil()).to_string(),
    };
    let menu_editor: String = menu_item_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn set_menu_item(
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    let menu_item = data::menu_items::get_item(&app_state, id, lang).await;
    let menu_item_editor = MenuItemEditor {
        id: menu_item.id,
        title: menu_item.title,
        description: menu_item.description.unwrap_or(String::new()),
        lang: menu_item.lang,
        price: menu_item.price.unwrap_or(0.0),
        category: menu_item.category.unwrap_or(uuid::Uuid::nil()).to_string(),
    };
    let menu_editor: String = menu_item_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn post_menu_item(
    State(app_state): State<AppState>,
    Form(menu_item_form): Form<MenuItemForm>,
) -> (StatusCode, Html<String>) {
    let account = data::account::get_account_details(&app_state).await;
    let result = data::menu_items::set_item(
        &app_state,
        &account.id,
        MenuItemModel {
            id: menu_item_form.id,
            lang: menu_item_form.lang,
            owner_id: account.id,
            title: menu_item_form.title,
            description: menu_item_form.description, 
            price: menu_item_form.price, 
            category: menu_item_form.category,
        },
    )
    .await;
    if result {
        return (StatusCode::OK, Html("Saved!".to_string()));
    }
    (StatusCode::OK, Html("Error".to_string()))
}
pub async fn post_details_home(
    State(_app_state): State<AppState>,
    Form(_menu_item): Form<MenuItemModel>,
) -> (StatusCode, Html<String>) {
    let info: String = "Details updated successfully".to_string();
    (StatusCode::OK, Html(info))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuItemForm {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub title: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub category: Option<uuid::Uuid>,
}
