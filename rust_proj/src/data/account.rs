use super::context::AppState;
use crate::data_models::AccountModel;
use sqlx::Postgres;

pub async fn get_account_details(app_state: &AppState) -> AccountModel {
    let result = sqlx::query_as::<Postgres, AccountModel>(
        r#"select id, languages from accounts"#,
    )
    .fetch_one(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            AccountModel::new()
        }
    }
}

pub async fn set_account_details(app_state: &AppState, account_model: &AccountModel) -> bool {
    let result = sqlx::query(
        "insert into accounts(id, languages) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET languages=$2 WHERE accounts.id=$1")
        .bind(account_model.id)
        .bind(account_model.clone().languages)
    .execute(&app_state.database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Saved account succesful");
            true
        }
        Err(err) => {
            println!("Cannot save account fail, error: {}", err);
            false
        }
    }
}

// pub async fn set_language(app_state: &AppState, language_id:i32, activated:bool)
// {
//     
// }
