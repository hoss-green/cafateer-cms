use crate::{data_context::context::DatabasePool, models::data::MenuItemModel};

pub async fn get_items_for_account(
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

pub async fn get_item_by_lang(
    database_pool: &DatabasePool,
    id: &uuid::Uuid,
    lang: i32,
    owner_id: &uuid::Uuid,
) -> MenuItemModel {
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, owner_id from menu_items where id=$1 and lang=$2 and owner_id=$3",
        id,
        lang,
        owner_id
    )
    .fetch_optional(database_pool)
    .await;

    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => {
                println!("Cannot find menu item");
                MenuItemModel::new(*id, *owner_id, lang)
            }
        },
        Err(err) => {
            println!("Cannot fetch menu item, err: {}", err);
                MenuItemModel::new(*id, *owner_id, lang)
        }
    }
}
