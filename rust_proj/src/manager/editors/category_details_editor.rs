use askama_axum::IntoResponse;
use axum::{extract::{Path, State}, Extension, Form};
use serde::{Deserialize, Serialize};

use crate::{data_context::context::AppState, models::data::ClaimsModel, session::claims::Claims};

pub async fn get_category_details(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,

) -> impl IntoResponse {
   "hello".into_response()
}

pub async fn update_category_details(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Form(menu_item_form): Form<MenuItemDetailsForm>,

) -> impl IntoResponse {
   "hello".into_response()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuItemDetailsForm {
    pub id: uuid::Uuid,
    pub ownder_id: Option<uuid::Uuid>,
    pub published: Option<bool>,
}
