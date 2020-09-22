use crate::db::TestDbOps;
use crate::person::*;
use crate::PersonsDatabase;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use mockall::predicate::*;
use mockall::*;

use diesel::r2d2::ConnectionManager;

fn establish_connection() -> diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
        .get()
        .expect("Failed to get PooledConnection")
}

#[test]
fn creation_test() {
    let pool = establish_connection();
    let conn = PersonsDatabase(pool);

    conn.test_transaction::<_, diesel::result::Error, _>(
        || -> Result<(), diesel::result::Error> {
            let p1 = Person {
                id: 0,
                age: 0,
                name: "John".to_string(),
                address: "foo".to_string(),
                work: "foo".to_string(),
            };
            let p2 = Person::create(&p1, &conn, TestDbOps)?;

            assert_eq!(p1, p2);

            Ok(())
        },
    );
}
