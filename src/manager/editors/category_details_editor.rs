use crate::{
    data_context::context::AppState,
    manager::templates::{
        components::CategoryDetailEditorVm,
        toggle_buttons::{DisableButton, EnableButton},
    },
    models::data::ClaimsModel,
    session::claims::Claims,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    response::Html,
    Extension,
};
use serde::{Deserialize, Serialize};

pub async fn get_category_details(
    Extension(_claims): Extension<Claims<ClaimsModel>>,
    State(_app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    CategoryDetailEditorVm { id }
        .render()
        .unwrap()
        .into_response()
}

pub async fn enable_category(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let _enable_success =
        crate::data_context::manager::category_detail::enable(database_pool, &claims.sub, &id)
            .await;

    let button: DisableButton = DisableButton {
        post_url: format!("/manager/menu/categories/disable/{}", id), //.to_string(),
        button_text: "Disable".to_string(),
    };

    Html(button.render().unwrap()).into_response()
}

pub async fn disable_category(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let _disable_success =
        crate::data_context::manager::category_detail::disable(database_pool, &claims.sub, &id)
            .await;
    let button: EnableButton = EnableButton {
        post_url: format!("/manager/menu/categories/enable/{}", id), //.to_string(),
        button_text: "Enable".to_string(),
    };

    Html(button.render().unwrap()).into_response()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuItemDetailsForm {
    pub id: uuid::Uuid,
    pub ownder_id: Option<uuid::Uuid>,
    pub published: Option<bool>,
}
