use axum::{
    routing::{get, post},
    Router,
};
use cafeteer::manager::{get_details_home, get_manager_home};
use cafeteer::presenter::restaurant::get_restaurant;
use cafeteer::{data::context::AppState, manager::post_details_home};
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

    let router = Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .route("/", get(get_restaurant))
        .route("/manager", get(get_manager_home))
        .route(
            "/manager/details",
            get(get_details_home).post(post_details_home),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&"127.0.0.1:4444")
        .await
        .unwrap();
    let server = axum::serve(listener, router); //, make_service)
    server.await.unwrap();
}

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// struct Params {
//     #[serde(default, deserialize_with = "empty_string_as_none")]
//     word: Option<String>,
// }

// Serde deserialization decorator to map empty Strings to None,
// fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
// where
//     D: Deserializer<'de>,
//     T: FromStr,
//     T::Err: fmt::Display,
// {
//     let opt = Option::<String>::deserialize(de)?;
//     match opt.as_deref() {
//         None | Some("") => Ok(None),
//         Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
//     }
// }
