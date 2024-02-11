use askama::Template;

use crate::data_models::reference_items::Language;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/language_selection_dropdown.html")]
pub struct LanguageSelectionDropdown {
    pub all_available_languages: Vec<Language>,
    pub user_selected_languages: Vec<Language>,

}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/primary_language_list.html")]
pub struct PrimaryLanguageList {
    pub primary_language_id: i32, 
    pub user_selected_languages: Vec<Language>,
}
