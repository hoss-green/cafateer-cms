use askama::Template;
use serde::{Deserialize, Serialize};
#[derive(Debug, Template, Clone, Serialize, Deserialize)]
#[template(path = "presenter/components/menu_tab.html", ext = "html")]
pub struct MenuTabComponent {
    pub category: uuid::Uuid,
    pub name: String,
    pub menu_items: Vec<MenuItemComponent>,
}

#[derive(Debug, Template, Clone, Serialize, Deserialize)]
#[template(path = "presenter/components/menu_item.html")]
pub struct MenuItemComponent {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub category: uuid::Uuid,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DetailsViewModel {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub blurb: String,
    pub lang_code: String,
    pub lang_name: String,
}

#[derive(Template, Clone, Serialize, Deserialize)]
#[template(path = "presenter/components/menu_item.html")]
pub struct MenuItemEditorComponent {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub lang: i32,
    pub price: f64,
    pub category: String,
}
