use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemModel {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub owner_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    // pub category: Option<Uuid>,
}

impl MenuItemModel {
    pub fn new(owner_id: uuid::Uuid) -> MenuItemModel {
        MenuItemModel {
            id: uuid::Uuid::new_v4(),
            owner_id,
            lang: 0,
            title: "Item Name".to_string(),
            description: None,
            price: None,
            // category: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MenuItemDetailsModel {
    pub id: uuid::Uuid,
    pub allergies: Option<Vec<Uuid>>,
    pub category: Option<Uuid>,
}

impl MenuItemDetailsModel {
    pub fn new() -> MenuItemDetailsModel {
        MenuItemDetailsModel {
            id: uuid::Uuid::new_v4(),
            category: None,
            allergies: None,
        }
    }
}
