use crate::data_models::MenuItemModel;
use super::context::AppState;

pub async fn get_items_for_account(app_state: &AppState) -> Vec<MenuItemModel> {
    let account = crate::data::account::get_account_details(app_state).await;
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, price, category, owner_id from menu_items where owner_id=$1",
        account.id 
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

pub async fn get_items_by_lang(app_state: &AppState, lang: i32) -> Vec<MenuItemModel> {
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, price, category, owner_id from menu_items where lang=$1",
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

pub async fn get_item(app_state: &AppState, id:uuid::Uuid, lang: i32) -> MenuItemModel {
    let account = crate::data::account::get_account_details(app_state).await;
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, price, category, owner_id from menu_items where id=$1 and lang=$2",
        id,
        lang
    )
    .fetch_optional(&app_state.database_pool)
    .await;

    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => MenuItemModel::new(account.id),
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            MenuItemModel::new(account.id)
        }
    }
}
