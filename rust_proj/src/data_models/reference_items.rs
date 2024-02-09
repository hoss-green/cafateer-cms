use std::i32;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Allergy {
    pub id: uuid::Uuid,
    pub icon: Option<String>,
    pub en: Option<String>,
    pub es: Option<String>,
    pub de: Option<String>,
    pub fr: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Language {
    pub id: i32,
    pub code: String,
    pub name: String,
}
