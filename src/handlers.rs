use actix_web::{Responder, web, HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::models::CreatePerson;

use crate::db;

pub async fn status() -> impl Responder {
    "{\"status\": \"UP\"}"
}

pub async fn get_persons(db_pool: web::Data<Pool>) -> impl Responder{

    let client: Client =
        db_pool.get().await.expect("Error connecting to the db");

    let result = db::get_person_list(&client).await;

    match result {
        Ok(persons) => HttpResponse::Ok().json(persons),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_person(db_pool: web::Data<Pool>, json: web::Json<CreatePerson>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the db");

    let result = db::create_person(&client, json.0).await;

    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}