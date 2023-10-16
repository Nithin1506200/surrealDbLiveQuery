use actix_web::{get, http::header, web, HttpRequest, Result as ActixResult};

use crate::{
    common::error::AppError,
    db::{self, users::User},
};

#[get("/")]
async fn get_all_users() -> Result<String, AppError> {
    let mut res = db::DB.query("SELECT * FROM users;").await?;
    let res: Vec<User> = res.take(0)?;
    Ok("gpt usre".to_string())
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
}
