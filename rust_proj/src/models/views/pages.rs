use askama::Template;
use crate::models::views::components::MenuTabComponent;

#[derive(Template)]
#[template(path="presenter/pages/restaurant.html")]
pub struct RestaurantPage<'a> {
    pub title: &'a str,
    pub menu_tabs: Vec<MenuTabComponent<'a>>
}
