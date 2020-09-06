use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String,
}

