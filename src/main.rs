#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
mod person;

use routes::*;

fn main() {
    rocket::ignite().mount("/", routes![show_unit, 
                                        show_all,
                                        add,
                                        patch,
                                        delete
                                        ]).launch();
}

