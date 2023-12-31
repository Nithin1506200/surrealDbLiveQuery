#![allow(dead_code)]

use dotenv::dotenv;
use once_cell::sync::Lazy;
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Config {
            database_url,
            jwt_secret,
        }
    }
}
pub static ENV: Lazy<Config> = Lazy::new(Config::init);
