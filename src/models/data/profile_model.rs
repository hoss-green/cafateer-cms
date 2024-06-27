use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct ProfileModel {
    pub id: uuid::Uuid,
    pub primary_language: i32
}

impl ProfileModel {
    pub fn new() -> ProfileModel {
        ProfileModel { 
            id: uuid::Uuid::new_v4(), 
            primary_language: 0
        }
    }
}

