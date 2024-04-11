use crate::data_context::context::AppState;
use crate::models::data::MenuItemDetailsModel;
use sqlx::Postgres;

pub async fn get_all(
    app_state: &AppState,
    owner_id: &uuid::Uuid,
) -> Vec<MenuItemDetailsModel> {
    let result = sqlx::query_as::<Postgres, MenuItemDetailsModel>(
        r#"select id, price, category, allergies, owner_id, published from menu_item_details where owner_id = $1"#,
    )
    .bind(owner_id)
    .fetch_all(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu_item_detail list, err: {}", err);
            vec![]
        }
    }
}

