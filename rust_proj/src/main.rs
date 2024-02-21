use axum::{
    routing::{delete, get, post, put},
    Router,
};
use cafeteer::{
    data::context::AppState,
    manager::{
        create_category_item, delete_category_item, get_category_item, get_menu_item_details,
        menu_item_manager::*,
        pages::{get_account_page, get_categories_page, get_home_page, get_menu_page},
        post_details_home, update_category_item,
    },
    presenter::menu::get_menu,
};
use cafeteer::{
    data::setup_db,
    manager::{get_details_home, post_language},
};
use cafeteer::{
    manager::{get_details_data, post_primary_language},
    presenter::restaurant::get_restaurant,
};
use dotenv::dotenv;
use tower_http;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pg = std::env::var("DATABASE_URL").unwrap();
    let state: AppState = AppState {
        database_pool: match cafeteer::data::context::get_db_pool(pg).await {
            Ok(db) => {
                println!("Connected to database");
                db
            }
            Err(er) => {
                println!("Could not create database, err: {}", er.to_string());
                panic!("Could not create database and connect to pool");
            }
        },
    };

    setup_db(&state).await;

    let router = Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .route("/", get(get_restaurant))
        .route("/menu", get(get_menu))
        .route("/manager", get(get_home_page))
        .route(
            "/manager/details",
            get(get_details_home).post(post_details_home),
        )
        .route("/manager/details/data/:id", get(get_details_data))
        .route("/manager/menu", get(get_menu_page))
        .route("/manager/menu/categories", get(get_categories_page))
        .route("/manager/menu/categories/:id/:lang", get(get_category_item))
        .route("/manager/menu/categories", put(update_category_item))
        .route("/manager/menu/categories", post(create_category_item))
        .route("/manager/menu/categories/:id", delete(delete_category_item))
        .route("/manager/menu/item/:id/:lang", get(get_menu_item))
        .route("/manager/menu/item", post(create_menu_item))
        .route("/manager/menu/item", put(update_menu_item))
        .route("/manager/menu/item/:id", post(delete_menu_item))
        .route("/manager/menu/item/details/:id", get(get_menu_item_details))
        .route("/manager/config", get(get_account_page))
        .route("/manager/config/languages", post(post_language))
        .route(
            "/manager/config/primary_language/:id",
            post(post_primary_language),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&"127.0.0.1:4444")
        .await
        .unwrap();
    let server = axum::serve(listener, router); //, make_service)
    server.await.unwrap();
}
