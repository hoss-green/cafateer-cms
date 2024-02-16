use super::{
    components::{MenuItemDetailsEditor, MenuItemEditor},
    macro_templates::MenuItemButton,
    templates::MenuPage,
};
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
                category: uuid::Uuid::nil().to_string(),// menu_item.category.unwrap_or(uuid::Uuid::nil()).to_string(),
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
    let account = data::account::get_account_details(&app_state).await;
    let menu_item = data::menu_items::get_item_by_lang(&app_state, id, lang, account.id).await;
    let menu_item_editor = MenuItemEditor {
        id: menu_item.id,
        title: menu_item.title,
        description: menu_item.description.unwrap_or(String::new()),
        lang: menu_item.lang,
        price: menu_item.price.unwrap_or(0.0),
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
        },
    )
    .await;
    if result {
        return (StatusCode::OK, Html("Saved!".to_string()));
    }
    (StatusCode::OK, Html("Error".to_string()))
}

pub async fn get_menu_item_details(
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> (StatusCode, Html<String>) {
    let account = data::account::get_account_details(&app_state).await;
    let categories = data::categories::get_category_list_by_lang(&app_state, &account.id, 1).await;
    let ref_allergies = data::references::get_allergies(&app_state).await;
    let menu_item_details = data::menu_item_details::get_menu_item_details(&app_state, id).await;
    let menu_item_editor = MenuItemDetailsEditor {
        id: menu_item_details.id,
        owner_id: account.id,
        allergies: ref_allergies,
        category: menu_item_details.category.unwrap_or(uuid::Uuid::nil()),
        categories,
    };
    let menu_editor: String = menu_item_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
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
