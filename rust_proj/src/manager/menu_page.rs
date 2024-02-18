use std::collections::HashMap;

use super::{
    components::{MenuItemDetailsEditor, MenuItemEditor},
    macro_templates::MenuItemButton,
    templates::MenuPage,
};
use crate::{
    data::{self, context::AppState, manager::categories},
    models::data::{reference_items::Language, CategoryModel, MenuItemDetailsModel, MenuItemModel},
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
    let account = data::manager::account::get_account_details(&app_state).await;
    // let categories = data::manager::categories::get_category_list(&app_state, &account.id).await;
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
                category: match menu_item_details.iter().find(|menu_item_desc| {
                    menu_item_desc.id == *unique_mi.0
                }) {
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

pub async fn post_menu_item(
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

pub async fn get_menu_item_details(
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let languages = Language::vec_from_int_vec(
        &data::references::get_languages(&app_state).await,
        &account.languages.languages,
    );
    let fetched_categories =
        data::manager::categories::get_category_list(&app_state, &account.id).await;
    let ref_allergies = data::references::get_allergies(&app_state).await;
    let mut unique_category_ids: HashMap<uuid::Uuid, bool> = HashMap::new();
    fetched_categories.clone().into_iter().for_each(|cat| {
        unique_category_ids.insert(cat.id, true);
    });

    let mut categories: Vec<CategoryModel> = vec![];
    unique_category_ids.iter().for_each(|unique_cat| {
        let mut fc = languages
            .iter()
            .filter_map(|lang| {
                match fetched_categories
                    .iter()
                    .find(|c| c.id == *unique_cat.0 && c.lang == lang.id)
                {
                    Some(cat) => Some(cat.clone()),
                    None => Some(CategoryModel {
                        id: *unique_cat.0,
                        lang: lang.id,
                        owner_id: account.id,
                        title: Some(format!("Missing Translation [{}]", lang.name).to_string()),
                        lang_name: Some(lang.name.to_string()),
                    }),
                }
            })
            .filter(|cm| cm.lang == account.languages.main_language) // account.languages.main_language)
            .collect::<Vec<CategoryModel>>();
        categories.append(&mut fc);
    });

    let menu_item_details =
        data::manager::menu_item_details::get_menu_item_detail(&app_state, &account.id, &id).await;
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
