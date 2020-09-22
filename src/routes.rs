use crate::db::MainDbOps;
use crate::person::*;
use crate::PersonsDatabase;

use serde::Serialize;

use rocket::http::hyper::header;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use diesel::result::Error;

use rocket_contrib::json::Json;

#[derive(Serialize, Debug)]
struct JsonError1 {
    message: String,
    errors: Vec<String>,
}

#[derive(Serialize, Debug)]
struct JsonError2 {
    message: String,
}

#[derive(Responder, Debug)]
enum JsonRespond {
    Item(Json<Person>),
    Items(Json<Vec<Person>>),
    Error1(Json<JsonError1>),
    Error2(Json<JsonError2>),
    Empty(()),
}

#[derive(Debug)]
pub struct ApiResponder {
    inner: JsonRespond,
    status: Status,
    location: Option<String>,
}

impl<'r> Responder<'r> for ApiResponder {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut build = Response::build_from(self.inner.respond_to(&req).unwrap());
        if let Some(location) = self.location {
            build.merge(
                Response::build()
                    .header(header::Location(location))
                    .finalize(),
            );
        }
        build.status(self.status).header(ContentType::JSON).ok()
    }
}

#[get("/persons/<id>")]
pub fn show_unit(id: i32, conn: PersonsDatabase) -> ApiResponder {
    match Person::read_id(id, &conn, MainDbOps) {
        Ok(v) => ApiResponder {
            inner: JsonRespond::Item(Json(v)),
            status: Status::Ok,
            location: None,
        },
        Err(e) => match e {
            Error::NotFound => ApiResponder {
                inner: JsonRespond::Error2(Json(JsonError2 {
                    message: e.to_string(),
                })),
                status: Status::NotFound,
                location: None,
            },
            _ => ApiResponder {
                inner: JsonRespond::Error1(Json(JsonError1 {
                    message: String::from("Error occured!"),
                    errors: vec![e.to_string()],
                })),
                status: Status::BadRequest,
                location: None,
            },
        },
    }
}

#[get("/persons")]
pub fn show_all(conn: PersonsDatabase) -> ApiResponder {
    match Person::read(&conn, MainDbOps) {
        Ok(v) => ApiResponder {
            inner: JsonRespond::Items(Json(v)),
            status: Status::Ok,
            location: None,
        },
        Err(e) => ApiResponder {
            inner: JsonRespond::Error1(Json(JsonError1 {
                message: String::from("Error occured!"),
                errors: vec![e.to_string()],
            })),
            status: Status::BadRequest,
            location: None,
        },
    }
}

#[post("/persons", data = "<p>")]
pub fn add(p: Json<Person>, conn: PersonsDatabase) -> impl Responder<'static> {
    let p = p.into_inner();

    match Person::create(&p, &conn, MainDbOps) {
        Ok(v) => ApiResponder {
            inner: JsonRespond::Empty(()),
            status: Status::Created,
            location: Some(
                "https://rsoi-person-service.herokuapp.com/person/".to_string() + &v.id.to_string(),
            ),
        },
        Err(e) => ApiResponder {
            inner: JsonRespond::Error1(Json(JsonError1 {
                message: String::from("Error occured!"),
                errors: vec![e.to_string()],
            })),
            status: Status::BadRequest,
            location: None,
        },
    }
}

#[patch("/persons/<id>", data = "<p>")]
pub fn patch(id: i32, p: Json<Person>, conn: PersonsDatabase) -> ApiResponder {
    match Person::update(id, &p, &conn, MainDbOps) {
        Ok(v) => ApiResponder {
            inner: JsonRespond::Item(Json(v)),
            status: Status::Ok,
            location: None,
        },
        Err(e) => match e {
            Error::NotFound => ApiResponder {
                inner: JsonRespond::Error2(Json(JsonError2 {
                    message: e.to_string(),
                })),
                status: Status::NotFound,
                location: None,
            },
            _ => ApiResponder {
                inner: JsonRespond::Error1(Json(JsonError1 {
                    message: String::from("Error occured!"),
                    errors: vec![e.to_string()],
                })),
                status: Status::BadRequest,
                location: None,
            },
        },
    }
}

#[delete("/persons/<id>")]
pub fn delete(id: i32, conn: PersonsDatabase) -> ApiResponder {
    match Person::delete(id, &conn, MainDbOps) {
        Ok(v) => match v {
            0 => ApiResponder {
                inner: JsonRespond::Error2(Json(JsonError2 {
                    message: String::from("NotFound"),
                })),
                status: Status::NotFound,
                location: None,
            },
            _ => ApiResponder {
                inner: JsonRespond::Empty(()),
                status: Status::Ok,
                location: None,
            },
        },
        Err(e) => ApiResponder {
            inner: JsonRespond::Error1(Json(JsonError1 {
                message: String::from("Error occured!"),
                errors: vec![e.to_string()],
            })),
            status: Status::BadRequest,
            location: None,
        },
    }
}
