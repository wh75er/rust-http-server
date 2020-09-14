use diesel::prelude::*;
use super::schema::persons;
use super::PersonsDatabase;
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug)]
pub struct Person {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String,
}

impl Person {
    pub fn create(p: &Person, conn: &PersonsDatabase) -> Result<Person, diesel::result::Error> {
        let mut vec = diesel::insert_into(persons::table)
            .values((
                persons::name.eq(&p.name),
                persons::age.eq(&p.age),
                persons::address.eq(&p.address),
                persons::work.eq(&p.work)
                ))
            .get_results(&**conn)?;
        vec.pop().ok_or(diesel::result::Error::NotFound)
    }

    pub fn read(conn: &PersonsDatabase) -> Result<Vec<Person>, diesel::result::Error> {
        persons::table.load::<Person>(&**conn)
    }

    pub fn read_id(id: i32, conn: &PersonsDatabase) -> Result<Person, diesel::result::Error> {
        let mut vec = persons::table.filter(persons::id.eq(id))
            .load::<Person>(&**conn)?;
        vec.pop().ok_or(diesel::result::Error::NotFound)
    }

    pub fn update(id: i32, p: &Person, conn: &PersonsDatabase) -> Result<Person, diesel::result::Error> {
        diesel::update(persons::table.find(id))
            .set(p)
            .get_result(&**conn)
    }

    pub fn delete(id: i32, conn: &PersonsDatabase) -> Result<usize, diesel::result::Error> {
        diesel::delete(persons::table.find(id)).execute(&**conn)
    }
}
