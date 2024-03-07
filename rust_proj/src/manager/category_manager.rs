use super::components::ComponentCategoryEditor;
use crate::{
    data_context::{self, context::AppState},
    models::data::CategoryModel,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_category_item(
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    println!("{:#?}", lang);
    let profile = data_context::manager::profile::get(&app_state.database_pool).await;
    let category = data_context::manager::category::get(&app_state, (id, lang), &profile.id).await;
    let category_editor = ComponentCategoryEditor {
        id: category.id,
        title: category.title.unwrap_or("".to_string()),
        lang,
    };
    let menu_editor: String = category_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn create_category_item(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    let profile = data_context::manager::profile::get(&app_state.database_pool).await;
    let result = data_context::manager::category::set(
        &app_state,
        &profile.id,
        CategoryModel {
            id: uuid::Uuid::new_v4(),
            lang: profile.primary_language,
            owner_id: profile.id,
            title: Some("new category".to_string()),
            lang_name: None,
        },
    )
    .await;
    if result {
        return (StatusCode::OK, Html("Saved!".to_string()));
    }
    (StatusCode::OK, Html("Error".to_string()))
}

pub async fn delete_category_item(
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> (StatusCode, Html<String>) {
    let profile = data_context::manager::profile::get(&app_state.database_pool).await;
    let result = data_context::manager::category::delete(&app_state, &profile.id, &id).await;
    if result {
        return (StatusCode::OK, Html(String::new()));
    }
    (StatusCode::INTERNAL_SERVER_ERROR, Html("Error".to_string()))
}

pub async fn update_category_item(
    State(app_state): State<AppState>,
    Form(details_item): Form<CategoryForm>,
) -> (StatusCode, Html<String>) {
    let profile = data_context::manager::profile::get(&app_state.database_pool).await;
    let result = data_context::manager::category::set(
        &app_state,
        &profile.id,
        CategoryModel {
            id: details_item.id,
            lang: details_item.lang,
            owner_id: profile.id,
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
