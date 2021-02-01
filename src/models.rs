use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub address: String,
    pub phone: u32
}