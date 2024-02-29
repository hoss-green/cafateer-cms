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
}

impl MenuItemModel {
    pub fn new(id: uuid::Uuid, owner_id: uuid::Uuid, lang:i32) -> MenuItemModel {
        MenuItemModel {
            id, 
            owner_id,
            lang, //: lang.unwrap_or(0),
            title: "Item Name".to_string(),
            description: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MenuItemDetailsModel {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub allergies: Option<sqlx::types::Json<Vec<Uuid>>>,
    pub category: Option<Uuid>,
    pub price: Option<f64>,
}

impl MenuItemDetailsModel {
    pub fn new(id:uuid::Uuid, owner_id:uuid::Uuid) -> MenuItemDetailsModel {
        MenuItemDetailsModel {
            id,//: uuid::Uuid::new_v4(),
            owner_id,
            category: None,
            allergies: None,
            price: None,
        }
    }
}
