use diesel::prelude::*;
use crate::schema::persons;
use crate::PersonsDatabase;
use crate::person::Person;
use std::result::Result;

pub struct MainDbOps;
pub struct TestDbOps;

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

impl DbOps for TestDbOps {
    fn insert(&self, p: &Person, conn: &PersonsDatabase) -> Result<Vec<Person>, diesel::result::Error> {
        Ok(vec!(p.clone()))
    }
    fn load(&self, conn: &PersonsDatabase) ->  Result<Vec<Person>, diesel::result::Error> {
        let p1 = Person{
            id: 1,
            age: 1,
            name: "1".to_string(),
            work: "1".to_string(),
            address: "1".to_string(),
        };
        let p2 = Person{
            id: 2,
            age: 2,
            name: "2".to_string(),
            work: "2".to_string(),
            address: "2".to_string(),
        };
        let p3 = Person{
            id: 3,
            age: 3,
            name: "3".to_string(),
            work: "3".to_string(),
            address: "3".to_string(),
        };
        let p4 = Person{
            id: 4,
            age: 4,
            name: "4".to_string(),
            work: "4".to_string(),
            address: "4".to_string(),
        };
        
        Ok(vec!(p1, p2, p3, p4))
    }

    fn load_id(&self, id: i32, conn: &PersonsDatabase) -> Result<Vec<Person>, diesel::result::Error> {
        let p = Person{
            id: id,
            age: id,
            name: id.to_string(),
            work: id.to_string(),
            address: id.to_string(),
        };

        Ok(vec!(p))
    }

    fn update(&self, id: i32, p: &Person, conn: &PersonsDatabase) -> Result<Person, diesel::result::Error> {
        let mut pcpy = p.clone();

        pcpy.id = id;

        Ok(pcpy)
    }

    fn delete(&self, id: i32, conn: &PersonsDatabase) -> Result<usize, diesel::result::Error> {
        Ok(1)
    }
}
