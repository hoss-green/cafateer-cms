use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: uuid::Uuid,
    pub title: String,
    pub lang: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub category: Option<Uuid>,
}

impl MenuItem {
    pub fn new() -> MenuItem {
        MenuItem {
            id: uuid::Uuid::new_v4(),
            lang: "en".to_string(),
            title: "Item Name".to_string(),
            description: None,
            price: None,
            category: None,
            
        }
    }
}
