mod models;

use actix_web::{HttpServer, App, web, Responder};
use std::io;
use crate::models::Person;

async fn status() -> impl Responder {
  "{\"status\": \"UP\"}"
}

async fn get_person(_name: web::Path<String>) -> impl Responder {
  let address = "stuff street 1232, 321321 some state";
  //println!("{}", address);
    web::HttpResponse::Ok()
    .json(Person {name: _name.to_string(), address: address.to_string(), phone: 45801153})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {

        App::new()
        .route("/", web::get().to(status))
        .route("/person/{name}", web::get().to(get_person))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
