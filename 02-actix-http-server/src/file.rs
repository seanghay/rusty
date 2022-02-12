use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
  id: u128,
  title: String,
  body: String,  
}

pub fn lists() -> Vec<Person> {
  let filename = "./db.json";
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  serde_json::from_str(contents.as_str()).unwrap()
}