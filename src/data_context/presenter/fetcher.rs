use sqlx::Postgres;

use crate::{
    data_context::context::{AppState, DatabasePool},
    models::{
        data::{CategoryModel, DetailsModel, ProfileLanguagesModel},
        views::menu_item_view_model::MenuItemViewModel,
    },
};

pub async fn get_details(app_state: &AppState, owner_id: &uuid::Uuid, lang: i32) -> DetailsModel {
    let result = sqlx::query_as!(
        DetailsModel,
        "select id, lang, blurb from details where lang = $1 and id = $2",
        lang,
        owner_id
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

pub async fn get_categories(app_state: &AppState, owner_id: &uuid::Uuid, lang: i32) -> Vec<CategoryModel> {
    let result = sqlx::query_as!(
        CategoryModel,
        "select id, lang, title, owner_id, NULL as lang_name from menu_categories where lang = $1 and owner_id = $2",
        lang,
        owner_id
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

pub async fn get_menu_item_vms(app_state: &AppState, owner_id: &uuid::Uuid, lang: i32) -> Vec<MenuItemViewModel> {
    let result = sqlx::query_as::<Postgres, MenuItemViewModel>(
        "select mi.id, title, description, d.price, mi.lang, mi.owner_id, d.category, d.allergies from menu_items as mi join menu_item_details d on mi.id = d.id and mi.owner_id = d.owner_id where mi.lang = $1 and mi.owner_id = $2")
        .bind(lang)
        .bind(owner_id)
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

pub async fn get_all_available_languages(
    database_pool: &DatabasePool,
    owner_id: &uuid::Uuid,
) -> Vec<i32> {
    let result = sqlx::query_as!(
        ProfileLanguagesModel,
        r#"select id, language, owner_id, published from account_languages where published = true and owner_id = $1"#,
        owner_id
    )
    .fetch_all(database_pool)
    .await;
    match result {
        Ok(r) => r
            .iter()
            .map(|language_model| language_model.language)
            .collect::<Vec<i32>>(),
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            vec![]
        }
    }
}

pub async fn get_all_ids_debug(database_pool: &DatabasePool) -> Vec<i32> {
    let result = sqlx::query_as!(
        ProfileLanguagesModel,
        r#"select id, language, owner_id, published from account_languages"#,
    )
    .fetch_all(database_pool)
    .await;
    match result {
        Ok(r) => r
            .iter()
            .map(|language_model| language_model.language)
            .collect::<Vec<i32>>(),
        Err(err) => {
            println!("Cannot fetch account, err: {}", err);
            vec![]
        }
    }
}
