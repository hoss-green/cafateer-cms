use crate::data_context::context::{AppState, DatabasePool};
use crate::models::data::CategoryDetailsModel;

pub async fn get(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> CategoryDetailsModel {
    let result = sqlx::query_as!(
        CategoryDetailsModel,
        r#"select id, owner_id, published from menu_category_details where id = $1"#,
        id
    )
    .fetch_one(database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch category_detail, err: {}", err);
            CategoryDetailsModel::new(*id, *owner_id)
        }
    }
}

pub async fn set(
    database_pool: &DatabasePool,
    account_id: &uuid::Uuid,
    category_details_model: &CategoryDetailsModel,
) -> bool {
    let result = sqlx::query!(
        "insert into menu_category_details(id, owner_id, published)
            VALUES ($1, $2, $3) 
            ON CONFLICT (id) DO UPDATE SET published=$3 
            WHERE menu_category_details.id=$1 AND menu_category_details.owner_id=$2",
        category_details_model.id,
        account_id,
        category_details_model.published
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Saved details succesful");
            true
        }
        Err(err) => {
            println!("Cannot save menu item details fail, error: {}", err);
            false
        }
    }
}

pub async fn disable(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> bool {
    let result = sqlx::query!(
        "update menu_category_details
            SET published = false WHERE owner_id = $1 and id = $2 ",
        owner_id,
        id,
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Catagory Disabled successfully");
            true
        }
        Err(err) => {
            println!("Disable category, fail, error: {}", err);
            false
        }
    }
}

pub async fn enable(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> bool {
    let result = sqlx::query!(
        "update menu_category_details 
            SET published = true WHERE owner_id = $1 and id = $2 ",
        owner_id,
        id,
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Enabled category successfully");
            true
        }
        Err(err) => {
            println!("Enable category, fail, error: {}", err);
            false
        }
    }
}
