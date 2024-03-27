use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    response::{AppendHeaders, Html},
};
use http::{
    header::{COOKIE, SET_COOKIE},
    HeaderMap,
};

use crate::{
    data_context::context::AppState,
    models::{
        data::reference_items::Language,
        views::{
            components::{MenuItemComponent, MenuTabComponent},
            pages::{MenuPage, RestaurantPage},
        },
    },
};

pub async fn get_restaurant(
    headers: HeaderMap,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let lang = match headers.get(COOKIE) {
        Some(cookie_header) => match cookie_header.to_str() {
            Ok(cookie_string) => cookie_string.to_string(),
            Err(_) => String::from("en"),
        },
        None => String::from("en"),
    };

    get_restaurant_with_lang(State(app_state), Path(lang)).await
}

pub async fn get_restaurant_with_lang(
    State(app_state): State<AppState>,
    Path(lang_code): Path<String>,
) -> impl IntoResponse {
    let lang =
        match crate::data_context::references::get_language(&app_state.database_pool, &lang_code)
            .await
        {
            Some(lang) => lang.id,
            None => 0,
        };
    let available_languages =
        crate::data_context::references::get_languages(&app_state.database_pool).await;
    let language_codes = crate::data_context::manager::profile_languages::get_all_ids_debug(
        &app_state.database_pool,
    )
    .await;

    let details = crate::data_context::presenter::fetcher::get_details(&app_state, lang).await;
    let categories =
        crate::data_context::presenter::fetcher::get_categories(&app_state, lang).await;
    let mut menu_items =
        crate::data_context::presenter::fetcher::get_menu_item_vms(&app_state, lang).await;
    let mut menu_tabs: Vec<MenuTabComponent> = vec![];
    menu_items.iter_mut().for_each(|mi| {
        let category_id = match mi.category {
            Some(cat) => cat,
            None => uuid::Uuid::nil(),
        };
        let mt = menu_tabs.iter_mut().find(|mt| mt.category == category_id);
        match mt {
            Some(mt) => mt.menu_items.push(MenuItemComponent {
                title: mi.title.clone(),
                description: mi.description.clone().unwrap_or(String::new()),
                price: mi.price.unwrap_or(0.0),
                category: mi.category.unwrap_or(uuid::Uuid::nil()),
            }),
            None => {
                let mtc = MenuTabComponent {
                    category: mi.category.unwrap_or(uuid::Uuid::nil()),
                    name: match mi.category {
                        Some(cat_id) => match categories.iter().find(|c| c.id == cat_id) {
                            Some(cat) => {
                                let cname = cat.clone().title.unwrap_or("Unknown".to_string());
                                cname.clone()
                            }
                            None => "Unknown".to_string(),
                        },
                        None => "Unknown".to_string(),
                    },
                    menu_items: vec![MenuItemComponent {
                        title: mi.title.clone(),
                        description: mi.description.clone().unwrap_or(String::new()),
                        price: mi.price.unwrap_or(0.0),
                        category: mi.category.unwrap_or(uuid::Uuid::nil()),
                    }],
                };
                menu_tabs.push(mtc.clone());
            }
        };
    });

    let title = "Sunny Cafe";
    let restaurant_page = RestaurantPage {
        title,
        menu_page: MenuPage {
            title,
            categories,
            menu_tabs,
        },
        blurb: &details.blurb.unwrap_or(String::new()),
        languages: Language::vec_from_int_vec(&available_languages, &language_codes),
    };

    let restaurant_page: String = restaurant_page.render().unwrap().to_string();

    // let cookie_string = format!("user_language={}; same-site=Lax; path=/;", lang_code);
    let headers: AppendHeaders<[(http::HeaderName, String); 1]> = AppendHeaders([(
        SET_COOKIE,
        format!("user_language={}; same-site=Lax; path=/;", lang),
    )]);
    (headers, Html(restaurant_page)).into_response()
    // (StatusCode::OK, Html(restaurant_page))
}
