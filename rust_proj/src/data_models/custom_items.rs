use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>, 
    pub price: Option<f64>,
    pub category: Option<String>,
}
