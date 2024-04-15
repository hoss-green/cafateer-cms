use crate::{
    data_context::{self, context::AppState},
    manager::templates::{components::MenuItemEditorVm, page_buttons::MenuItemEditButton},
    models::data::{ClaimsModel, MenuItemModel},
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

pub async fn get_menu_item(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path((id, lang)): Path<(uuid::Uuid, i32)>,
) -> (StatusCode, Html<String>) {
    let menu_item = data_context::manager::menu_item::get_by_lang(
        &app_state.database_pool,
        &id,
        lang,
        &claims.sub,
    )
    .await;
    let menu_item_editor = MenuItemEditorVm {
        id: menu_item.id,
        title: menu_item.title,
        description: menu_item.description.unwrap_or(String::new()),
        lang: menu_item.lang,
    }
    .render()
    .unwrap()
    .to_string();
    (StatusCode::OK, Html(menu_item_editor))
}

pub async fn update_menu_item(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Form(menu_item_form): Form<MenuItemForm>,
) -> (StatusCode, Html<String>) {
    let result = data_context::manager::menu_item::set(
        &app_state.database_pool,
        &claims.sub,
        MenuItemModel {
            id: menu_item_form.id,
            lang: menu_item_form.lang,
            owner_id: claims.sub,
            title: menu_item_form.title,
            description: menu_item_form.description,
        },
    )
    .await;
    if result.is_some() {
        return (StatusCode::OK, Html("Saved!".to_string()));
    }
    (StatusCode::OK, Html("Error".to_string()))
}

pub async fn create_menu_item(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
        let mim = MenuItemModel {
            id: uuid::Uuid::new_v4(),
            lang: claims.body.lang,
            owner_id: claims.sub,
            title: "new menu_item".to_string(),
            description: None,
        };
    match data_context::manager::menu_item::create(
        &app_state.database_pool,
        &claims.sub,
        &mim
    )
    .await
    {
        true => MenuItemEditButton {
            id: mim.id,
            category: String::new(),
            enabled: false,
            title: mim.title,
            languages: vec![],
        }
        .render()
        .unwrap()
        .into_response(),
        false => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_menu_item(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let result =
        data_context::manager::menu_item::delete(&app_state.database_pool, &claims.sub, &id).await;
    if result {
        return (StatusCode::OK, Html(String::new())).into_response();
    }
    (StatusCode::INTERNAL_SERVER_ERROR, Html("Error".to_string())).into_response()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuItemForm {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<uuid::Uuid>,
}
