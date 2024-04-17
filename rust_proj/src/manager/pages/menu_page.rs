use crate::{
    data_context::{self, context::AppState, manager::menu_item_details},
    manager::templates::{component_buttons::MenuItemButtonVm, pages::MenuPageVm, view_models::DropDownLanguageVm},
    models::data::{reference_items::Language, ClaimsModel, MenuItemDetailsModel},
    session::claims::Claims,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;
use std::collections::HashMap;

pub async fn get(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let menu_item_details: Vec<MenuItemDetailsModel> = menu_item_details::get_all(&app_state, &claims.sub).await;
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    let all_languages = &data_context::references::get_languages(database_pool).await;
    let languages: Vec<DropDownLanguageVm> = all_languages
        .iter()
        .map(|lang| DropDownLanguageVm {
            id: lang.id,
            name: lang.name.clone(),
            published: account_languages
                .iter()
                .any(|ac_lang| ac_lang.language == lang.id && ac_lang.published),
        })
        .collect();
    let mut menu_items =
        data_context::manager::menu_items::get_for_account(database_pool, &claims.sub).await;
    let mut unique_menu_ids: HashMap<uuid::Uuid, bool> = HashMap::new();
    menu_items.sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));
    menu_items.clone().into_iter().for_each(|mi| {
        unique_menu_ids.insert(mi.id, true);
    });

    let menu_item_buttons: Vec<MenuItemButtonVm> = unique_menu_ids
        .iter()
        .map(|unique_mi| {
            let button_title = match menu_items
                .iter()
                .find(|mi| mi.id == *unique_mi.0 && mi.lang == claims.body.lang)
            {
                Some(cat) => cat.clone().title,
                None => "No title".to_string(),
            };
            MenuItemButtonVm {
                id: *unique_mi.0,
                title: button_title,
                category: match menu_item_details
                    .iter()
                    .find(|menu_item_desc| menu_item_desc.id == *unique_mi.0)
                {
                    Some(cat) => cat.id.clone().to_string(),
                    None => "None".to_string(),
                },
                user_languages: languages.clone(),
                published: match menu_item_details
                    .iter()
                    .find(|menu_item_desc| menu_item_desc.id == *unique_mi.0)
                {
                    Some(menu_item) => menu_item.published,
                    None => false,
                },
            }
        })
        .collect();

    let menu_editor = MenuPageVm {
        title: "Edit Menu",
        menu_item_buttons,
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor)).into_response()
}
