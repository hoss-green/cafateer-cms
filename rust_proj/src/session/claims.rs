use ::serde::{Deserialize, Serialize};
use uuid::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims<T> {
    pub sub: Uuid,
    pub email: String,
    pub exp: i64,
    pub roles: Vec<String>,
    pub body: T
    // pub sub_expiry: NaiveDateTime,
    // pub product_id: String,
    // pub sub_status: String,
}

// impl Claims<T> {
//     pub fn new() -> Claims {
//         Claims {
//             sub: Uuid::nil(),
//             email: String::new(),
//             language: 0,
//             // sub_expiry: NaiveDateTime::default(),
//             exp: 0,
//             roles: Vec::<String>::new(),
//             body: None//T::default()
//             // product_id: String::new(),
//             // sub_status: String::new(),
//         }
//     }
// }
