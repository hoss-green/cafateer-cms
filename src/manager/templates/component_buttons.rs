use super::view_models::DropDownLanguageVm;

#[derive(Debug, Clone)]
pub struct MenuItemButtonVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub category: String,
    pub user_languages: Vec<DropDownLanguageVm>,
    pub published: bool,
}

#[derive(Debug, Clone)]
pub struct CategoryButtonVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub user_languages: Vec<DropDownLanguageVm>,
    pub published: bool,
}
