use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String,
}

impl BioModel {
    pub fn new() -> BioModel {
        BioModel {
            id: uuid::Uuid::new_v4(),
            name: "My El Cafe".to_string(),
            address: "Nowhere".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailsModel {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub blurb: Option<String>,
}

impl DetailsModel {
    pub fn new(id: uuid::Uuid, lang: i32, blurb: Option<String>) -> DetailsModel {
        DetailsModel {
            id,
            lang, //  "en".to_string(),
            blurb,
        }
    }
    pub fn default() -> DetailsModel {
        DetailsModel {
            id: uuid::Uuid::new_v4(),
            lang: 0, //  "en".to_string(),
            blurb: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailLangModel {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub blurb: Option<String>,
    pub lang_code: String,
    pub lang_name: String,
}

impl DetailLangModel {
    pub fn new(
        id: uuid::Uuid,
        lang: i32,
        blurb: Option<String>,
        lang_code: String,
        lang_name: String,
    ) -> DetailLangModel {
        DetailLangModel {
            id,
            lang,
            blurb,
            lang_code,
            lang_name,
        }
    }
    pub fn default() -> DetailLangModel {
        DetailLangModel {
            id: uuid::Uuid::new_v4(),
            lang: 0, //  "en".to_string(),
            blurb: None,
            lang_code: "en".to_string(),
            lang_name: "English".to_string(),
        }
    }
}
