use actix_web::web;
pub mod login;
pub mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(login::config)
        .service(web::scope("/users").configure(users::config));
}
