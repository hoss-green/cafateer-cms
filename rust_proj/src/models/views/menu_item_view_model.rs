use askama::Template;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Template, Clone, Serialize, Deserialize)]
#[template(path = "presenter/components/menu_item.html")]
pub struct MenuItemViewModel {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>, 
    pub lang: i32,
    pub price: Option<f64>,
    pub owner_id: uuid::Uuid,
    pub allergies: Option<Vec<uuid::Uuid>>,
    pub category: Option<uuid::Uuid>,
}
