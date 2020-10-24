mod models;
mod config;

use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use std::io;
use dotenv::dotenv;
use crate::config::Config;

async fn index() -> impl Responder {
  web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string()})
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    dotenv().ok();
    let config = Config::from_env().unwrap();
    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}