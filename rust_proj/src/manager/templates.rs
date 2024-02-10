use askama::Template;

use crate::{data_models::reference_items::Language, view_models::components::DetailsViewModel};

#[derive(Template)]
#[template(path = "manager/start_page.html")]
pub struct StartPage<'a> {
    pub title: &'a str,
}


#[derive(Template)]
#[template(path = "manager/account_page.html")]
pub struct AccountPage<'a> {
    pub title: &'a str,
    pub language: Language,
    pub available_languages: Vec<Language>,
    pub selected_languages: Vec<Language>
}


#[derive(Template)]
#[template(path = "manager/details_page.html")]
pub struct DetailsPage<'a> {
    pub title: &'a str,
    pub languages: Vec<Language>,
    pub details: Vec<DetailsViewModel>
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
