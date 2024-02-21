use askama::Template;
use crate::models::views::components::MenuTabComponent;

#[derive(Template)]
#[template(path="presenter/pages/restaurant.html")]
pub struct RestaurantPage<'a> {
    pub title: &'a str,
    pub menu_tabs: Vec<MenuTabComponent<'a>>
}

#[derive(Template)]
#[template(path="presenter/pages/menu_page.html")]
pub struct MenuPage<'a> {
    pub title: &'a str,
    pub menu_tabs: Vec<MenuTabComponent<'a>>
}
