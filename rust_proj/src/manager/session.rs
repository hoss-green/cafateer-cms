use crate::{
    models::data::{ClaimsModel, ProfileModel},
    session::{claims::Claims, models::AccountModel, tokens::account_to_jwt},
};
use http::HeaderValue;
pub mod pages;
mod templates;

pub fn create_cookie_header<'a>(
    user_account: &AccountModel,
    profile: &ProfileModel,
) -> HeaderValue {
    let claims_model: ClaimsModel = ClaimsModel {
        lang: profile.primary_language,
    };

    let claims = Claims {
        sub: user_account.id,
        email: user_account.email_normalised.clone(),
        roles: vec![],
        body: claims_model,
        exp: 0,
    };
    let cookie_string = format!("token={}; same-site=Lax; path=/;", account_to_jwt(claims));
    match HeaderValue::from_str(&cookie_string) {
        Ok(header_val) => header_val,
        Err(_) => {
            panic!("could not create header");
        }
    }
}
