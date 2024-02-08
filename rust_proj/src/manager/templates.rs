use askama::Template;

use crate::data_models::BioItem;

#[derive(Template)]
#[template(path="manager/main_page.html")]
pub struct ManagerHomePage<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path="manager/details_page.html")]
pub struct DetailsPage<'a> {
    pub title: &'a str,
    pub id: uuid::Uuid,
    pub lang: String,
    pub name: String, 
    pub info: String
}
