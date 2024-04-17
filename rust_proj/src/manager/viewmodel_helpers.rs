use super::templates::view_models::DropDownLanguageVm;
use crate::data_context::{self, context::DatabasePool};

pub async fn get_dropdown_language_vms(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
) -> Vec<DropDownLanguageVm> {
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(database_pool, owner_id).await;
    let all_languages = &data_context::references::get_languages(database_pool).await;
    all_languages
        .iter()
        .map(|lang| DropDownLanguageVm {
            id: lang.id,
            name: lang.name.clone(),
            published: account_languages
                .iter()
                .any(|ac_lang| ac_lang.language == lang.id && ac_lang.published),
        })
        .collect()
}
