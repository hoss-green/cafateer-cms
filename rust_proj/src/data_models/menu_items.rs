use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub owner_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub category: Option<Uuid>,
}

impl MenuItem {
    pub fn new(owner_id:uuid::Uuid) -> MenuItem {
        MenuItem {
            id: uuid::Uuid::new_v4(),
            owner_id,
            lang: 0,
            title: "Item Name".to_string(),
            description: None,
            price: None,
            category: None,
            
        }
    }
}
