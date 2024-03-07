use crate::data_context::context::{AppState, DatabasePool};
use crate::models::data::ProfileModel;

pub async fn get(database_pool: &DatabasePool) -> ProfileModel {
    let result = sqlx::query_as!(ProfileModel, r#"select id, primary_language from profiles"#,)
        .fetch_one(database_pool)
        .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            ProfileModel::new()
        }
    }
}

pub async fn set(database_pool: &DatabasePool, account_model: &ProfileModel) -> bool {
    let result = sqlx::query!(
        "insert into profiles(id, primary_language) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET primary_language=$2 WHERE profiles.id=$1", 
        account_model.id,
        account_model.primary_language)
    .execute(database_pool)
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
