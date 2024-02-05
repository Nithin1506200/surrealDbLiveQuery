use crate::common::error::AppError;
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

use super::{Tables, DB};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub pass: String,
    pub account_id: RecordId,
    pub id: RecordId,
}

/**
 * email should be unique
 */
pub async fn get_user_by_email(email: &str) -> actix_web::Result<User, ()> {
    let response = DB
        .query(format!(
            "SELECT * FROM {} WHERE email = '{:?}'",
            Tables::Users.to_string(),
            email
        ))
        .await;
    match response {
        Ok(mut response) => {
            let user: Result<Vec<User>, _> = response.take(0);
            match user {
                Ok(u) => {
                    if u.len() == 1 {
                        Ok(u[0].to_owned())
                    } else {
                        Err(())
                    }
                }
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

pub async fn validate(email: &str, pass: &str) -> actix_web::Result<(bool, Vec<User>), AppError> {
    let mut response = DB
        .query(format!(
            "SELECT * FROM {} WHERE email = {:?} AND crypto::argon2::compare(pass,{:?}) LIMIT 1",
            Tables::Users.to_string(),
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
    merchant_id: &str,
    name: &str,
) -> actix_web::Result<Vec<User>, AppError> {
    let mut response = DB
        .query(format!("CREATE {} CONTENT {{ name:'{}',email:'{}',pass: crypto::argon2::generate('{}'),merchant_id:'{}'}}",
            Tables::Users.to_string(),
            name,
            email,
            pass,
            merchant_id.to_string()
        ))
        .await?;

    println!("----yxx-----------",);
    let users = response.take::<Vec<User>>(0)?;
    let user = users[0].clone();
    println!("----xxx-----------{:?}", user);
    // println!()
    Ok(vec![])
}
#[cfg(test)]
mod test {
    use serde_json::Value;

    use crate::db::{users::User, Tables};

    #[test]
    fn query_format() {
        let name = "tobie";
        let email = "ntihn@gmail.com";
        let pass = "123";
        let merchant_id = "GooglePay";
        let format=format!("CREATE {} CONTENT = {{ name:'{}',email:'{}',pass: crypto::argon2::generate('{}'),merchant_id:'{}'}}",

            Tables::Users.to_string(),
            name,
            email,
            pass,
            merchant_id
        );
        println!("{}", format)
    }
    #[test]
    fn val_parse() {
        let string = r#"Object {"email": String("nithin@gmail.com"), "id": Object {"tb": String("users"), "id": Object {"String": String("9edus9nf0ijmbd64v84b")}}, "merchant_id": String("Paypal"), "name": String("nithin"), "pass": String("$argon2id$v=19$m=19456,t=2,p=1$2V5V+IMdL4u7nxd0P5IIyQ$r4Vl1zJPHXy4ar4/3RwNV4tq0YWpe2mGyrxmqFdxdJU")}"#;
        let val: Value = string.into();
        let user = serde_json::from_value::<User>(val.clone());
        println!("{:?}", user);
    }
}
