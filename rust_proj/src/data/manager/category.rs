use crate::data::context::AppState;
use crate::models::data::CategoryModel;

pub async fn get(
    app_state: &AppState,
    (id, lang): (uuid::Uuid, i32),
    owner_id: &uuid::Uuid,
) -> CategoryModel {
    let result = sqlx::query_as!(
        CategoryModel,
        "select id, lang, title, owner_id, NULL as lang_name from menu_categories where id = $1 and lang = $2 and owner_id = $3",
        id,
        lang,
        owner_id
    )
    .fetch_optional(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => CategoryModel::new(Some(id), *owner_id),
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            CategoryModel::new(Some(id), *owner_id)
        }
    }
}

pub async fn set(
    app_state: &AppState,
    account_id: &uuid::Uuid,
    details_item: CategoryModel,
) -> bool {
    let result = sqlx::query!(
        "insert into menu_categories(owner_id, id, lang, title) VALUES ($1, $2, $3, $4) ON CONFLICT (id, lang) DO UPDATE SET title=$4 WHERE menu_categories.id=$2 and menu_categories.lang=$3",
        &account_id,
        details_item.id,
        details_item.lang,
        details_item.title,
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

pub async fn delete(
    app_state: &AppState,
    account_id: &uuid::Uuid,
    category_id: &uuid::Uuid,
) -> bool {
    let result = sqlx::query!(
        "delete from menu_categories where owner_id=$1 and id=$2",
        &account_id,
        &category_id,
    )
    .execute(&app_state.database_pool)
    .await;

    match result {
        Ok(r) => {
            if r.rows_affected() > 0 {
                println!("Deleted category succesfully {:?}", r);
                return true;
            }
            false
        }
        Err(err) => {
            println!("Cannot save item, fail, error: {}", err);
            false
        }
    }
}
