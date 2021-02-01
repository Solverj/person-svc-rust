mod config;
mod models;

use actix_web::{HttpServer, App, web, Responder};
use std::io;
use crate::models::Person;
use crate::config::Config;
use dotenv::dotenv;

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

  dotenv().ok();
  let config = Config::from_env().unwrap();
  println!("starting server at: http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(|| {

        App::new()
        .route("/", web::get().to(status))
        .route("/person/{name}", web::get().to(get_person))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
