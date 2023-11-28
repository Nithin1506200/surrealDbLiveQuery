use actix_web::web;

use self::{login::login_controller, signup::signup_controller};

mod login;
mod signup;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_controller).service(signup_controller);
}
