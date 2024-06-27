use crate::data_context::context::AppState;
use crate::models::data::CategoryDetailsModel;

pub async fn get_all(app_state: &AppState, owner_id: &uuid::Uuid) -> Vec<CategoryDetailsModel> {
    let result = sqlx::query_as!(
        CategoryDetailsModel,
        r#"select id,  owner_id, published from menu_category_details where owner_id = $1"#,
        owner_id
    )
    .fetch_all(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch category details, err: {}", err);
            vec![]
        }
    }
}
