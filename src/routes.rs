use super::person::*;
use super::PersonsDatabase;

use rocket_contrib::json::Json;

#[get("/persons/<id>")]
pub fn show_unit(id: i32, conn: PersonsDatabase) -> Json<Person> {
    Json(Person::read_id(id, &conn).unwrap())
}

#[get("/persons")]
pub fn show_all(conn: PersonsDatabase) -> Json<Vec<Person>> {
    Json(Person::read(&conn).unwrap())
}

#[post("/persons", data = "<p>")]
pub fn add(p: Json<Person>, conn: PersonsDatabase) -> Json<Person> {
    let p = p.into_inner();
    Person::create(&p, &conn);
    Json(p)
}

#[patch("/persons/<id>", data = "<p>")]
pub fn patch(id: i32, p: Json<Person>, conn:PersonsDatabase) -> Json<Person> {
    format!("You want make some changes on {} user!\n", id);
    Json(Person::update(id, &p, &conn).unwrap())
}

#[delete("/persons/<id>")]
pub fn delete(id: i32, conn:PersonsDatabase) -> String {
    Person::delete(id, &conn);
    format!("You want to delete {} user!\n", id)
}
