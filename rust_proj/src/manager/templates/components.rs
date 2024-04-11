use crate::models::data::{reference_items::Allergy, CategoryModel};
use askama::Template;

use super::view_models::AccountLanguageVm;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/detail_editor.html")]
pub struct ComponentDetailEditorVm {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub lang_name: String,
    pub blurb: String,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/category_editor.html")]
pub struct ComponentCategoryEditorVm {
    pub id: uuid::Uuid,
    pub lang: i32,
    pub title: String,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/primary_language_list.html")]
pub struct PrimaryLanguageListVm {
    pub primary_language_id: i32,
    pub user_selected_languages: Vec<AccountLanguageVm>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/menu_item_editor.html")]
pub struct MenuItemEditorVm {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub lang: i32,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/components/menu_item_details_editor.html")]
pub struct MenuItemDetailsEditorVm {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub allergies: Vec<Allergy>,
    pub category: uuid::Uuid,
    pub categories: Vec<CategoryModel>,
    pub price: f64,
    pub published: bool
}
