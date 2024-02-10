use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct AccountModel {
    pub id: uuid::Uuid,
    pub languages: sqlx::types::Json<LaunguageJsonModel>
}

impl AccountModel {
    pub fn new() -> AccountModel {
        AccountModel { 
            id: uuid::Uuid::new_v4(), 
            languages: sqlx::types::Json(LaunguageJsonModel{ main_language: 0, languages: vec![0] })
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaunguageJsonModel {
    pub main_language: i32,
    pub languages: Vec::<i32> 
}

