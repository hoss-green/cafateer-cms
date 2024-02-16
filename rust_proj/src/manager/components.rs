use crate::data_models::{reference_items::{Allergy, Language}, CategoryModel};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/detail_editor.html")]
pub struct ComponentDetailEditor {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub lang_name: String,
    pub blurb: String,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/category_editor.html")]
pub struct ComponentCategoryEditor {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub title: String,
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
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/menu_item_details_editor.html")]
pub struct MenuItemDetailsEditor {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub allergies: Vec<Allergy>,
    pub category: uuid::Uuid,
    pub categories: Vec<CategoryModel>,
}
