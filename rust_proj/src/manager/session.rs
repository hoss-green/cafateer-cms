use askama_axum::{IntoResponse, Response};
use axum::body::Body;
use http::{header::SET_COOKIE, HeaderValue};

use crate::{
    models::data::{ClaimsModel, ProfileModel},
    session::{claims::Claims, models::AccountModel, tokens::account_to_jwt},
};
pub mod pages;
mod templates;

pub async fn create_cookie_header<'a>(user_account: &AccountModel, profile: &ProfileModel) -> HeaderValue {
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
        Ok(header_val) => {
            header_val
        }
        Err(_) => {
            panic!("could not create header");
        }
    }
}

// pub trait TokenExtensions {
//     async fn add_session_cookie_mut(&self, user_account: &AccountModel, profile: &ProfileModel) -> dyn IntoResponse Box<>;
// }
//
// impl TokenExtensions for dyn IntoResponse {
//     async fn add_session_cookie_mut(&self, user_account: &AccountModel, profile: &ProfileModel) -> dyn IntoResponse {
//         // let response = self.clone();
//         let claims_model: ClaimsModel = ClaimsModel {
//             lang: profile.primary_language,
//         };
//
//         let claims = Claims {
//             sub: user_account.id,
//             email: user_account.email_normalised.clone(),
//             roles: vec![],
//             body: claims_model,
//             exp: 0,
//         };
//
//         let cookie = format!("token={}; same-site=Lax; path=/;", account_to_jwt(claims));
//         match HeaderValue::from_str(&cookie) {
//             Ok(header_val) => {
//
//                 self
//                 // *self.into_response()
//                 // self.headers_mut().insert(SET_COOKIE, header_val);
//                 // self.clone().into_response()
//                 // self.into_response()
//                 // let mut redirect = Redirect::to("/manager").into_response();
//                 // redirect .headers_mut().insert(SET_COOKIE, header_val);
//                 // return redirect;
//             }
//             Err(_) => {
//                 println!("could not parse header");
//                 // self.into_response()
//                 // self.into_response()
//                 self
//             },
//
//         }
//     }
// }
// pub fn refresh_cookie(mut response: Response, account_id: &uuid::Uuid) -> impl IntoResponse {
//     match HeaderValue::from_str(&create_token_cookie(&user_account, &profile).await) {
//         Ok(header_val) => {
//             response.headers_mut().insert(SET_COOKIE, header_val);
//             response
//         }
//         Err(_) => {
//             println!("could not parse header");
//             response
//         }
//     }
// }

// pub async fn create_token_cookie(account: &AccountModel, profile: &ProfileModel) -> String {
//
//     use chrono::{Duration, Utc};
//     let claims = Claims {
//         sub: user_account.id,
//         email: user_account.email_normalised.clone(),
//         roles: vec![],
//         body: claims_model,// ClaimsModel { lang: profile_model.primary_language },
//         // roles: user_account.roles,
//         // sub_expiry: user_account.subscription_expiry,
//         // sub_status: user_account.subscription_status.unwrap_or(String::new()),
//         // product_id: user_account.product_id.unwrap_or(String::new()),
//         exp: now
//             .checked_add_signed(Duration::seconds(EXPIRY_SECONDS))
//             .unwrap()
//             .timestamp(),
//     };
//     let claims_model: ClaimsModel = ClaimsModel {
//         lang: profile.primary_language,
//     };
//     let cookie_string = account_to_jwt::<ClaimsModel>(&account, &claims_model);
//     format!("token={}; same-site=Lax; path=/;", cookie_string)
// }
