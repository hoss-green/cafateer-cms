use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLanguageVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub code: i32,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropDownLanguageVm {
    pub id: i32,
    pub name: String,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectableLanguageVm {
    pub lang_id: i32,
    pub code: String,
    pub title: String,
    pub user_lang_id: Option<uuid::Uuid>,
    pub published: bool
}

