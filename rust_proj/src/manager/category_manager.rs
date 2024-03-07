use super::components::ComponentCategoryEditor;
use crate::{
    data_context::{self, context::AppState},
    models::data::CategoryModel,
    session::claims::Claims,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    response::Html,
    Extension, Form,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get_category_item(
    Extension(claims): Extension<Claims>,
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    let category =
        data_context::manager::category::get(&app_state.database_pool, (id, lang), &claims.sub)
            .await;
    let category_editor = ComponentCategoryEditor {
        id: category.id,
        title: category.title.unwrap_or("".to_string()),
        lang,
    };
    let menu_editor: String = category_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor))
}

pub async fn create_category_item(
    Extension(claims): Extension<Claims>,
    State(app_state): State<AppState>,
) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let result = data_context::manager::category::set(
        database_pool,
        &claims.sub,
        &CategoryModel {
            id: uuid::Uuid::new_v4(),
            lang: claims.language,
            owner_id: claims.sub,
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
    Extension(claims): Extension<Claims>,
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    match data_context::manager::category::delete(&app_state.database_pool, &claims.sub, &id).await
    {
        true => return (StatusCode::OK, Html(String::new())).into_response(),
        false => (StatusCode::INTERNAL_SERVER_ERROR, Html("Error".to_string())).into_response()
    }
}

pub async fn update_category_item(
    Extension(claims): Extension<Claims>,
    State(app_state): State<AppState>,
    Form(details_item): Form<CategoryForm>,
) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let result = data_context::manager::category::set(
        database_pool,
        &claims.sub,
        &CategoryModel {
            id: details_item.id,
            lang: details_item.lang,
            owner_id: claims.sub,
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
