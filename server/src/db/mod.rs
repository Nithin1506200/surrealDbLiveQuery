#![allow(dead_code)]
pub mod scopes;
pub mod users;
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
    Test,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Db {
    Test,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Tables {
    Users,
    Merchants,
}
