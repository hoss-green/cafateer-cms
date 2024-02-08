use crate::data_models::BioItem;

use super::context::AppState;

pub async fn get_details(app_state: &AppState) -> BioItem {
    let result = sqlx::query_as!(BioItem, " select id, lang, name, info from bios")
        .fetch_optional(&app_state.database_pool)
        .await;

    match result {
        Ok(r) => match r {
            Some(item) => item,
            None => BioItem::new()
        },
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            BioItem::new()
        }
    }
}

pub async fn set_details(app_state: &AppState, bio_item: BioItem) -> bool {
    // nsert into bios (id, lang, name, info) VALUES ('85dc08f4-a12e-4a84-9a05-8385ff096fe1', 'en', 'billy', 'info') ON CONFLICT (id) DO UPDATE SET lang='dddd', name='eklj' WHERE bios.id='85dc08f4-a12e-4a84-9a05-8385ff096fe1';
    println!("setting details");
    let result = sqlx::query!(
        "insert into bios(id, lang, name, info) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET lang=$2, name=$3, info=$4 WHERE bios.id=$1",
        bio_item.id,
        bio_item.lang,
        bio_item.name,
        bio_item.info
    )
    .execute(&app_state.database_pool)
    .await;

    match result {
        Ok(r) => {
            println!("Saved item succesfully");
            true
        }
        Err(err) => {
            println!("Cannot save item, fail");
            false
        }
    }
}
