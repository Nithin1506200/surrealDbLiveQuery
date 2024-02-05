#![allow(dead_code)]

use crate::config::ENV;
use crate::db::scopes::Scope;
use crate::db::users::User;
use crate::db::{Db, Namespace};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename(serialize = "NS", deserialize = "NS"))]
    pub ns: Namespace,
    #[serde(rename(serialize = "DB", deserialize = "DB"))]
    pub db: Db,
    #[serde(rename(serialize = "SC", deserialize = "SC"))]
    pub sc: Scope,
    pub email: String,
    pub account_id: String,
    pub exp: usize,
    pub iat: String,
}
/**
 * Expiry in 1 day
 */
pub fn get_token_from_user(user: &User) -> String {
    let exp = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        ns: Namespace::Test,
        db: Db::Test,
        sc: Scope::OrderViewerAll,
        email: user.email.to_string(),
        account_id: user.account_id.to_string(),
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
    use chrono::Utc;
    use surrealdb::sql::Thing;

    use crate::{
        auth_service::get_token_from_user,
        db::{scopes::Scope, users::User, Db, Namespace, DB},
    };

    use super::Claims;

    #[test]
    fn get_token() {
        let user: User = User {
            name: "nithin".to_owned(),
            email: "nithin@gmail.com".to_owned(),
            pass: "Fsadfasf".to_owned(),
            account_id: Thing::from(("merchant", "Fsadf")),
            id: Thing::from(("user", "Fsdafadf")),
        };
        println!("{:?}", get_token_from_user(&user))
    }
    #[test]
    fn serialize_deserialize() {
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("valid timestamp")
            .timestamp();

        let claims: Claims = Claims {
            ns: Namespace::Test,
            db: Db::Test,
            sc: Scope::OrderViewerAll,
            email: "ntihin.n".to_string(),
            account_id: "dsaf".to_string(),
            exp: exp as usize,
            iat: Utc::now().to_string(),
        };
        let json = serde_json::to_string(&claims).expect("failed converting");
        println!("SERIALIZE TEST : {:}", json);
        let claims_des: Claims = serde_json::from_str(&json).unwrap();
        println!("DESERIALIZE TEST : {:?}", claims_des);
    }
}
