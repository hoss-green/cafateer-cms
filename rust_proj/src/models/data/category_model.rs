use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryModel {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub owner_id: uuid::Uuid,
    pub title: Option<String>,
    pub lang_name: Option<String>
}

impl CategoryModel {
    pub fn new(id:Option<uuid::Uuid>, owner_id:uuid::Uuid) -> CategoryModel {
        CategoryModel {
            id: id.unwrap_or(uuid::Uuid::new_v4()),
            lang: 0,
            owner_id,
            title: Some("new category".to_string()),
            lang_name: None
        }
    }
}
