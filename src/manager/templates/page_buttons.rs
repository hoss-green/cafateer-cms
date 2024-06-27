use askama::Template;

use super::view_models::DropDownLanguageVm;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/page_buttons/menu_item_edit_button.html")]
pub struct MenuItemEditButton {
    pub id: uuid::Uuid,
    pub title: String,
    pub category: String,
    pub languages: Vec<DropDownLanguageVm>,
    pub enabled: bool,
}

#[derive(Template, Debug, Clone)]
#[template(path = "manager/page_buttons/category_edit_button.html")]
pub struct CategoryItemEditButton {
    pub id: uuid::Uuid,
    pub title: String,
    pub languages: Vec<DropDownLanguageVm>,
    pub enabled: bool,
}
