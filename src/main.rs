#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod person;
mod routes;
mod schema;

#[cfg(test)]
mod tests;

use rocket::fairing::AdHoc;
use rocket::Rocket;

use dotenv::dotenv;

use routes::*;

embed_migrations!();

#[database("pgdb")]
pub struct PersonsDatabase(diesel::PgConnection);

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = PersonsDatabase::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            println!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket<T>(db: T) -> rocket::Rocket
where
    T: rocket::fairing::Fairing,
{
    rocket::ignite()
        .mount("/", routes![show_unit, show_all, add, patch, delete])
        .attach(db)
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
}

fn main() {
    dotenv().ok();

    rocket(PersonsDatabase::fairing()).launch();
}
