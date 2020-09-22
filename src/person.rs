use crate::schema::persons;
use crate::PersonsDatabase;
use serde::{Deserialize, Serialize};
use crate::db::DbOps;
use std::result::Result;
use std::fmt::Display;
use std::fmt;

#[derive(PartialEq, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug, Clone)]
pub struct Person {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String,
}

enum ValidateErr {
    AgeErr,
    NameErr,
    AddressErr,
    WorkErr,
    SymbolsErr,
}

impl Display for ValidateErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidateErr::AgeErr => f.write_str("Age is incorrect!"),
            ValidateErr::NameErr => f.write_str("Name consists of invalid characters!"),
            ValidateErr::AddressErr => f.write_str("Address consists of invalid characters!"),
            ValidateErr::WorkErr => f.write_str("Address consists of invalid characters!"),
            ValidateErr::SymbolsErr => f.write_str("String contains invalid symbols!"),
        }
    }
}

pub trait Validate {
    fn age(age: &i32) -> Result<(), ValidateErr>;
    fn symbols(s: &str) -> Result<(), ValidateErr>;
    fn name(name: &str) -> Result<(), ValidateErr>;
    fn work(work: &str) -> Result<(), ValidateErr>;
    fn address(addr: &str) -> Result<(), ValidateErr>;
    fn validate(&self, p: &Person) -> Result<(), ValidateErr>;
}

impl Validate for Person {
    fn age(age: &i32) -> Result<(), ValidateErr> {
        match age > &0 {
            true => Ok(()),
            false => Err(ValidateErr::AgeErr),
        }
    }

    fn symbols(s: &str) -> Result<(), ValidateErr> {
        let a: Vec<char> = "<>,./\\`'~;:[]\"{}-_+|#@$%^&?!*()".chars().collect();
        match s.contains(&a[..]) {
            false => Ok(()),
            true => Err(ValidateErr::SymbolsErr),
        }
    }

    fn name(name: &str) -> Result<(), ValidateErr> {
        Self::symbols(name).map_err(|_| ValidateErr::NameErr)
    }

    fn work(work: &str) -> Result<(), ValidateErr> {
        Self::symbols(work).map_err(|_| ValidateErr::WorkErr)
    }

    fn address(addr: &str) -> Result<(), ValidateErr> {
        Self::symbols(addr).map_err(|_| ValidateErr::AddressErr)
    }

    fn validate(&self, p: &Person) -> Result<(), ValidateErr> {
        let _ = Self::age(&self.age)?;
        let _ = Self::name(&self.name)?;
        let _ = Self::work(&self.work)?;
        Self::address(&self.address)
    }

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
