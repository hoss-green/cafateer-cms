use super::context::AppState;
use crate::data_models::reference_items::{Allergy, Language};

pub async fn get_allergies(state: &AppState) -> Vec<Allergy> {
    let result = sqlx::query_as!(
        Allergy,
        "select id, lang, title, colour, icon from ref_allergens",
    )
    .fetch_all(&state.database_pool)
    .await;
    match result {
        Ok(res) => res,
        Err(_) => vec![],
    }
}

pub async fn get_languages(state: &AppState) -> Vec<Language> {
    let result = sqlx::query_as!(Language, "select id, code, name from ref_languages")
        .fetch_all(&state.database_pool)
        .await;
    match result {
        Ok(res) => res,
        Err(_) => vec![],
    }
}
