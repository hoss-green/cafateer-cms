use crate::data_context::context::AppState;
use crate::models::data::CategoryDetailsModel;
use sqlx::Postgres;

pub async fn get(
    app_state: &AppState,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> CategoryDetailsModel {
    let result = sqlx::query_as!(
        CategoryDetailsModel,
        r#"select id, owner_id, published from menu_category_details where id = $1"#,
        id
    )
    .fetch_one(&app_state.database_pool)
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
    app_state: &AppState,
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
