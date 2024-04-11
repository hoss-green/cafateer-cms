use crate::data_context::context::DatabasePool;
use crate::models::data::MenuItemDetailsModel;
use sqlx::Postgres;

pub async fn get(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> MenuItemDetailsModel {
    let result = sqlx::query_as::<Postgres, MenuItemDetailsModel>(
        r#"select id, price, category, allergies, owner_id, published from menu_item_details where id = $1"#,
    )
    .bind(id)
    .fetch_one(database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu_item_detail, err: {}", err);
            MenuItemDetailsModel::new(*id, *owner_id)
        }
    }
}

pub async fn set(
    database_pool: &DatabasePool,
    account_id: &uuid::Uuid,
    menu_item_details_model: &MenuItemDetailsModel,
) -> bool {
    let result = sqlx::query(
        "insert into menu_item_details(id, owner_id, price, category, allergies, published)
            VALUES ($1, $2, $3, $4, $5, $6) 
            ON CONFLICT (id) DO UPDATE SET price=$3, category=$4, allergies=$5, published=$6
            WHERE menu_item_details.id=$1 AND menu_item_details.owner_id=$2",
    )
    .bind(menu_item_details_model.id)
    .bind(account_id)
    .bind(menu_item_details_model.price)
    .bind(menu_item_details_model.category)
    .bind(menu_item_details_model.clone().allergies)
    .bind(menu_item_details_model.published)
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
        "update menu_item_details 
            SET published = false WHERE owner_id = $1 and id = $2 ",
        owner_id,
        id,
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("MenuItem Disabled successfully");
            true
        }
        Err(err) => {
            println!("Disable MenuItem, fail, error: {}", err);
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
        "update menu_item_details 
            SET published = true WHERE owner_id = $1 and id = $2 ",
        owner_id,
        id,
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("MenuItem Enabled successfully");
            true
        }
        Err(err) => {
            println!("Enable MenuItem, fail, error: {}", err);
            false
        }
    }
}
