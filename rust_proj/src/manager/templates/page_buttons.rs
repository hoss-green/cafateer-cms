use askama::Template;

use crate::models::data::reference_items::Language;

#[derive(Template, Debug, Clone)]
#[template(path = "manager/page_buttons/menu_item_edit_button.html")]
pub struct MenuItemEditButton {
    pub id: uuid::Uuid,
    pub title: String,
    pub category: String,
    pub enabled: bool,
    pub languages: Vec<Language>
}
