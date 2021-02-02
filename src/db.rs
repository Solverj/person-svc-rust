use crate::models::{Person, CreatePerson};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_person_list(client: &Client) -> Result<Vec<Person>, io::Error> {
    let statement = client.prepare("select * from person").await.unwrap();
    let persons = client.query(&statement, &[])
        .await
        .expect("Error getting person list")
        .iter()
        .map(|row| Person::from_row_ref(row).unwrap())
        .collect::<Vec<Person>>();

    Ok(persons)
}

pub async fn create_person(client: &Client, person: CreatePerson) -> Result<Person, io::Error> {
    let name = person.name.clone();
    let phone = person.phone.clone();
    let address = person.address.clone();
    let statement = client.prepare("insert into Person (name, phone, address) values ($1, $2, $3) returning id, name, phone, address").await.unwrap();
    client.query(&statement, &[&name, &phone, &address])
        .await
        .expect("Error creating person")
        .iter()
        .map(|row| Person::from_row_ref(row).unwrap())
        .collect::<Vec<Person>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating person"))
}