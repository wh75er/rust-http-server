use diesel::prelude::*;
use crate::schema::persons;
use crate::PersonsDatabase;
use crate::person::Person;
use std::result::Result;

pub struct MainDbOps;
//pub struct TestDbOps;

pub trait DbOps {
    fn insert(&self, p: &Person, conn: &PersonsDatabase) -> Result<Vec<Person>, diesel::result::Error>;
    fn load(&self, conn: &PersonsDatabase) ->  Result<Vec<Person>, diesel::result::Error>;
    fn load_id(&self, id: i32, conn: &PersonsDatabase) -> Result<Vec<Person>, diesel::result::Error>;
    fn update(&self, id: i32, p: &Person, conn: &PersonsDatabase) -> Result<Person, diesel::result::Error>;
    fn delete(&self, id: i32, conn: &PersonsDatabase) -> Result<usize, diesel::result::Error>;
}

impl DbOps for MainDbOps {
    fn insert(&self, p: &Person, conn: &PersonsDatabase) -> Result<Vec<Person>, diesel::result::Error> {
        diesel::insert_into(persons::table)
            .values((
                persons::name.eq(&p.name),
                persons::age.eq(&p.age),
                persons::address.eq(&p.address),
                persons::work.eq(&p.work)
                ))
            .get_results(&**conn)
    }
    
    fn load(&self, conn: &PersonsDatabase) ->  Result<Vec<Person>, diesel::result::Error> {
        persons::table.load::<Person>(&**conn)
    }

    fn load_id(&self, id: i32, conn: &PersonsDatabase) -> Result<Vec<Person>, diesel::result::Error> {
        persons::table.filter(persons::id.eq(id))
            .load::<Person>(&**conn)
    }

    fn update(&self, id: i32, p: &Person, conn: &PersonsDatabase) -> Result<Person, diesel::result::Error> {
        diesel::update(persons::table.find(id))
            .set(p)
            .get_result(&**conn)
    }

    fn delete(&self, id: i32, conn: &PersonsDatabase) -> Result<usize, diesel::result::Error> {
        diesel::delete(persons::table.find(id)).execute(&**conn)
    }
}
