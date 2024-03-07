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
