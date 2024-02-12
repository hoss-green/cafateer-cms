use crate::data_models::{DetailLangModel, DetailsModel};

use super::context::AppState;

pub async fn get_detail(app_state: &AppState, account_id: &uuid::Uuid, id: i32) -> DetailLangModel {
    let result = sqlx::query_as!(DetailLangModel, "select details.id, details.lang, details.blurb, ref_languages.code as lang_code, ref_languages.name as lang_name from details join ref_languages on details.lang = ref_languages.id where details.id = $1 AND ref_languages.id = $2", account_id,id)
        .fetch_optional(&app_state.database_pool)
        .await;
    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => DetailLangModel::new(*account_id, id, None, "".to_string(), "".to_string()),
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            DetailLangModel::new(*account_id, id, None, "".to_string(), "".to_string())
        }
    }
}
pub async fn get_details_list(app_state: &AppState) -> Vec<DetailLangModel> {
    let result = sqlx::query_as!(DetailLangModel, "select details.id, details.lang, details.blurb, ref_languages.code as lang_code, ref_languages.name as lang_name from details join ref_languages on details.lang = ref_languages.id")
        .fetch_all(&app_state.database_pool)
        .await;
    match result {
        Ok(r) => r,
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            vec![]
        }
    }
}

pub async fn set_details(app_state: &AppState, account_id:&uuid::Uuid, details_item: DetailsModel) -> bool {
    let result = sqlx::query!(
        "insert into details(id, lang, blurb) VALUES ($1, $2, $3) ON CONFLICT (id, lang) DO UPDATE SET blurb=$3 WHERE details.id=$1 and details.lang=$2",
        &account_id,
        details_item.lang,
        details_item.blurb,
    )
    .execute(&app_state.database_pool)
    .await;

    match result {
        Ok(_r) => {
            println!("Saved item succesfully");
            true
        }
        Err(err) => {
            println!("Cannot save item, fail, error: {}", err);
            false
        }
    }
}
