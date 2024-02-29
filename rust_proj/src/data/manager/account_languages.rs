use crate::{data::context::AppState, models::data::AccountLanguagesModel};

pub async fn get_all(app_state: &AppState, owner_id: uuid::Uuid) -> Vec<AccountLanguagesModel> {
    let result = sqlx::query_as!(
        AccountLanguagesModel,
        r#"select id, language, owner_id from account_languages where owner_id=$1"#,
        owner_id
    )
    .fetch_all(&app_state.database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            vec![]
        }
    }
}

pub async fn delete(app_state: &AppState, owner_id: uuid::Uuid, lang_id: i32) -> bool {
    let result = sqlx::query!(r#"delete from account_languages where owner_id=$1 AND language=$2"#, owner_id, lang_id)
        .execute(&app_state.database_pool)
        .await;
    match result {
        Ok(r) => {
            if r.rows_affected() > 0 {
                println!("Deleted language succesfully {:?}", r);
                return true;
            }
            false
        }
        Err(err) => {
            println!("Cannot delete language, fail, error: {}", err);
            false
        }
    }
}

pub async fn add(app_state: &AppState, account_languages_model: &AccountLanguagesModel) -> bool {
    let result = sqlx::query!(
        "insert into account_languages(owner_id, id, language)  VALUES ($1, $2, $3)",
        &account_languages_model.owner_id,
        account_languages_model.id,
        account_languages_model.language,
    )
    .execute(&app_state.database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Added language successfully");
            true
        }
        Err(err) => {
            println!("Cannot add language, fail, error: {}", err);
            false
        }
    }
}
// pub async fn get_primary(
//     app_state: &AppState,
//     owner_id: uuid::Uuid,
// ) -> AccountLanguagesModel {
//     let result = sqlx::query_as!(
//         AccountLanguagesModel,
//         r#"select id, language, owner_id from account_languages where owner_id=$1 and is_primary"#,
//         owner_id
//     )
//     .fetch_one(&app_state.database_pool)
//     .await;
//     match result {
//         Ok(r) => r,
//         Err(err) => {
//             println!("Cannot fetch account, err: {}", err);
//             AccountLanguagesModel::new(owner_id)
//         }
//     }
// }

pub async fn set_account_details(
    app_state: &AppState,
    account_languages_model: &AccountLanguagesModel,
) -> bool {
    let result = sqlx::query!(
        // "insert into accounts(id, languages) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET languages=$2 WHERE accounts.id=$1", account_model.id)
        "insert into accounts(id) VALUES ($1)",
        account_languages_model.id
    )
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
