use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioItem {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String
}

impl BioItem {
    pub fn new() -> BioItem{
        BioItem {
            id: uuid::Uuid::new_v4(),
            name: "My El Cafe".to_string(),
            address: "Nowhere".to_string() 
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailsItem {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub blurb: Option<String>,
}

impl DetailsItem {
    pub fn new() -> DetailsItem{
        DetailsItem {
            id: uuid::Uuid::new_v4(),
            lang: 0,//  "en".to_string(),
            blurb: None,
        }
    }
}
