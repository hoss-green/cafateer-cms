use crate::{data_context::context::DatabasePool, models::data::CategoryModel};

pub async fn get(
    database_pool: &DatabasePool,
    (id, lang): (uuid::Uuid, i32),
    owner_id: &uuid::Uuid,
) -> CategoryModel {
    let result = sqlx::query_as!(
        CategoryModel,
        "select id, lang, owner_id, title, NULL as lang_name from menu_categories where id = $1 and lang = $2",
        id,
        lang,
    )
    .fetch_optional(database_pool)
    .await;
    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => CategoryModel::new(Some(id), owner_id),
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            CategoryModel::new(Some(id), owner_id)
        }
    }
}

pub async fn exists(
    database_pool: &DatabasePool,
    id: &uuid::Uuid,
    owner_id: &uuid::Uuid,
) -> bool {
    match sqlx::query_as!(
        CategoryModel,
        "select id, lang, owner_id, title, NULL as lang_name from menu_categories where id = $1 and owner_id = $2",
        id,
        owner_id,
    )
    .fetch_optional(database_pool)
    .await
    {
        Ok(category_item) => category_item.is_some(),
        Err(_) => false,
    }
}

pub async fn set(
    database_pool: &DatabasePool,
    account_id: &uuid::Uuid,
    category_item: &CategoryModel,
) -> bool {
    let result = sqlx::query!(
        "insert into menu_categories(owner_id, id, lang, title) VALUES ($1, $2, $3, $4) ON CONFLICT (id, lang) DO UPDATE SET title=$4 WHERE menu_categories.id=$2 and menu_categories.lang=$3",
        &account_id,
        category_item.id,
        category_item.lang,
        category_item.title,
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

pub async fn create(
    database_pool: &DatabasePool,
    account_id: &uuid::Uuid,
    category_item: &CategoryModel,
) -> bool {
    let mut tx = database_pool
        .begin()
        .await
        .expect("Could not create category");
    let item_create = sqlx::query_as!(
        CategoryModel,
        "insert into menu_categories(owner_id, id, lang, title) VALUES ($1, $2, $3, $4)",
        &account_id,
        category_item.id,
        category_item.lang,
        category_item.title,
    )
    .execute(&mut *tx)
    .await;

    let details_create = sqlx::query_as!(
        CategoryModel,
        "insert into menu_category_details(owner_id, id, published) VALUES ($1, $2, $3)",
        &account_id,
        category_item.id,
        false
    )
    .execute(&mut *tx)
    .await;

    if item_create.is_err() || details_create.is_err() {
        let _ = tx.rollback().await;
        false
    } else {
        let tx_result = tx.commit().await;
        tx_result.is_ok()
    }
}

pub async fn delete(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
    category_id: &uuid::Uuid,
) -> bool {
    let mut tx = database_pool
        .begin()
        .await
        .expect("Could not create category");
    let item_delete = sqlx::query!(
        "delete from menu_categories where owner_id=$1 and id=$2 RETURNING *",
        &owner_id,
        &category_id,
    )
    .fetch_all(&mut *tx)
    .await;
    let detail_delete = sqlx::query!(
        "delete from menu_category_details where owner_id=$1 and id=$2 RETURNING *",
        &owner_id,
        &category_id,
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
                println!("Cannot delete category item, fail, error: {}", err);
                false
            }
        }
    }
}
