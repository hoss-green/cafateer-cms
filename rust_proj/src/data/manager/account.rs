use crate::data::context::AppState;
use crate::models::data::AccountModel;

pub async fn get(app_state: &AppState) -> AccountModel {
    let result = sqlx::query_as!(AccountModel, r#"select id, primary_language from accounts"#,)
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

pub async fn set(app_state: &AppState, account_model: &AccountModel) -> bool {
    let result = sqlx::query!(
        "insert into accounts(id, primary_language) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET primary_language=$2 WHERE accounts.id=$1", 
        account_model.id,
        account_model.primary_language)
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
