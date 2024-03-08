use crate::{data_context::context::DatabasePool, models::data::ProfileLanguagesModel};

pub async fn get_all(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
) -> Vec<i32> {
    let result = sqlx::query_as!(
        ProfileLanguagesModel,
        r#"select id, language, owner_id, published from account_languages where owner_id=$1"#,
        owner_id
    )
    .fetch_all(database_pool)
    .await;
    match result {
        Ok(r) => r.iter().map(|language_model| language_model.language).collect::<Vec<i32>>(),
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            vec![]
        }
    }
}

pub async fn get_published(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
) -> Vec<i32> {
    let result = sqlx::query_as!(
        ProfileLanguagesModel,
        r#"select id, language, owner_id, published from account_languages where owner_id=$1 and published = true"#,
        owner_id
    )
    .fetch_all(database_pool)
    .await;
    match result {
        Ok(r) => r.iter().map(|language_model| language_model.language).collect::<Vec<i32>>(),
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            vec![]
        }
    }
}

pub async fn delete(database_pool: &DatabasePool, owner_id: uuid::Uuid, lang_id: i32) -> bool {
    let result = sqlx::query!(
        r#"delete from account_languages where owner_id=$1 AND language=$2"#,
        owner_id,
        lang_id
    )
    .execute(database_pool)
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

pub async fn add(
    database_pool: &DatabasePool,
    account_languages_model: &ProfileLanguagesModel,
) -> bool {
    let result = sqlx::query!(
        "insert into account_languages(owner_id, id, language)  VALUES ($1, $2, $3)",
        &account_languages_model.owner_id,
        account_languages_model.id,
        account_languages_model.language,
    )
    .execute(database_pool)
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

pub async fn set_account_details(
    database_pool: &DatabasePool,
    profile_languages_model: &ProfileLanguagesModel,
) -> bool {
    let result = sqlx::query!(
        "insert into profiles(id) VALUES ($1)",
        profile_languages_model.id
    )
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
