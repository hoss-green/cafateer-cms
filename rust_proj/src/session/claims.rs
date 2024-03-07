use ::serde::{Deserialize, Serialize};
use uuid::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub exp: i64,
    pub roles: Vec<String>,
    // pub sub_expiry: NaiveDateTime,
    // pub product_id: String,
    // pub sub_status: String,
}

impl Claims {
    pub fn new() -> Claims {
        Claims {
            sub: Uuid::nil(),
            email: String::new(),
            // sub_expiry: NaiveDateTime::default(),
            exp: 0,
            roles: Vec::<String>::new(),
            // product_id: String::new(),
            // sub_status: String::new(),
        }
    }
}
