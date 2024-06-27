use crate::{data_context::context::DatabasePool, models::data::MenuItemModel};

pub async fn get_for_account(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
) -> Vec<MenuItemModel> {
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, owner_id from menu_items where owner_id=$1",
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

pub async fn get_by_id(
    database_pool: &DatabasePool,
    menu_item_id: &uuid::Uuid,
) -> Vec<MenuItemModel> {
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, owner_id from menu_items where id=$1",
        menu_item_id
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

pub async fn get_items_by_lang(database_pool: &DatabasePool, lang: i32) -> Vec<MenuItemModel> {
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, owner_id from menu_items where lang=$1",
        lang
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
