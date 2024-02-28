use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use http::StatusCode;

use crate::{
    data::context::AppState,
    models::views::{
        components::{MenuItemComponent, MenuTabComponent},
        pages::{MenuPage, RestaurantPage},
    },
};

pub async fn get_restaurant(State(app_state): State<AppState>) -> (StatusCode, Html<String>) {
    get_restaurant_with_lang(State(app_state), Path(0)).await
}

pub async fn get_restaurant_with_lang(
    State(app_state): State<AppState>,
    Path(lang): Path<i32>,
) -> (StatusCode, Html<String>) {
    let details = crate::data::presenter::fetcher::get_details(&app_state, lang).await;
    let categories = crate::data::presenter::fetcher::get_categories(&app_state, lang).await;
    let mut menu_items = crate::data::presenter::fetcher::get_menu_items(&app_state, lang).await;

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
    let menu_page = MenuPage {
        title,
        categories,
        menu_tabs,
    };
    let restaurant_page = RestaurantPage {
        title,
        menu_page,
        blurb: &details.blurb.unwrap_or(String::new()),
    };

    let restaurant_page: String = restaurant_page.render().unwrap().to_string();

    (StatusCode::OK, Html(restaurant_page))
}
