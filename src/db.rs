use crate::models::{Person};
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