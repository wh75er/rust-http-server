#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate r2d2;

mod routes;
mod person;
mod schema;
mod db;

#[cfg(test)] mod tests;

use routes::*;
use dotenv::dotenv;

#[database("pgdb")]
pub struct PersonsDatabase(diesel::PgConnection);

fn rocket<T>(db: T) -> rocket::Rocket 
    where T: rocket::fairing::Fairing
{
    rocket::ignite()
        .mount("/", routes![show_unit, 
                            show_all,
                            add,
                            patch,
                            delete
                            ])
        .attach(db)
}

fn main() {
    dotenv().ok();

    rocket(PersonsDatabase::fairing()).launch();
}

