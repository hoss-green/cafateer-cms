use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryModel {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub owner_id: uuid::Uuid,
    pub title: Option<String>,
    pub lang_name: Option<String>
}

impl CategoryModel {
    pub fn new(id:Option<uuid::Uuid>, owner_id: &uuid::Uuid) -> CategoryModel {
        CategoryModel {
            id: id.unwrap_or(uuid::Uuid::new_v4()),
            lang: 0,
            owner_id:*owner_id, 
            title: Some("new category".to_string()),
            lang_name: None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CategoryDetailsModel {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub published: bool
}

impl CategoryDetailsModel {
    pub fn new(id:uuid::Uuid, owner_id:uuid::Uuid) -> CategoryDetailsModel {
        CategoryDetailsModel {
            id,//: uuid::Uuid::new_v4(),
            owner_id,
            published: false
        }
    }
}
