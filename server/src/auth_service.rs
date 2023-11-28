#![allow(dead_code)]

use crate::config::ENV;
use crate::db::scopes::Scope;

use crate::db::users::User;
use crate::db::{Db, Namespace};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
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
pub fn get_token_from_user(user: &User) -> String {
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
        &EncodingKey::from_secret(ENV.jwt_secret.as_ref()),
    )
    .expect("json");
    token
}

#[cfg(test)]
mod auth_test {
    use surrealdb::sql::Thing;

    use crate::{auth_service::get_token_from_user, db::users::User};

    #[test]
    fn get_token() {
        let user: User = User {
            name: "nithin".to_owned(),
            email: "nithin@gmail.com".to_owned(),
            pass: "Fsadfasf".to_owned(),
            merchant_id: Thing::from(("merchant", "Fsadf")),
            id: Thing::from(("user", "Fsdafadf")),
        };
        println!("{:?}", get_token_from_user(&user))
    }
}
