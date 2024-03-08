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

pub async fn set(
    database_pool: &DatabasePool,
    account_id: &uuid::Uuid,
    details_item: MenuItemModel,
) -> bool {
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
            true
        }
        Err(err) => {
            println!("Cannot save item, fail, error: {}", err);
            false
        }
    }
}

pub async fn delete(database_pool: &DatabasePool, owner_id: &uuid::Uuid, id: &uuid::Uuid) -> bool {
    let result = sqlx::query!(
        "delete from menu_items where id=$1 and owner_id=$2",
        &id,
        &owner_id,
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(r) => {
            if r.rows_affected() > 0 {
                println!("Deleted menu_item succesfully {:?}", r);
                return true;
            }
            false
        }
        Err(err) => {
            println!("Cannot delete menu item, fail, error: {}", err);
            false
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
