use crate::{data_context::context::DatabasePool, models::data::MenuItemModel};

pub async fn get(
    database_pool: &DatabasePool,
    id: uuid::Uuid,
    lang: i32,
    owner_id: uuid::Uuid,
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
        Ok(r) => {
            if let Some(item) = r {
                return item;
            }
        }
        Err(err) => {
            println!("Cannot fetch menu item, err: {}", err);
        }
    }
    MenuItemModel::new(uuid::Uuid::new_v4(), owner_id, lang)
}

pub async fn exists(database_pool: &DatabasePool, id: &uuid::Uuid, owner_id: &uuid::Uuid) -> bool {
    match sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, owner_id from menu_items where id=$1 and owner_id=$2",
        owner_id,
        id,
    )
    .fetch_optional(database_pool)
    .await
    {
        Ok(menu_item_model) => menu_item_model.is_some(),
        Err(_) => false,
    }
}

pub async fn set(
    database_pool: &DatabasePool,
    account_id: &uuid::Uuid,
    details_item: MenuItemModel,
) -> Option<MenuItemModel> {
    let result = sqlx::query!(
        "insert into menu_items(owner_id, id, lang, title, description) 
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id, lang) DO UPDATE SET title=$4, description=$5
            WHERE menu_items.id=$2 and menu_items.lang=$3",
        &account_id,
        details_item.id,
        details_item.lang,
        details_item.title,
        details_item.description,
    )
    .execute(database_pool)
    .await;
    match result {
        Ok(_r) => {
            println!("Saved item succesfully");
            Some(details_item.clone())
        }
        Err(err) => {
            println!("Cannot save item, fail, error: {}", err);
            None
        }
    }
}

pub async fn create(
    database_pool: &DatabasePool,
    account_id: &uuid::Uuid,
    menu_item_model: &MenuItemModel,
) -> bool {
    let mut tx = database_pool
        .begin()
        .await
        .expect("Could not create transaction");
    let item_add = sqlx::query!(
        "insert into menu_items(owner_id, id, lang, title, description) VALUES ($1, $2, $3, $4, $5)",
        &account_id,
        menu_item_model.id,
        menu_item_model.lang,
        menu_item_model.title,
        menu_item_model.description,
    )
    .execute(&mut *tx)
    .await;

    let detail_add = sqlx::query!(
        "insert into menu_item_details(id, owner_id, published)
            VALUES ($1, $2, false) ",
        menu_item_model.id,
        account_id,
    )
    .execute(&mut *tx)
    .await;

    if item_add.is_err() || detail_add.is_err() {
        let _ = tx.rollback().await;
        false
    } else {
        let result = tx.commit().await;
        result.is_ok()
    }
}

pub async fn delete(database_pool: &DatabasePool, owner_id: &uuid::Uuid, id: &uuid::Uuid) -> bool {
    let mut tx = database_pool
        .begin()
        .await
        .expect("Could not create transaction");
    let item_delete = sqlx::query!(
        "delete from menu_items where id=$1 and owner_id=$2 RETURNING *",
        &id,
        &owner_id,
    )
    .fetch_all(&mut *tx)
    .await;
    let detail_delete = sqlx::query!(
        "delete from menu_item_details where id=$1 and owner_id=$2 RETURNING *",
        &id,
        &owner_id,
    )
    .fetch_all(&mut *tx)
    .await;

    if !matches!(item_delete, Ok(item_delete_count) if !item_delete_count.is_empty())
        || !matches!(detail_delete, Ok(detail_delete_count) if !detail_delete_count.is_empty())
    {
        //this is bad
        let _ = tx.rollback().await;
        false
    } else {
        match tx.commit().await {
            Ok(_) => true,
            Err(err) => {
                println!("Cannot delete menu item, fail, error: {}", err);
                false
            }
        }
    }
}

pub async fn get_by_lang(
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
