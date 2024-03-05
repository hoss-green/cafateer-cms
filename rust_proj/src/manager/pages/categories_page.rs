// use super::{macro_templates::CategoryButton, templates::CategoriesPage};
use crate::{
    data::{self, context::AppState}, manager::{macro_templates::CategoryButton, templates::CategoriesPage}, models::data::reference_items::Language
};
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn get_categories_page(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let account = data::manager::profile::get(&app_state).await;
    let account_languages = crate::data::manager::profile_languages::get_all(&app_state, account.id).await;
    let languages = account_languages.iter().map(|ac_lang_model| ac_lang_model.language).collect::<Vec<i32>>();
    let languages = Language::vec_from_int_vec(
        &data::references::get_languages(&app_state).await,
        &languages,
    );
    let mut fetched_categories =
        data::manager::categories::get_category_list(&app_state, &account.id).await;
    fetched_categories
        .sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));

    let mut unique_category_ids: HashMap<uuid::Uuid, bool> = HashMap::new();
    fetched_categories.clone().into_iter().for_each(|cat| {
        unique_category_ids.insert(cat.id, true);
    });

    // println!("{:#?}", fetched_categories);
    let category_item_buttons: Vec<CategoryButton> = unique_category_ids
        .iter()
        .map(|unique_cat| {
            let button_title = match fetched_categories
                .iter()
                .find(|cat| cat.id == *unique_cat.0 && cat.lang == account.primary_language)
            {
                Some(cat) => cat.clone().title.unwrap_or("No title".to_string()),
                None => "No title".to_string(),
            };
            CategoryButton {
                id: *unique_cat.0,
                title: button_title,
                user_languages: languages.clone(),
            }
        })
        .collect();

    let menu_editor = CategoriesPage {
        title: "Edit Menu",
        category_buttons: category_item_buttons,
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryForm {
    id: uuid::Uuid,
    title: Option<String>,
    lang: i32,
}
