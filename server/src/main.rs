mod auth_service;
mod common;
mod config;
mod controller;
mod db;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::initdb().await.expect("failed db");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .service(web::scope("api").configure(controller::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
