use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "Person")]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub phone: i32,
}

#[derive(Deserialize)]
pub struct CreatePerson {
    pub name: String,
    pub phone: i32,
    pub address: String
}