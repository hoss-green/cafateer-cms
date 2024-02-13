use askama::Template;

use crate::data_models::{reference_items::Language, MenuItem};

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/component_edit_details.html")]
pub struct ComponentEditDetails {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub lang_name: String,
    pub blurb: String,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/primary_language_list.html")]
pub struct PrimaryLanguageList {
    pub primary_language_id: i32,
    pub user_selected_languages: Vec<Language>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/menu_item_editor.html")]
pub struct MenuItemEditor {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub lang: i32,
    pub price: f64,
    pub category: String,
}
