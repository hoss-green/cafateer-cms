use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountLanguagesModel {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub language: i32,
    // pub is_primary: bool,
}

impl AccountLanguagesModel {
    pub fn new(owner_id:uuid::Uuid) -> AccountLanguagesModel {
        AccountLanguagesModel { 
            id: uuid::Uuid::new_v4(),
            owner_id,
            language: 0,
            // is_primary: false 
        }
    }
}
