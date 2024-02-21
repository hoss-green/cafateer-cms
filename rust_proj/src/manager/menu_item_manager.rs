use super::
    components::MenuItemEditor
;
use crate::{
    data::{self, context::AppState},
    models::data::{CategoryModel, MenuItemModel},
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};


pub async fn get_menu_item(
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let menu_item =
        data::manager::menu_items::get_item_by_lang(&app_state, id, lang, account.id).await;
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

pub async fn update_menu_item(
    State(app_state): State<AppState>,
    Form(menu_item_form): Form<MenuItemForm>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let result = data::manager::menu_items::set_item(
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


pub async fn create_menu_item(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let result = data::manager::menu_item::set(
        &app_state,
        &account.id,
        MenuItemModel {
            id: uuid::Uuid::new_v4(),
            lang: account.languages.main_language,
            owner_id: account.id,
            title: "new menu_item".to_string(),
            description: None,
            price: None 
        },
    )
    .await;
    if result {
        return (StatusCode::OK, Html("Saved!".to_string()));
    }
    (StatusCode::OK, Html("Error".to_string()))
}

pub async fn delete_menu_item(
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let result = data::manager::menu_item::delete(&app_state, &account.id, &id).await;
    if result {
        return (StatusCode::OK, Html(String::new()));
    }
    (StatusCode::INTERNAL_SERVER_ERROR, Html("Error".to_string()))
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
