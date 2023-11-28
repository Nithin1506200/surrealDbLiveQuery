#![allow(dead_code)]
mod merchants;
pub mod scopes;
pub mod users;
use std::fmt;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
extern crate derive_more;

/**
 * Singleton connection for db
 */
pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn initdb() -> surrealdb::Result<()> {
    DB.connect::<Ws>("127.0.0.1:8000")
        .await
        .expect("unable to connect db");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("unable to sign to db");
    // Select a namespace + database
    DB.use_ns("test")
        .use_db("test")
        .await
        .expect("unable to connect ns and db");
    println!("db connected");
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Namespace {
    #[serde(rename(serialize = "test", deserialize = "test"))]
    Test,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Db {
    #[serde(rename(serialize = "test", deserialize = "test"))]
    Test,
}
#[derive(Debug)]
pub enum Tables {
    Users,
    Merchants,
}
impl fmt::Display for Tables {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", format!("{:?}", self).to_lowercase())
    }
}
pub trait DbServices {
    fn get_table_name() -> String;
    fn create(&self);
    fn delete_using_id(id: &str);
    fn get_();
    fn update_using_id(&self);
}
#[cfg(test)]
mod test {
    use crate::db::Tables;

    #[test]
    fn tables() {
        println!("{:?}", Tables::Users.to_string());
        // assert!(format!("{:?}", Tables::Users.to_string()) == "users")
    }
}
