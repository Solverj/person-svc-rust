use std::io;

use actix_web::{App, HttpServer, web};
use deadpool_postgres::tokio_postgres::NoTls;
use dotenv::dotenv;

use crate::config::Config;
use crate::handlers::{get_persons, status, create_person};

mod config;
mod models;
mod handlers;
mod db;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    println!("starting server at: http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/persons", web::get().to(get_persons))
            .route("/person", web::post().to(create_person))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
