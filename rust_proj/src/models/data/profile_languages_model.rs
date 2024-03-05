use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileLanguagesModel {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub language: i32,
    // pub is_primary: bool,
}

impl ProfileLanguagesModel {
    pub fn new(owner_id:uuid::Uuid) -> ProfileLanguagesModel {
        ProfileLanguagesModel { 
            id: uuid::Uuid::new_v4(),
            owner_id,
            language: 0,
            // is_primary: false 
        }
    }
}
