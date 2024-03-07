use crate::{data_context::context::AppState, models::data::reference_items::Language};

pub async fn get_languages(state: &AppState) -> Vec<Language> {
    let result = sqlx::query_as!(Language, "select id, code, name from ref_languages")
        .fetch_all(&state.database_pool)
        .await;
    match result {
        Ok(res) => res,
        Err(_) => vec![],
    }
}
