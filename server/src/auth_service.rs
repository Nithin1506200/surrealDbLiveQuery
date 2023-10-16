#![allow(dead_code)]
use crate::config::ENV;
use crate::db::scopes::Scope;
use crate::db::users::User;
use crate::db::{Db, Namespace};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::EnumAccess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub NS: Namespace,
    pub DB: Db,
    pub SC: Scope,
    pub email: String,
    pub merchant_id: String,
    pub exp: usize,
    pub iat: String,
}
/**
 * Expiry in 1 day
 */
pub fn get_token(user: &User) -> String {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        NS: Namespace::Test,
        DB: Db::Test,
        SC: Scope::OrderViewerAll,
        email: user.email.to_string(),
        merchant_id: user.merchant_id.to_string(),
        exp: exp as usize,
        iat: Utc::now().to_string(),
    };

    let header = Header::new(Algorithm::HS512);
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_base64_secret(ENV.jwt_secret.as_str()).expect("json error"),
    )
    .expect("json");
    token
}
