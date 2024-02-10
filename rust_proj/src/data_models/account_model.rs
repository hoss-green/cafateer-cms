use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct AccountModel {
    pub id: uuid::Uuid,
    pub languages: sqlx::types::Json<LaunguageJsonModel>
    // pub main_language: i32,
    // pub languages: sqlx::types::JsonValue
}

impl AccountModel {
    pub fn new() -> AccountModel {
        AccountModel { 
            id: uuid::Uuid::new_v4(), 
            // main_language: 0, 
            // languages: json!(Vec::<i32>::new())
            languages: sqlx::types::Json(LaunguageJsonModel{ main_language: 0, languages: vec![0] })
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaunguageJsonModel {
    pub main_language: i32,
    pub languages: Vec::<i32> 
}
