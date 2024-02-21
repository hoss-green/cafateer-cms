use sqlx::Postgres;
use crate::models::data::MenuItemDetailsModel;
use crate::data::context::AppState;

pub async fn get_menu_item_detail(app_state: &AppState, owner_id:&uuid::Uuid, id:&uuid::Uuid) -> MenuItemDetailsModel {
    let result = sqlx::query_as::<Postgres, MenuItemDetailsModel>(
        r#"select id, category, allergies, owner_id from menu_item_details"#,
    )
    .bind(id)
    .fetch_one(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu_item_detail, err: {}", err);
            MenuItemDetailsModel::new(*owner_id)
        }
    }
}

pub async fn get_menu_item_details(app_state: &AppState, id:&uuid::Uuid) -> Vec<MenuItemDetailsModel> {
    let result = sqlx::query_as::<Postgres, MenuItemDetailsModel>(
        r#"select id, category, allergies, owner_id from menu_item_details"#,
    )
    .bind(id)
    .fetch_all(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu_item_detail, err: {}", err);
            // MenuItemDetailsModel::new()
            vec![]
        }
    }
}
