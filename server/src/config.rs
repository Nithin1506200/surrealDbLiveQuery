#![allow(dead_code)]

use dotenv::dotenv;
use once_cell::sync::Lazy;
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let port = std::env::var("PORT").expect("port must be set");
        let port: u16 = port.parse().expect("port should be number");
        Config {
            database_url,
            jwt_secret,
            port,
        }
    }
}
pub static ENV: Lazy<Config> = Lazy::new(Config::init);
