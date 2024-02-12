use askama::Template;

use crate::data_models::reference_items::Language;

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
