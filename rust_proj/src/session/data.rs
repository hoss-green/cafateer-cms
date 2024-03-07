use crate::data_context::context::DatabasePool;
use super::models::AccountModel;

pub async fn save_sign_up(database_pool: &DatabasePool, account_model: &AccountModel) -> bool {
    let result = sqlx::query!(
        "insert into accounts(id, email, email_normalised, password_hash, salt, sign_up, status) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        account_model.id,
        account_model.email,
        account_model.email_normalised,
        account_model.password_hash,
        account_model.salt,
        account_model.sign_up,
        account_model.status
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Registration succesful");
            true
        }
        Err(err) => {
            println!("Cannot register account, error: {}", err);
            false
        }
    }
}

pub async fn get_account_by_email(database_pool: &DatabasePool, email_normalised: &String) -> Option<AccountModel> {
    let result = sqlx::query_as!(AccountModel, 
        "select id, email, email_normalised, password_hash, salt, sign_up, status FROM accounts where email_normalised=$1",
   email_normalised).fetch_optional(database_pool).await;
    match result {
        Ok(am) => am,
        Err(err) => {
            println!("Could not execute query, error {}", err);
            None
        }
    }
}
