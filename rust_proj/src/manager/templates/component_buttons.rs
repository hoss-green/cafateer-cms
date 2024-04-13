use askama::Template;

use crate::models::data::reference_items::Language;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct LanguageSelectionDropdownVm {
    pub all_available_languages: Vec<Language>,
    pub user_selected_languages: Vec<Language>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct MenuItemButtonVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub category: String,
    pub user_languages: Vec<Language>,
    pub published: bool
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct CategoryButtonVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub user_languages: Vec<Language>,
    pub published: bool
}
