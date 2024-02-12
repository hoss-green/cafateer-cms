use axum::{
    routing::{get, post},
    Router,
};
use cafeteer::{data::context::AppState, manager::post_details_home};
use cafeteer::{
    data::{account::set_language, setup_db},
    manager::{get_account_page, get_details_home, get_start_page, post_language},
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
        .route("/manager", get(get_start_page))
        .route(
            "/manager/details",
            get(get_details_home).post(post_details_home),
        )
        .route("/manager/details/data/:id", get(get_details_data))
        .route("/manager/account", get(get_account_page))
        .route("/manager/account/languages", post(post_language))
        .route(
            "/manager/account/primary_language/:id",
            post(post_primary_language),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&"127.0.0.1:4444")
        .await
        .unwrap();
    let server = axum::serve(listener, router); //, make_service)
    server.await.unwrap();
}
