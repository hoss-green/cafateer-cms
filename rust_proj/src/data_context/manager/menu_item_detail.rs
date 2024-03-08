use crate::data_context::context::AppState;
use crate::models::data::MenuItemDetailsModel;
use sqlx::Postgres;

pub async fn get(
    app_state: &AppState,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> MenuItemDetailsModel {
    let result = sqlx::query_as::<Postgres, MenuItemDetailsModel>(
        r#"select id, price, category, allergies, owner_id from menu_item_details where id = $1"#,
    )
    .bind(id)
    .fetch_one(&app_state.database_pool)
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
    app_state: &AppState,
    account_id: &uuid::Uuid,
    menu_item_details_model: &MenuItemDetailsModel,
) -> bool {
    let result = sqlx::query(
        "insert into menu_item_details(id, owner_id, price, category, allergies)
            VALUES ($1, $2, $3, $4, $5) 
            ON CONFLICT (id) DO UPDATE SET price=$3, category=$4, allergies=$5 
            WHERE menu_item_details.id=$1 AND menu_item_details.owner_id=$2",
    )
    .bind(menu_item_details_model.id)
    .bind(account_id)
    .bind(menu_item_details_model.price)
    .bind(menu_item_details_model.category)
    .bind(menu_item_details_model.clone().allergies)
    .execute(&app_state.database_pool)
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
