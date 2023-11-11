#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod database;
mod models;
mod schema;
mod controller;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        controller::list_materials,
        controller::add_material,
        controller::list_mix_designs,
        controller::create_mix_design,
        controller::add_material_to_mix,
        controller::get_mix_materials,
        controller::get_pivot_data,
    ])
}

