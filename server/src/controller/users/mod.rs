use crate::common::error::AppError;

use crate::db::users::User;
use crate::db::{self};
use actix_web::web::Json;
use actix_web::{get, web, Result as ActixResult};

#[get("/")]
async fn get_all_users() -> ActixResult<Json<Vec<User>>, AppError> {
    let mut res = db::DB.query("SELECT * FROM users;").await?;
    let res: Vec<User> = res.take(0)?;
    println!("------*******Fsadfasfasdfasdfasfd");
    Ok(web::Json(res))
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
}
