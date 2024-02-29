use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct AccountModel {
    pub id: uuid::Uuid,
    pub primary_language: i32
}

impl AccountModel {
    pub fn new() -> AccountModel {
        AccountModel { 
            id: uuid::Uuid::new_v4(), 
            primary_language: 0
        }
    }
}

