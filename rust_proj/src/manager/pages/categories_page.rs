use crate::{
    data_context::{self, context::AppState},
    manager::{macro_templates::CategoryButton, templates::CategoriesPage},
    models::data::reference_items::Language, session::claims::Claims,
};
use askama::Template;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn get_categories_page(
    Extension(claims): Extension<Claims>,
    State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    let languages = account_languages
        .iter()
        .map(|ac_lang_model| ac_lang_model.language)
        .collect::<Vec<i32>>();
    let languages = Language::vec_from_int_vec(
        &data_context::references::get_languages(database_pool).await,
        &languages,
    );
    let mut fetched_categories =
        data_context::manager::categories::get_category_list(database_pool, &claims.sub).await;
    fetched_categories
        .sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));

    let mut unique_category_ids: HashMap<uuid::Uuid, bool> = HashMap::new();
    fetched_categories.clone().into_iter().for_each(|cat| {
        unique_category_ids.insert(cat.id, true);
    });

    let category_item_buttons: Vec<CategoryButton> = unique_category_ids
        .iter()
        .map(|unique_cat| {
            let button_title = match fetched_categories
                .iter()
                .find(|cat| cat.id == *unique_cat.0 && cat.lang == claims.language)
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
