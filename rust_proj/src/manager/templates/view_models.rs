use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLanguageVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub code: i32,
    pub published: bool,
}
