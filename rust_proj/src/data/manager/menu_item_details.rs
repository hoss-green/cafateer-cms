use sqlx::Postgres;
use crate::models::data::MenuItemDetailsModel;
use crate::data::context::AppState;

pub async fn get_menu_item_details(app_state: &AppState, id:uuid::Uuid) -> MenuItemDetailsModel {
    let result = sqlx::query_as::<Postgres, MenuItemDetailsModel>(
        r#"select id, category, allergies from menu_item_details"#,
    )
    .bind(id)
    .fetch_one(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu_item_detail, err: {}", err);
            MenuItemDetailsModel::new()
        }
    }
}

// pub async fn set_account_details(app_state: &AppState, menu_item_details_model: &MenuItemDetailsModel) -> bool {
//     let result = sqlx::query(
//         "insert into menu_item_details(id, categories, allergies) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET category=$2, allergies=$3 WHERE menu_item_details.id=$1")
//         .bind(menu_item_details_model.id)
//         .bind(menu_item_details_model.clone().category)
//         .bind(menu_item_details_model.clone().allergies)
//     .execute(&app_state.database_pool)
//     .await;
//
//     match result {
//         Ok(_r) => {
//             println!("Saved menu_item_details succesful");
//             true
//         }
//         Err(err) => {
//             println!("Cannot save menu_item_details fail, error: {}", err);
//             false
//         }
//     }
// }
//
//
