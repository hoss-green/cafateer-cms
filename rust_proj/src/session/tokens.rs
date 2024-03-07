use crate::session::claims::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use super::models::AccountModel;

const EXPIRY_SECONDS: i64 = 60 * 60; //60 * 60 -> 1 hour

pub fn account_to_jwt(user_account: &AccountModel) -> String {
    use chrono::{Duration, Utc};

    let now = Utc::now();

    let claims = Claims {
        sub: user_account.id,
        email: user_account.email_normalised.clone(),
        roles: vec![],
        // roles: user_account.roles,
        // sub_expiry: user_account.subscription_expiry,
        // sub_status: user_account.subscription_status.unwrap_or(String::new()),
        // product_id: user_account.product_id.unwrap_or(String::new()),
        exp: now
            .checked_add_signed(Duration::seconds(EXPIRY_SECONDS))
            .unwrap()
            .timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    );
    let token = match token {
        Ok(token) => token,
        Err(err) => panic!("Could not create token: {} ", err),
    };
    token
}

pub fn validate_jwt_and_get_claims(jwt: String) -> Result<Claims, String> {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use jsonwebtoken::decode_header;
    let header = match decode_header(jwt.as_str()) {
        Ok(header_values) => header_values,
        Err(err) => return Err(format!("Could not get claims from token {}", err)),
    };

    let token_data = match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::new(header.alg),
    ) {
        Ok(token) => token.claims,
        Err(err) => return Err(format!("Could not get claims from token {}", err)),
    };

    let expiry_time: DateTime<Utc> = chrono::DateTime::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp_opt(token_data.exp, 0).unwrap(),
        Utc,
    );
    let expired = Utc::now() > expiry_time;
    if expired {
        return Err(format!("Attempted to use an expired token"));
    };

    Ok(token_data)
}



fn get_secret() -> String {
    use std::env;
    match env::var("JWT_SECRET") {
        Ok(value) => value,
        Err(_) => panic!("could not retrieve MP_JWT_SECRET from env variables"),
    }
}
