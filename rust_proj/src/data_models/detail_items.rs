
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioItem {
    pub id: uuid::Uuid,
    pub lang: String,
    pub name: String,
    pub info: Option<String>,
}

impl BioItem {
    pub fn new() -> BioItem{
        BioItem {
            id: uuid::Uuid::new_v4(),
            lang: "en".to_string(),
            name: "My El Cafe".to_string(),
            info: None
        }
    }
}
