use crate::{data::context::AppState, models::data::{CategoryModel, DetailsModel, MenuItemModel}};

pub async fn get_details(app_state: &AppState, lang: i32) -> DetailsModel {
    let result = sqlx::query_as!(
        DetailsModel,
        "select id, lang, blurb from details where lang=$1",
        lang
    )
    .fetch_optional(&app_state.database_pool)
    .await;

    match result {
        Ok(r) => r.unwrap_or(DetailsModel::default()),
        Err(err) => {
            println!("Cannot fetch menu items, err: {}", err);
            DetailsModel::default()
        }
    }
}

pub async fn get_categories(app_state: &AppState, lang: i32) -> Vec<CategoryModel> {
    let result = sqlx::query_as!(
        CategoryModel,
        "select id, lang, title, owner_id, NULL as lang_name from menu_categories where lang = $1",
        lang
    )
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

pub async fn get_menu_items(app_state: &AppState, lang: i32) -> Vec<MenuItemModel> {
    let result = sqlx::query_as!(
        MenuItemModel,
        "select id, lang, title, description, price, owner_id from menu_items where lang=$1",
        lang
    )
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
