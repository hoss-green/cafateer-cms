use crate::data_context::context::DatabasePool;
use crate::models::data::CategoryModel;

pub async fn get_category_list(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
) -> Vec<CategoryModel> {
    let result = sqlx::query_as!(
        CategoryModel,
        "select id, lang, title, owner_id, NULL as lang_name from menu_categories where owner_id = $1",
        owner_id
    )
    .fetch_all(database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            vec![]
        }
    }
}
