use askama::Template;
use crate::models::{data::{reference_items::Language, CategoryModel}, views::components::MenuTabComponent};

#[derive(Template)]
#[template(path="presenter/pages/restaurant.html")]
pub struct RestaurantPage<'a> {
    pub title: &'a str,
    pub blurb: &'a str,
    pub menu_page: MenuPage<'a>,
    pub languages: Vec<Language>,
}

#[derive(Template)]
#[template(path="presenter/pages/menu_page.html")]
pub struct MenuPage<'a> {
    pub title: &'a str,
    pub categories: Vec<CategoryModel>,
    pub menu_tabs: Vec<MenuTabComponent>
}
