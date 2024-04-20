use super::view_models::DropDownLanguageVm;
// use askama::Template;

// #[derive(Template, Debug, Clone)]
// #[template(path = "manager/macros/language_selection_dropdown.html")]
// pub struct PrimaryLanguageButtonVm {
//     pub id: uuid::Uuid,
//     pub title: String,
//     pub category: String,
//     pub user_languages: Vec<DropDownLanguageVm>,
//     pub published: bool,
// }

#[derive(Debug, Clone)]
// #[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct MenuItemButtonVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub category: String,
    pub user_languages: Vec<DropDownLanguageVm>,
    pub published: bool,
}

#[derive(Debug, Clone)]
// #[template(path = "manager/macros/language_selection_dropdown.html")]
pub struct CategoryButtonVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub user_languages: Vec<DropDownLanguageVm>,
    pub published: bool,
}
