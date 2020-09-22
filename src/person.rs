use crate::db::DbOps;
use crate::schema::persons;
use crate::PersonsDatabase;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
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

#[derive(Debug, PartialEq)]
pub enum ValidateErr {
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

pub trait Validator {
    fn age(age: &i32) -> Result<(), ValidateErr>;
    fn symbols(s: &str) -> Result<(), ValidateErr>;
    fn name(name: &str) -> Result<(), ValidateErr>;
    fn work(work: &str) -> Result<(), ValidateErr>;
    fn address(addr: &str) -> Result<(), ValidateErr>;
    fn validate(&self) -> Result<(), Vec<ValidateErr>>;
}

impl Validator for Person {
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
        if name.eq("") {
            return Err(ValidateErr::NameErr);
        }
        Self::symbols(name).map_err(|_| ValidateErr::NameErr)
    }

    fn work(work: &str) -> Result<(), ValidateErr> {
        if work.eq("") {
            return Err(ValidateErr::WorkErr);
        }
        Self::symbols(work).map_err(|_| ValidateErr::WorkErr)
    }

    fn address(addr: &str) -> Result<(), ValidateErr> {
        if addr.eq("") {
            return Err(ValidateErr::AddressErr);
        }
        Self::symbols(addr).map_err(|_| ValidateErr::AddressErr)
    }

    fn validate(&self) -> Result<(), Vec<ValidateErr>> {
        let mut v: Vec<ValidateErr> = vec!();

        let _ = Self::age(&self.age).map_err(|e| v.push(e));
        let _ = Self::name(&self.name).map_err(|e| v.push(e));
        let _ = Self::work(&self.work).map_err(|e| v.push(e));
        let _ = Self::address(&self.address).map_err(|e| v.push(e));

        match v.len() {
            0 => Ok(()),
            _ => Err(v),
        }
    }
}

impl Person {
    pub fn create(
        p: &Person,
        conn: &PersonsDatabase,
        dbops: impl DbOps,
    ) -> Result<Person, diesel::result::Error> {
        let mut vec = dbops.insert(p, conn)?;
        vec.pop().ok_or(diesel::result::Error::NotFound)
    }

    pub fn read(
        conn: &PersonsDatabase,
        dbops: impl DbOps,
    ) -> Result<Vec<Person>, diesel::result::Error> {
        dbops.load(conn)
    }

    pub fn read_id(
        id: i32,
        conn: &PersonsDatabase,
        dbops: impl DbOps,
    ) -> Result<Person, diesel::result::Error> {
        let mut vec = dbops.load_id(id, conn)?;
        vec.pop().ok_or(diesel::result::Error::NotFound)
    }

    pub fn update(
        id: i32,
        p: &Person,
        conn: &PersonsDatabase,
        dbops: impl DbOps,
    ) -> Result<Person, diesel::result::Error> {
        dbops.update(id, p, conn)
    }

    pub fn delete(
        id: i32,
        conn: &PersonsDatabase,
        dbops: impl DbOps,
    ) -> Result<usize, diesel::result::Error> {
        dbops.delete(id, conn)
    }
}
