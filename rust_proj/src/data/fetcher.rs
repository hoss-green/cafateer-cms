use crate::data_models::MenuItem;

use super::context::AppState;

pub async fn get_menu_items(app_state: &AppState) -> Vec<MenuItem> {
    let result = sqlx::query_as!(
        MenuItem,
        " select id, title, description, price, category from menu_items"
    )
    .fetch_all(&app_state.database_pool)
    .await;

    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            vec![]
        }
    }
}
