use core::panic;

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
        editors::{self, get_menu_item_details, post_details_home, update_menu_item_details},
        pages::{account_page, categories_page, home_page, languages_page, menu_page},
    },
    presenter::{menu::get_menu, restaurant::get_restaurant_with_lang},
};
use cafeteer::{data_context::setup_db, manager::editors::get_details_home};
use cafeteer::{manager::session, presenter::restaurant::get_restaurant};
use dotenv::dotenv;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() {

    //uuid for single mode = 
    //deadbeef-0000-dead-beef-010203040506
    
    let single_user_id = uuid::Uuid::try_parse("deadbeef-0000-dead-beef-010203040506").unwrap();
    dotenv().ok();
    let pg = std::env::var("DATABASE_URL").unwrap();
    let single_user_mode: bool = match std::env::var("SINGLE_USER_MODE") {
        Ok(value) => match value.to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("SINGLE_USER_MODE must be either 'true' or 'false'"),
        },
        Err(err) => panic!("SINGLE_USER_MODE not set error: {}", err),
    };

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
        single_user_mode,
    };
    setup_db(&state).await;

    let router = Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .route("/", get(get_restaurant))
        .route("/:lang", get(get_restaurant_with_lang))
        .route("/menu", get(get_menu))
        .route("/manager", get(home_page::get))
        .route(
            "/manager/details",
            get(get_details_home).post(post_details_home),
        )
        .route("/manager/details/data/:id", get(editors::get_details_data))
        .route("/manager/menu", get(menu_page::get))
        .route("/manager/menu/categories", get(categories_page::get))
        .route(
            "/manager/menu/categories/:id/:lang",
            get(editors::get_category_item),
        )
        .route(
            "/manager/menu/categories",
            put(editors::update_category_item),
        )
        .route(
            "/manager/menu/categories",
            post(editors::create_category_item),
        )
        .route(
            "/manager/menu/categories/:id",
            delete(editors::delete_category_item),
        )
        .route("/manager/menu/item/:id/:lang", get(editors::get_menu_item))
        .route("/manager/menu/item", post(editors::create_menu_item))
        .route("/manager/menu/item", put(editors::update_menu_item))
        .route("/manager/menu/item/:id", delete(editors::delete_menu_item))
        .route("/manager/menu/item/details/:id", get(get_menu_item_details))
        .route("/manager/menu/item/details", put(update_menu_item_details))
        .route("/manager/config/languages", get(languages_page::get))
        // .route("/manager/languages", post(post_language))
        .route("/manager/config", get(account_page::get))
        // .route("/manager/config/languages", post(post_language))
        .route("/session", get(Redirect::permanent("/session/login")))
        .route("/session/login", get(session::pages::login))
        .route("/session/login", post(session::pages::do_login))
        .route("/session/sign_up", get(session::pages::sign_up))
        .route("/session/sign_up", post(session::pages::do_signup))
        .route(
            "/session/sign_up_success",
            get(session::pages::sign_up_success),
        )
        .route(
            "/manager/config/primary_language/:id",
            post(editors::post_primary_language),
        )
        .route_layer(middleware::from_fn(auth_middleware::check_auth))
        .with_state(state);

    let host = std::env::var("HOST").unwrap();
    let post = std::env::var("PORT").unwrap();

    let advertise_url = format!("{}:{}", host, post);
    println!("Server is running on: {}", advertise_url);

    let listener = tokio::net::TcpListener::bind(advertise_url).await.unwrap();
    // this is to ignore the trailing slash
    // everthing in app happens BEFORE the routing
    let app = NormalizePathLayer::trim_trailing_slash().layer(router);
    let server = axum::serve(
        listener,
        axum::ServiceExt::<Request>::into_make_service(app),
    );

    server.await.unwrap()
}
