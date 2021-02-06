use actix_web::{Responder, web, HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::models::CreatePerson;

use crate::db;

pub async fn status() -> impl Responder {
    "{\"status\":\"UP\"}"
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

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{body::Body, test, web, App};
    use serde_json::json;

#[actix_rt::test]
async fn test_status() {
        let mut app = 
        test::init_service(
            App::new()
            .route("/", web::get().to(status))
        ).await;
        let req = test::TestRequest::with_header("content-type", "application/json").to_request();
        let mut resp = test::call_service(&mut app, req).await;
        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        assert!(resp.status().is_success());
        assert_eq!(
            &Body::from(json!({"status":"UP"})), body
        );
    }
}