#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::{Build, Rocket};
use rocket::serde::json::Json;

use self::models::*;
use self::schema::materials::dsl::*;

mod database;
mod models;
mod schema;

#[get("/")]
fn index() -> Json<Vec<Material>> {
    let connection = &mut database::establish_connection();
    materials.load::<Material>(connection).map(Json).expect("'Error loading materials")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

