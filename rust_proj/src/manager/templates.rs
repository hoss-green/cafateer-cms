use askama::Template;

use crate::{data_models::reference_items::Language, view_models::components::DetailsViewModel};

#[derive(Template)]
#[template(path = "manager/start_page.html")]
pub struct StartPage<'a> {
    pub title: &'a str,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/account_page.html")]
pub struct AccountPage<'a> {
    pub title: &'a str,
    pub primary_language: Language,
    pub system_languages: Vec<Language>,
    pub user_languages: Vec<Language>,
}

#[derive(Template)]
#[template(path = "manager/details_page.html")]
pub struct DetailsPage<'a> {
    pub title: &'a str,
    pub languages: Vec<Language>,
    pub details: Vec<DetailsViewModel>,
}

#[derive(Template)]
#[template(path = "manager/bio_page.html")]
pub struct BioPage<'a> {
    pub title: &'a str,
    pub id: uuid::Uuid,
    pub lang: String,
    pub name: String,
    pub info: String,
}



// pub fn foo() -> String {
//    "OMG".to_string()
// }

pub fn is_selected(id:&i32, languages:&Vec<Language>) -> String {
   match languages.iter().any(|item| item.id == *id) {
    true => "checked".to_string(),
    false => "".to_string(),
}
}
