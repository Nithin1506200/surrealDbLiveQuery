use actix_web::web;

use crate::common::{self, error::AppError, merchant};

use super::{Tables, DB};
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub pass: String,
    pub merchant_id: String,
    pub id: String,
}
/**
 * when success returns
 */
pub async fn get_user_by_email(email: &str) -> actix_web::Result<Vec<User>, AppError> {
    let mut response = DB
        .query(format!(
            "SELECT * FROM {:?} WHERE email = {:?}",
            Tables::Users,
            email
        ))
        .await?;
    let user: Vec<User> = response.take(0)?;
    Ok(user)
}

pub async fn validate(email: &str, pass: &str) -> actix_web::Result<(bool, Vec<User>), AppError> {
    let mut response = DB
        .query(format!(
            "SELECT * FROM {:?} WHERE email = {:?} AND crypto::argon2::compare(pass,{:?})",
            Tables::Users,
            email,
            pass
        ))
        .await?;

    let user: Vec<User> = response.take(0)?;

    Ok((user.len() == 1, user))
}

pub async fn create(
    email: &str,
    pass: &str,
    merchant_id: common::merchant::Merchants,
    name: &str,
) -> actix_web::Result<Vec<User>, AppError> {
    let mut response = DB
        .query(format!(
            "CREATE {:?} CONTENT {{  name:{},
        email:{},
        pass: crypto::argon2::generate({}),
        merchant_id:{}
    }}",
            Tables::Users,
            name,
            email,
            pass,
            merchant_id.to_string()
        ))
        .await?;
    let users: Vec<User> = response.take(0)?;
    Ok(users)
}
