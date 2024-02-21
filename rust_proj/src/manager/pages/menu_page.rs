use crate::{
    data::{self, context::AppState},
    manager::{macro_templates::MenuItemButton, templates::MenuPage},
    models::data::{reference_items::Language, MenuItemDetailsModel},
};
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn get_menu_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let menu_item_details: Vec<MenuItemDetailsModel> =
        data::manager::menu_item_details::get_menu_item_details(&app_state, &account.id).await;
    let languages = Language::vec_from_int_vec(
        &data::references::get_languages(&app_state).await,
        &account.languages.languages,
    );
    let mut menu_items = data::manager::menu_items::get_items_for_account(&app_state).await;
    menu_items.sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));
    let mut unique_menu_ids: HashMap<uuid::Uuid, bool> = HashMap::new();
    menu_items.clone().into_iter().for_each(|mi| {
        unique_menu_ids.insert(mi.id, true);
    });

    let menu_item_buttons: Vec<MenuItemButton> = unique_menu_ids
        .iter()
        .map(|unique_mi| {
            let button_title = match menu_items
                .iter()
                .find(|mi| mi.id == *unique_mi.0 && mi.lang == account.languages.main_language)
            {
                Some(cat) => cat.clone().title,
                None => "No title".to_string(),
            };
            MenuItemButton {
                id: *unique_mi.0,
                title: button_title,
                category: match menu_item_details
                    .iter()
                    .find(|menu_item_desc| menu_item_desc.id == *unique_mi.0)
                {
                    Some(cat) => cat.id.clone().to_string(),
                    None => "None".to_string(),
                },
                user_languages: languages.clone(),
            }
        })
        .collect();

    let menu_editor = MenuPage {
        title: "Edit Menu",
        menu_item_buttons,
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
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
