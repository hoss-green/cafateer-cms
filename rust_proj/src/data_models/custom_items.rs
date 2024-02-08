use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>, 
    pub price: Option<f64>,
    pub category: Option<String>,
}

impl MenuItem {
    pub fn new() -> MenuItem{
        MenuItem { id: uuid::Uuid::new_v4(), title: "Item Name".to_string(), description: None, price: None, category: None }
    }
}
