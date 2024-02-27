use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use http::StatusCode;

use crate::{
    data::{context::AppState, manager::categories},
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
    let mut menu_items_lunch: Vec<MenuItemComponent> = vec![];
    let details = crate::data::presenter::fetcher::get_details(&app_state, lang).await;
    let categories = crate::data::presenter::fetcher::get_categories(&app_state, lang).await;
    let menu_items = crate::data::presenter::fetcher::get_menu_items(&app_state, lang).await;

    let menu_items_breakfast = menu_items
        .iter()
        .map(|menu_item| {
            let desc = match &menu_item.description {
                Some(str) => str.clone(),
                None => String::new(),
            };
            MenuItemComponent {
                title: menu_item.title.clone(),
                description: desc,
                price: menu_item.price.unwrap_or(0.0),
                category: uuid::Uuid::new_v4()
            }
            .clone()
        })
        .collect();

    // menu_items_lunch.push(MenuItemComponent {
    //     title: "CCCC is the title".to_string(),
    //     description: "description 3".to_string(), //.to_string(),
    //     price: 15.50,
    //     category: "Lunch".to_string(), //.to_string(),
    // });

    let menu_tab_breakfast: MenuTabComponent = MenuTabComponent {
        name: "Breakfast", //.to_string(),
        menu_items: menu_items_breakfast,
    };

    let menu_tab_lunch: MenuTabComponent = MenuTabComponent {
        name: "Lunch", //.to_string(),
        menu_items: menu_items_lunch,
    };

    let menu_tabs: Vec<MenuTabComponent> = vec![menu_tab_breakfast, menu_tab_lunch];
    let title = "Sunny Cafe";
    let menu_page = MenuPage { title, categories, menu_tabs };
    let restaurant_page = RestaurantPage {
        title,
        menu_page,
        blurb: &details.blurb.unwrap_or(String::new()),
    };

    let restaurant_page: String = restaurant_page.render().unwrap().to_string();

    (StatusCode::OK, Html(restaurant_page))
}
