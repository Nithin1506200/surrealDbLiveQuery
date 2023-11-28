mod auth_service;
mod common;
mod config;
mod controller;
mod db;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use config::ENV;
use env_logger;
use env_logger::Env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::initdb().await.expect("failed db");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/ping", web::get().to(HttpResponse::Ok))
            .service(web::scope("api").configure(controller::config))
    })
    .workers(3)
    .bind(ENV.database_url.as_str())?
    .run()
    .await
}
