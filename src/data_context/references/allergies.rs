use crate::{data_context::context::DatabasePool, models::data::reference_items::Allergy};

pub async fn get_allergies(database_pool: &DatabasePool) -> Vec<Allergy> {
    let result = sqlx::query_as!(
        Allergy,
        "select id, lang, title, colour, icon from ref_allergens",
    )
    .fetch_all(database_pool)
    .await;
    match result {
        Ok(res) => res,
        Err(_) => vec![],
    }
}
