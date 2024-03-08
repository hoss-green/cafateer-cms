use crate::{
    data_context::{self, context::AppState, manager::menu_item_details},
    manager::{macro_templates::MenuItemButton, templates::MenuPage},
    models::data::{reference_items::Language, ClaimsModel, MenuItemDetailsModel},
    session::claims::Claims,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, response::Html, Extension};
use http::StatusCode;
use std::collections::HashMap;

pub async fn get_menu_page(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let menu_item_details: Vec<MenuItemDetailsModel> =
        menu_item_details::get_all(&app_state, &claims.sub).await;
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub).await;
    let languages = Language::vec_from_int_vec(
        &data_context::references::get_languages(database_pool).await,
        &account_languages,
    );
    let mut menu_items =
        data_context::manager::menu_items::get_for_account(database_pool, &claims.sub).await;
    let mut unique_menu_ids: HashMap<uuid::Uuid, bool> = HashMap::new();
    menu_items.sort_by(|a, b| (format!("{}{}", a.id, a.lang)).cmp(&format!("{}{}", b.id, b.lang)));
    menu_items.clone().into_iter().for_each(|mi| {
        unique_menu_ids.insert(mi.id, true);
    });

    let menu_item_buttons: Vec<MenuItemButton> = unique_menu_ids
        .iter()
        .map(|unique_mi| {
            let button_title = match menu_items
                .iter()
                .find(|mi| mi.id == *unique_mi.0 && mi.lang == claims.body.lang)
            {
                Some(cat) => cat.clone().title,
                None => "No title".to_string(),
            };
            MenuItemButton {
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
            }
        })
        .collect();

    let menu_editor = MenuPage {
        title: "Edit Menu",
        menu_item_buttons,
    };

    let menu_editor: String = menu_editor.render().unwrap().to_string();
    (StatusCode::OK, Html(menu_editor)).into_response()
}
