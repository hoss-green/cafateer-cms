use askama::Template;
use super::view_models::PrimaryLanguageVm;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct LanguageSelectionDropdownVm {
     pub user_languages: Vec<PrimaryLanguageVm>,
}

