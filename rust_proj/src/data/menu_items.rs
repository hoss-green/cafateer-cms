use crate::data_models::MenuItem;
use super::context::AppState;

pub async fn get_items_by_id(app_state: &AppState, id: uuid::Uuid) -> Vec<MenuItem> {
    let result = sqlx::query_as!(
        MenuItem,
        "select id, lang, title, description, price, category from menu_items where id=$1",
        id
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

pub async fn get_items_by_lang(app_state: &AppState, lang: String) -> Vec<MenuItem> {
    let result = sqlx::query_as!(
        MenuItem,
        "select id, lang, title, description, price, category from menu_items where lang=$1",
        lang
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

pub async fn get_item(app_state: &AppState, id:uuid::Uuid, lang: String) -> MenuItem {
    let result = sqlx::query_as!(
        MenuItem,
        "select id, lang, title, description, price, category from menu_items where id=$1 and lang=$2",
        id,
        lang
    )
    .fetch_optional(&app_state.database_pool)
    .await;

    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => MenuItem::new(),
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            MenuItem::new()
        }
    }
}
