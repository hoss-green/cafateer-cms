use chrono::NaiveDateTime;

pub struct AccountModel {
    pub id: uuid::Uuid,
    pub email: String,
    pub email_normalised: String,
    pub salt: String,
    pub password_hash: String,
    pub sign_up: NaiveDateTime,
    pub status: i32
}
