use crate::{data_context::context::AppState, models::data::reference_items::Allergy};

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
