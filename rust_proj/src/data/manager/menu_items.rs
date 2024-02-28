use crate::{data::context::AppState, models::data::MenuItemModel};

pub async fn get_items_for_account(app_state: &AppState) -> Vec<MenuItemModel> {
    let account = crate::data::manager::account::get_account_details(app_state).await;
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, price,  owner_id from menu_items where owner_id=$1",
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
        "select id, lang, title, description, price, owner_id from menu_items where lang=$1",
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

pub async fn get_item_by_lang(app_state: &AppState, id:uuid::Uuid, lang: i32, owner_id:uuid::Uuid) -> MenuItemModel {
    let account = crate::data::manager::account::get_account_details(app_state).await;
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, price, owner_id from menu_items where id=$1 and lang=$2 and owner_id=$3",
        id,
        lang,
        owner_id
    )
    .fetch_optional(&app_state.database_pool)
    .await;

    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => 
            {
            println!("Cannot find menu item");
                MenuItemModel::new(account.id)
            }
        },
        Err(err) => {
            println!("Cannot fetch menu item, err: {}", err);
            MenuItemModel::new(account.id)
        }
    }
}

pub async fn set_item(
    app_state: &AppState,
    account_id: &uuid::Uuid,
    details_item: MenuItemModel,
) -> bool {
    let result = sqlx::query!(
        "insert into menu_items(owner_id, id, lang, title, description, price) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id, lang) DO UPDATE SET title=$4, description=$5, price=$6 WHERE menu_items.id=$2 and menu_items.lang=$3",
        &account_id,
        details_item.id,
        details_item.lang,
        details_item.title,
        details_item.description,
        details_item.price,
    )
    .execute(&app_state.database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Saved item succesfully");
            true
        }
        Err(err) => {
            println!("Cannot save item, fail, error: {}", err);
            false
        }
    }
}



