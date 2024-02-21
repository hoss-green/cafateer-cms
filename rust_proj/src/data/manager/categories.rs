use crate::data::context::AppState;
use crate::models::data::CategoryModel;

pub async fn get_category_list(app_state: &AppState, owner_id: &uuid::Uuid) -> Vec<CategoryModel> {
    let result = sqlx::query_as!(
        CategoryModel,
        "select id, lang, title, owner_id, NULL as lang_name from menu_categories where owner_id = $1",
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

// pub async fn get_category_list_by_lang(
//     app_state: &AppState,
//     owner_id: &uuid::Uuid,
//     lang: i32,
// ) -> Vec<CategoryModel> {
//     let result = sqlx::query_as!(
//         CategoryModel,
//         "select m1.id, m1.lang, m1.owner_id, m2.title, rf.name as lang_name
//         from (
//             select distinct mm1.id, mm2.lang as lang, mm2.owner_id as owner_id from
//             menu_categories as mm1
//             left join (
//                 select distinct lang, owner_id from
//                 menu_categories
//             ) mm2
//         on mm1.owner_id = mm2.owner_id
//         ) m1
//         join ref_languages as rf on m1.lang = rf.id 
//         left join (
//             select id, lang, title, owner_id from menu_categories where lang=$1
//         ) m2
//         on m1.id = m2.id and m1.lang = m2.lang where m1.lang=$1 and m1.owner_id = $2",
//         lang,
//         owner_id
//     )
//     .fetch_all(&app_state.database_pool)
//     .await;
//     match result {
//         Ok(r) => r,
//         Err(err) => {
//             println!("Cannot fetch menu items, err: {}", err);
//             vec![]
//         }
//     }
// }

// pub async fn get_category(
//     app_state: &AppState,
//     (id, lang): (uuid::Uuid, i32),
//     owner_id: &uuid::Uuid,
// ) -> CategoryModel {
//     let result = sqlx::query_as!(
//         CategoryModel,
//         "select id, lang, title, owner_id, NULL as lang_name from menu_categories where id = $1 and lang = $2 and owner_id = $3",
//         id,
//         lang,
//         owner_id
//     )
//     .fetch_optional(&app_state.database_pool)
//     .await;
//     match result {
//         Ok(r) => match r {
//             Some(item) => item,
//             None => CategoryModel::new(Some(id), *owner_id),
//         },
//         Err(err) => {
//             println!("Cannot fetch menu items, err: {}", err);
//             CategoryModel::new(Some(id), *owner_id)
//         }
//     }
// }
// pub async fn set_category(
//     app_state: &AppState,
//     account_id: &uuid::Uuid,
//     details_item: CategoryModel,
// ) -> bool {
//     let result = sqlx::query!(
//         "insert into menu_categories(owner_id, id, lang, title) VALUES ($1, $2, $3, $4) ON CONFLICT (id, lang) DO UPDATE SET title=$4 WHERE menu_categories.id=$2 and menu_categories.lang=$3",
//         &account_id,
//         details_item.id,
//         details_item.lang,
//         details_item.title,
//     )
//     .execute(&app_state.database_pool)
//     .await;
//
//     match result {
//         Ok(_r) => {
//             println!("Saved item succesfully");
//             true
//         }
//         Err(err) => {
//             println!("Cannot save item, fail, error: {}", err);
//             false
//         }
//     }
// }
//
// pub async fn delete_category(
//     app_state: &AppState,
//     account_id: &uuid::Uuid,
//     category_id: &uuid::Uuid,
// ) -> bool {
//     let result = sqlx::query!(
//         "delete from menu_categories where owner_id=$1 and id=$2",
//         &account_id,
//         &category_id,
//     )
//     .execute(&app_state.database_pool)
//     .await;
//
//     match result {
//         Ok(r) => {
//             if r.rows_affected() > 0 {
//                 println!("Deleted category succesfully {:?}", r);
//                 return true;
//             }
//             false
//         }
//         Err(err) => {
//             println!("Cannot save item, fail, error: {}", err);
//             false
//         }
//     }
// }
