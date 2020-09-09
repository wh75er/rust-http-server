#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod routes;
mod person;

use routes::*;
use dotenv::dotenv;
use rocket_contrib::databases::diesel;

#[database("pgdb")]
pub struct PersonsDatabase(diesel::PgConnection);

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes![show_unit, 
                            show_all,
                            add,
                            patch,
                            delete
                            ])
        .attach(PersonsDatabase::fairing())
        .launch();
}

