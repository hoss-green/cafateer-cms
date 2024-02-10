
use sqlx::Postgres;

use crate::data_models::AccountModel;

use super::context::AppState;

pub async fn get_account_details(app_state: &AppState) -> AccountModel {
    let result = sqlx::query_as::<Postgres, AccountModel>( r#"select id, languages as "languages!" from accounts"#)
        .fetch_one(&app_state.database_pool)
        .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            AccountModel::new()
        }
    }
}

// pub async fn set_details(app_state: &AppState, account_model: AccountModel) -> bool {
//     println!("setting details");
//     let result = sqlx::query!(
//         "insert into accounts(id, languages) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET languages=$2 WHERE accounts.id=$1",
//         account_model.id,
//         // account_model.main_language,
//         account_model.languages,
//     )
//     .execute(&app_state.database_pool)
//     .await;
//
//     match result {
//         Ok(_r) => {
//             println!("Saved item succesfully");
//             true
//         }
//         Err(err) => {
//             println!("Cannot save item, fail, error: {}", err);
//             false
//         }
//     }
// }
