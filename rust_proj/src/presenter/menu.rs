use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use http::StatusCode;

use crate::{
    data_context::context::AppState,
    models::views::{components::MenuTabComponent, pages::MenuPage},
};

pub async fn get_menu(
    State(app_state): State<AppState>,
    lang: Path<i32>,
) -> (StatusCode, Html<String>) {
    // let mut menu_items_lunch: Vec<MenuItemComponent> = vec![];
     let categories = crate::data_context::presenter::fetcher::get_categories(&app_state, &uuid::Uuid::new_v4(), *lang).await;
    // let menu_items = crate::data::presenter::fetcher::get_menu_items(&app_state, *lang).await;

    // let menu_items_breakfast = menu_items
    //     .iter()
    //     .map(|menu_item| {
    //         let desc = match &menu_item.description {
    //             Some(str) => str.clone(),
    //             None => String::new(),
    //         };
    //         MenuItemComponent {
    //             title: menu_item.title.clone(),
    //             description: desc,
    //             price: menu_item.price.unwrap_or(0.0),
    //             category: uuid::Uuid::new_v4()
    //             // String::new(),
    //         }
    //         .clone()
    //     })
    //     .collect();

    // menu_items_lunch.push(MenuItemComponent {
    //     title: "CCCC is the title".to_string(),
    //     description: "description 3".to_string(), //.to_string(),
    //     price: 15.50,
    //     category: "Lunch".to_string(), //.to_string(),
    // });

    // let menu_tab_breakfast: MenuTabComponent = MenuTabComponent {
    //     name: "Breakfast", //.to_string(),
    //     category: uuid::Uuid::new_v4(),
    //     menu_items: menu_items_breakfast,
    // };
    //
    // let menu_tab_lunch: MenuTabComponent = MenuTabComponent {
    //     name: "Lunch", //.to_string(),
    //     category: uuid::Uuid::new_v4(),
    //     menu_items: menu_items_lunch,
    // };
    //
    let menu_tabs: Vec<MenuTabComponent> = vec![];
    let menu_page = MenuPage {
        title: "Sunny Cafe",
        categories,
        menu_tabs,
    };

    let restaurant_page: String = menu_page.render().unwrap().to_string();

    (StatusCode::OK, Html(restaurant_page))
}
