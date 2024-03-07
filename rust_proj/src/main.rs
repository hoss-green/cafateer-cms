use axum::{
    extract::Request,
    middleware,
    response::Redirect,
    routing::{delete, get, post, put},
    Router,
};
use cafeteer::{
    auth_middleware,
    data_context::context::AppState,
    manager::{
        create_category_item, delete_category_item, get_category_item, get_menu_item_details,
        menu_item_manager::*,
        pages::{get_account_page, get_categories_page, get_home_page, get_menu_page},
        post_details_home, update_category_item, update_menu_item_details,
    },
    presenter::{menu::get_menu, restaurant::get_restaurant_with_lang},
    session,
};
use cafeteer::{
    data_context::setup_db,
    manager::{get_details_home, post_language},
};
use cafeteer::{
    manager::{get_details_data, post_primary_language},
    presenter::restaurant::get_restaurant,
};
use dotenv::dotenv;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pg = std::env::var("DATABASE_URL").unwrap();
    let state: AppState = AppState {
        database_pool: match cafeteer::data_context::context::get_db_pool(pg).await {
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
        .route("/:lang", get(get_restaurant_with_lang))
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
        .route("/manager/menu/item/:id", delete(delete_menu_item))
        .route("/manager/menu/item/details/:id", get(get_menu_item_details))
        .route("/manager/menu/item/details", put(update_menu_item_details))
        .route("/manager/config", get(get_account_page))
        .route("/manager/config/languages", post(post_language))
        .route("/session", get(Redirect::permanent("/session/login")))
        .route("/session/login", get(session::login))
        .route("/session/login", post(session::do_login))
        .route("/session/sign_up", get(session::sign_up))
        .route("/session/sign_up", post(session::do_signup))
        .route("/session/sign_up_success", get(session::sign_up_success))
        .route(
            "/manager/config/primary_language/:id",
            post(post_primary_language),
        )
        .route_layer(middleware::from_fn(auth_middleware::check_auth))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&"127.0.0.1:4444")
        .await
        .unwrap();
    // this is to ignore the trailing slash
    // everthing in app happens BEFORE the routing
    let app = NormalizePathLayer::trim_trailing_slash().layer(router);
    let server = axum::serve(
        listener,
        axum::ServiceExt::<Request>::into_make_service(app),
    );

    server.await.unwrap()
}
