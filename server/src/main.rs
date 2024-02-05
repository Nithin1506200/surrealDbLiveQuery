mod auth_service;
mod common;
mod config;
mod controller;
mod db;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpResponse, HttpServer};
use config::ENV;
use env_logger;
use env_logger::Env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::initdb().await.expect("failed db");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allow_any_method()
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .route("/ping", web::get().to(HttpResponse::Ok))
            .service(web::scope("api").configure(controller::config))
    })
    .workers(3)
    .bind(("127.0.0.1", ENV.port))?
    .run()
    .await
}
