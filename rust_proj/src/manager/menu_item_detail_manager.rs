use super::components::MenuItemDetailsEditor;
use crate::{
    data::{self, context::AppState},
    models::data::{reference_items::Language, CategoryModel, MenuItemDetailsModel},
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        price: menu_item_details.price.unwrap_or(0.0),
        categories,
    };
    let menu_editor: String = menu_item_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn update_menu_item_details(
    State(app_state): State<AppState>,
    Form(menu_item_form): Form<MenuItemDetailsForm>,
) -> (StatusCode, Html<String>) {
    let account = data::manager::account::get_account_details(&app_state).await;
    let result = data::manager::menu_item_details::set(
        &app_state,
        &account.id,
        &MenuItemDetailsModel {
            id: menu_item_form.id,
            owner_id: account.id,
            category: menu_item_form.category,
            allergies: match menu_item_form.allergies {
                Some(ags) => Some(sqlx::types::Json(ags)),
                None => None,
            },
            price: menu_item_form.price,
        },
    )
    .await;
    if result {
        return (StatusCode::OK, Html("Saved!".to_string()));
    }
    (StatusCode::OK, Html("Error".to_string()))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuItemDetailsForm {
    pub id: uuid::Uuid,
    pub category: Option<uuid::Uuid>,
    pub allergies: Option<Vec<uuid::Uuid>>,
    pub price: Option<f64>,
}
