use diesel::prelude::*;
use super::schema::persons;
use super::PersonsDatabase;
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Person {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String,
}

impl Person {
    pub fn create(p: Person, conn: &PersonsDatabase) -> Result<(), diesel::result::Error> {
        diesel::insert_into(persons::table)
            .values(&p)
            .execute(&**conn)
            .expect("Error inserting person");

        Ok(())
    }

    pub fn read(conn: &PersonsDatabase) -> Vec<Person> {
        persons::table.load::<Person>(&**conn)
            .unwrap()
    }

    pub fn read_id(id: i32, conn: &PersonsDatabase) -> Result<Person, ()> {
        let result = persons::table.filter(persons::id.eq(id))
            .load::<Person>(&**conn)
            .expect("Error loading person").pop();

        match result {
            Some(x) => Ok(x),
            None => Err(()),
        }
    }

    pub fn update(id: i32, p: Person, conn: &PersonsDatabase) -> Result<Person, diesel::result::Error> {
        Ok(diesel::update(persons::table.find(id))
            .set(&p)
            .get_result(&**conn)?)
    }

    pub fn delete(id: i32, conn: &PersonsDatabase) -> Result<(), diesel::result::Error> {
        let result = diesel::delete(persons::table.find(id)).execute(&**conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
