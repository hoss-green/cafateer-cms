use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryModel {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub owner_id: uuid::Uuid,
    pub title: Option<String>,
}

impl CategoryModel {
    pub fn new(owner_id:uuid::Uuid) -> CategoryModel {
        CategoryModel {
            id: uuid::Uuid::new_v4(),
            lang: 0,
            owner_id,
            title: Some("new category".to_string())
        }
    }
}
