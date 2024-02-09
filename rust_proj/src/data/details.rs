use crate::data_models::DetailsItem;

use super::context::AppState;

pub async fn get_details(app_state: &AppState) -> DetailsItem {
    let result = sqlx::query_as!(DetailsItem, r#"select id, lang, blurb from details"#)
        .fetch_optional(&app_state.database_pool)
        .await;

    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => DetailsItem::new()
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            DetailsItem::new()
        }
    }
}

pub async fn set_details(app_state: &AppState, details_item: DetailsItem) -> bool {
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
