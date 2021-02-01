use actix_web::{HttpServer, App, web, Responder};
use std::io;

async fn status() -> impl Responder {
  "{\"status\": \"UP\"}"
}

async fn get_person_name(name: web::Path<String>) -> impl Responder {
    println!("{}", name);
    if name.to_string() == "silver" {
      "{\"phone\": \"1337\"}"
    }
    else { 
      "{\"phone\": \"123123\"}"
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {

        App::new()
        .route("/", web::get().to(status))
        .route("/person/{name}", web::get().to(get_person_name))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
