use crate::{data_context::context::DatabasePool, models::data::ProfileLanguagesModel};

pub async fn get_all(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
) -> Vec<ProfileLanguagesModel> {
    let result = sqlx::query_as!(
        ProfileLanguagesModel,
        r#"select id, language, owner_id, published from account_languages where owner_id=$1"#,
        owner_id
    )
    .fetch_all(database_pool)
    .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            vec![]
        }
    }
}

pub async fn get_all_ids(
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
        "insert into account_languages(owner_id, id, language, published)  VALUES ($1, $2, $3, $4)",
        &account_languages_model.owner_id,
        account_languages_model.id,
        account_languages_model.language,
        account_languages_model.published
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

pub async fn disable(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> bool {
    let result = sqlx::query!(
        "update account_languages
            SET published = false WHERE owner_id = $1 and id = $2 ",
        owner_id,
        id,
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Language Disabled successfully");
            true
        }
        Err(err) => {
            println!("Disable language, fail, error: {}", err);
            false
        }
    }
}

pub async fn enable(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
    id: &uuid::Uuid,
) -> bool {
    let result = sqlx::query!(
        "update account_languages
            SET published = true WHERE owner_id = $1 and id = $2 ",
        owner_id,
        id,
    )
    .execute(database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Enabled language successfully");
            true
        }
        Err(err) => {
            println!("Enable language, fail, error: {}", err);
            false
        }
    }
}
