use super::person::*;
use super::schema::persons;
use super::PersonsDatabase;

use rocket_contrib::json::Json;
use crate::diesel::prelude::*;

#[get("/persons/<id>")]
pub fn show_unit(id: i32) -> String {
    format!("You want some info about {}'s person!\n", id)
}

#[get("/persons")]
pub fn show_all(conn: PersonsDatabase) -> String {
    let results = persons::table.load::<Person>(&*conn)
        .expect("Error loading persons");
    println!("Displaying {} persons", results.len());
    for person in results {
        println!("{} works at {}", person.name, person.work);
    }
    format!("You want some info about everybody!\n")
}

#[post("/persons", data = "<p>")]
pub fn add(p: Json<Person>) -> Json<Person> {
    p
}

#[patch("/persons/<id>", data = "<p>")]
pub fn patch(id: i32, p: Json<Person>) -> Json<Person> {
    format!("You want make some changes on {} user!\n", id);
    let mut p2 = p;
    p2.id = id;
    p2
}

#[delete("/persons/<id>")]
pub fn delete(id: i32) -> String {
    format!("You want to delete {} user!\n", id)
}
