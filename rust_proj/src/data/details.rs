use crate::data_models::{DetailLangModel, DetailsModel};

use super::context::AppState;

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

pub async fn set_details(app_state: &AppState, details_item: DetailsModel) -> bool {
    println!("setting details");
    let result = sqlx::query!(
        "insert into details(id, lang, blurb) VALUES ($1, $2, $3) ON CONFLICT (id, lang) DO UPDATE SET blurb=$3 WHERE details.id=$1 and details.lang=$2",
        details_item.id,
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
