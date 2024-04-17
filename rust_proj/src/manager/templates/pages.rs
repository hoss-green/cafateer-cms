use crate::manager::templates::component_buttons::MenuItemButtonVm;
use crate::manager::templates::component_buttons::CategoryButtonVm;
use crate::models::data::reference_items::Language;
use askama::Template;

use super::view_models::AccountLanguageVm;
use super::view_models::SelectableLanguageVm;

#[derive(Template)]
#[template(path = "manager/start_page.html")]
pub struct StartPageVm<'a> {
    pub title: &'a str,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/config_page.html")]
pub struct ConfigPageVm<'a> {
    pub title: &'a str,
    pub primary_language: Language,
    pub user_languages: Vec<AccountLanguageVm>,

}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/languages_page.html")]
pub struct LanguagesPageVm<'a> {
    pub title: &'a str,
    pub primary_language: Language,
    pub languages: Vec<SelectableLanguageVm>
}

#[derive(Template)]
#[template(path = "manager/details_page.html")]
pub struct DetailsPageVm<'a> {
    pub title: &'a str,
    pub languages: Vec<Language>,
}

#[derive(Template)]
#[template(path = "manager/categories_page.html")]
pub struct CategoriesPageVm<'a> {
    pub title: &'a str,
    pub category_buttons: Vec<CategoryButtonVm>,
}

#[derive(Template)]
#[template(path = "manager/bio_page.html")]
pub struct BioPageVm<'a> {
    pub title: &'a str,
    pub id: uuid::Uuid,
    pub lang: String,
    pub name: String,
    pub info: String,
}

#[derive(Template)]
#[template(path = "manager/menu_page.html")]
pub struct MenuPageVm<'a> {
    pub title: &'a str,
    pub menu_item_buttons: Vec<MenuItemButtonVm>,
}

pub fn is_selected(id: &i32, languages: &[Language]) -> String {
    match languages.iter().any(|item| item.id == *id) {
        true => "checked".to_string(),
        false => "".to_string(),
    }
}
