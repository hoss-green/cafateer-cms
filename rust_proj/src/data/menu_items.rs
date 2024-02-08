use crate::data_models::MenuItem;

use super::context::AppState;

pub async fn get_details(app_state: &AppState) -> MenuItem {
    let result = sqlx::query_as!(MenuItem, " select id, title, description, price, category from menu_items")
        .fetch_optional(&app_state.database_pool)
        .await;

    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => MenuItem::new()
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            MenuItem::new()
        }
    }
}
