use askama::Template;

use crate::data_models::reference_items::Language;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct LanguageSelectionDropdown {
    pub all_available_languages: Vec<Language>,
    pub user_selected_languages: Vec<Language>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct MenuItemButton {
    pub id: uuid::Uuid,
    pub title: String,
    pub category: String,
    pub user_languages: Vec<Language>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct CategoryButton {
    pub id: uuid::Uuid,
    pub title: String,
    pub user_languages: Vec<Language>,
}
