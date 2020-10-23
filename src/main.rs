mod models;

use crate::models::Status;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::io;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn index() -> impl Responder {
  web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string()})
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}