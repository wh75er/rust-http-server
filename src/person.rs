use crate::schema::persons;
use crate::PersonsDatabase;
use serde::{Deserialize, Serialize};
use crate::db::DbOps;
use std::result::Result;

#[derive(PartialEq, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug, Clone)]
pub struct Person {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String,
}

impl Person {
    pub fn create(p: &Person, conn: &PersonsDatabase, dbops: impl DbOps) -> Result<Person, diesel::result::Error> {
        let mut vec = dbops.insert(p, conn)?;
        vec.pop().ok_or(diesel::result::Error::NotFound)
    }

    pub fn read(conn: &PersonsDatabase, dbops: impl DbOps) -> Result<Vec<Person>, diesel::result::Error> {
        dbops.load(conn)
    }

    pub fn read_id(id: i32, conn: &PersonsDatabase, dbops: impl DbOps) -> Result<Person, diesel::result::Error> {
        let mut vec = dbops.load_id(id, conn)?;
        vec.pop().ok_or(diesel::result::Error::NotFound)
    }

    pub fn update(id: i32, p: &Person, conn: &PersonsDatabase, dbops: impl DbOps) -> Result<Person, diesel::result::Error> {
        dbops.update(id, p, conn)
    }

    pub fn delete(id: i32, conn: &PersonsDatabase, dbops: impl DbOps) -> Result<usize, diesel::result::Error> {
        dbops.delete(id, conn)
    }
}
