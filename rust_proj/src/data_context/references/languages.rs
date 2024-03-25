use crate::{data_context::context::DatabasePool, models::data::reference_items::Language};

pub async fn get_languages(database_pool: &DatabasePool) -> Vec<Language> {
    let result = sqlx::query_as!(Language, "select id, code, name from ref_languages")
        .fetch_all(database_pool)
        .await;
    match result {
        Ok(res) => res,
        Err(_) => vec![],
    }
}

pub async fn get_language(database_pool: &DatabasePool, code:String) -> Option<Language> {
    let lang_code = code.to_lowercase();
    let result = sqlx::query_as!(Language, "select id, code, name from ref_languages where code = $1", lang_code)
        .fetch_optional(database_pool)
        .await;
    match result {
        Ok(res) => res,
        Err(_) => None,
    }
}
