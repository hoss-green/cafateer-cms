use crate::{data::context::AppState, models::data::MenuItemModel};

pub async fn get_menu_items(app_state: &AppState) -> Vec<MenuItemModel> {
    let result = sqlx::query_as!(
        MenuItemModel,
        " select id, lang, title, description, price, owner_id from menu_items"
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
